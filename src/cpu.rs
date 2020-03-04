use crate::mmu::MMU;
use crate::registers::CpuFlags;
use crate::registers::Registers;
use serde_json::*;
extern crate hex;
use crate::apu::APU;
use crate::gpu::GPU;
use crate::interrupt_controller::InterruptController;
use crate::joypad::*;
use crate::link_cable::LinkCable;
use crate::memory_map::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct CPU {
    pub registers: Registers,
    pub mmu: MMU,
    pub interrupt_controller: InterruptController,
    pub gpu: GPU,
    link_cable: LinkCable,
    pub joypad: Joypad,
    pub apu: APU,
    max_pc: u16,
    pub cb_prefix: bool,
}
pub enum MBCType {
    MBC0,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            mmu: MMU::new(),
            interrupt_controller: InterruptController::new(),
            gpu: GPU::new(),
            link_cable: LinkCable::new(),
            joypad: Joypad::new(),
            apu: APU::new(),
            max_pc: 0,
            cb_prefix: false,
        }
    }

    pub fn swap_mbc(mbc_type: MBCType) {
        match mbc_type {
            MBCType::MBC0 => {
                println!("Not swapping the MBC");
            }
        }
    }

    pub fn open_rom(&mut self, rom_path: PathBuf) {
        self.mmu.open_rom(rom_path);
    }

    pub fn do_cycle(&mut self) -> u8 {
        // if self.registers.pc > self.max_pc {
        //     self.max_pc = self.registers.pc;
        //     self.print_registers();
        // }
        let instruction: u8 = self.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let cycles = self.execute_instruction(instruction);
        // if cycles == 0 {
        //     panic!("should never gotten here...");
        // }
        return cycles;
    }

    pub fn print_registers(&self) {
        println!("{:#4X?}", self.registers);
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.read_word(self.registers.pc);
        self.registers.pc += 2;
        return word;
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.registers.pc);
        self.registers.pc += 1;
        return byte;
    }

    // Executes an instruction, returns the cycles spent
    fn execute_instruction(&mut self, instruction: u8) -> u8 {
        if self.cb_prefix {
            self.cb_prefix = false;
            match instruction {
                // RLC Reg
                0x00..=0x07 => self.rlc_opcode(instruction),
                // RRC Reg
                0x08..=0x0F => self.rrc_opcode(instruction),
                // Swap Reg
                0x30..=0x37 => self.swap_opcode(instruction),
                0x80..=0xBF => {
                    let bit = parse_destination_register(instruction, 0x08);
                    self.res_opcode(instruction, bit)
                }
                _ => {
                    //self.print_registers();
                    handle_unimplemented_instruction(instruction, true);
                    0
                }
            }
        } else {
            match instruction {
                // NOP
                0x00 => 1,
                // LD BC, d16
                0x01 => {
                    let value = self.fetch_word();
                    self.registers.setbc(value);
                    3
                }
                // LD (BC), A
                0x02 => {
                    self.write_byte(self.registers.bc(), self.registers.a);
                    2
                }
                // INC BC
                0x03 => {
                    self.registers.setbc(self.registers.bc().wrapping_add(1));
                    2
                }
                // INC B
                0x04 => {
                    self.registers.b = self.alu_inc(self.registers.b);
                    1
                }
                // DEC B
                0x05 => {
                    self.registers.b = self.alu_dec(self.registers.b);
                    1
                }
                // LD B, d8
                0x06 => {
                    self.registers.b = self.fetch_byte();
                    2
                }
                // RLCA
                0x07 => {
                    self.registers.a = self.alu_rlca(self.registers.a);
                    1
                }
                // LD (a16), SP
                0x08 => {
                    let value = self.fetch_word();
                    self.write_word(value, self.registers.sp);
                    5
                }
                // ADD HL,BC
                0x09 => {
                    let value = self.alu_add16(self.registers.hl(), self.registers.bc());
                    self.registers.sethl(value);
                    2
                }
                // LD A, (BC)
                0x0A => {
                    self.registers.a = self.read_byte(self.registers.bc());
                    2
                }
                // DEC BC
                0x0B => {
                    self.registers.setbc(self.registers.bc().wrapping_sub(1));
                    2
                }
                // INC C
                0x0C => {
                    self.registers.c = self.alu_inc(self.registers.c);
                    1
                }
                // DEC C
                0x0D => {
                    self.registers.c = self.alu_dec(self.registers.c);
                    1
                }
                // LD C, d8
                0x0E => {
                    self.registers.c = self.fetch_byte();
                    2
                }
                // RRCA
                0x0F => {
                    self.registers.a = self.alu_rrca(self.registers.a);
                    1
                }
                // STOP TODO
                0x10 => {
                    (handle_unimplemented_instruction(instruction, false));
                    1
                }
                // LD DE, d16
                0x11 => {
                    let value = self.fetch_word();
                    self.registers.setde(value);
                    3
                }
                // LD (DE), A
                0x12 => {
                    self.write_byte(self.registers.de(), self.registers.a);
                    2
                }
                // INC DE
                0x13 => {
                    self.registers.setde(self.registers.de().wrapping_add(1));
                    2
                }
                // INC D
                0x14 => {
                    self.registers.d = self.alu_inc(self.registers.d);
                    1
                }
                // DEC D
                0x15 => {
                    self.registers.d = self.alu_dec(self.registers.d);
                    1
                }
                // LD D, d8
                0x16 => {
                    self.registers.d = self.fetch_byte();
                    2
                }
                // RLA
                0x17 => {
                    self.registers.a = self.alu_rla(self.registers.a);
                    1
                }
                // JR
                0x18 => {
                    self.registers.pc = self.calculate_jr_address();
                    3
                }
                // ADD HL, DE
                0x19 => {
                    let value = self.alu_add16(self.registers.hl(), self.registers.de());
                    self.registers.sethl(value);
                    2
                }
                // LD A, (DE)
                0x1A => {
                    self.registers.a = self.read_byte(self.registers.de());
                    2
                }
                // DEC DE
                0x1B => {
                    self.registers.setde(self.registers.de().wrapping_sub(1));
                    2
                }
                // INC E
                0x1C => {
                    self.registers.e = self.alu_inc(self.registers.e);
                    1
                }
                // DEC E
                0x1D => {
                    self.registers.e = self.alu_dec(self.registers.e);
                    1
                }
                // LD E, d8
                0x1E => {
                    self.registers.e = self.fetch_byte();
                    2
                }
                // RRA
                0x1F => {
                    self.registers.a = self.alu_rra(self.registers.a);
                    1
                }
                // JR NZ, r8
                0x20 => {
                    let took_jump = self.jr_if_nflag(CpuFlags::Z);
                    took_jump
                }
                // LD HL, d16
                0x21 => {
                    let value = self.fetch_word();
                    self.registers.sethl(value);
                    3
                }
                // LD (HL+), A
                0x22 => {
                    let hl = self.registers.hl_and_inc();
                    self.write_byte(hl, self.registers.a);
                    2
                }
                // INC HL
                0x23 => {
                    self.registers.increment_hl();
                    2
                }
                // INC H
                0x24 => {
                    self.registers.h = self.alu_inc(self.registers.h);
                    1
                }
                // DEC H
                0x25 => {
                    self.registers.h = self.alu_dec(self.registers.h);
                    1
                }
                // LD H, d8
                0x26 => {
                    self.registers.h = self.fetch_byte();
                    2
                }
                // DAA TODO
                0x27 => {
                    (handle_unimplemented_instruction(instruction, false));
                    1
                }
                // JR Z, r8
                0x28 => {
                    let took_jump = self.jr_if_flag(CpuFlags::Z);
                    took_jump
                }
                // ADD HL, HL
                0x29 => {
                    let value = self.alu_add16(self.registers.hl(), self.registers.hl());
                    self.registers.sethl(value);
                    2
                }
                // LD A, (HL+)
                0x2A => {
                    let hl = self.registers.hl_and_inc();
                    self.registers.a = self.read_byte(hl);
                    2
                }
                // DEC HL
                0x2B => {
                    self.registers.decrement_hl();
                    2
                }
                // INC L
                0x2C => {
                    self.registers.l = self.alu_inc(self.registers.l);
                    1
                }
                // DEC L
                0x2D => {
                    self.registers.l = self.alu_dec(self.registers.l);
                    1
                }
                // LD L, d8
                0x2E => {
                    self.registers.l = self.fetch_byte();
                    2
                }
                // CPL
                0x2F => {
                    self.alu_cpl();
                    1
                }
                // JR NC, r8
                0x30 => {
                    let took_jump = self.jr_if_nflag(CpuFlags::C);
                    took_jump
                }
                // LD SP, d16
                0x31 => {
                    let value = self.fetch_word();
                    self.registers.sp = value;
                    3
                }
                // LD (HL-), A
                0x32 => {
                    let hl = self.registers.hl_and_dec();
                    self.write_byte(hl, self.registers.a);
                    2
                }
                // INC SP
                0x33 => {
                    self.registers.sp = self.registers.sp.wrapping_add(1);
                    2
                }
                // INC (HL)
                0x34 => {
                    let mut value = self.read_byte(self.registers.hl());
                    value = self.alu_inc(value);
                    self.write_byte(self.registers.hl(), value);
                    3
                }
                // DEC (HL)
                0x35 => {
                    let mut value = self.read_byte(self.registers.hl());
                    value = self.alu_dec(value);
                    self.write_byte(self.registers.hl(), value);
                    3
                }
                // LD (HL), d8
                0x36 => {
                    let immediate = self.fetch_byte();
                    self.write_byte(self.registers.hl(), immediate);
                    3
                }
                // SCF TODO
                0x37 => handle_unimplemented_instruction(instruction, false),
                // JR C, r8
                0x38 => {
                    let took_jump = self.jr_if_flag(CpuFlags::C);
                    took_jump
                }
                // ADD HL, SP
                0x39 => {
                    let value = self.alu_add16(self.registers.hl(), self.registers.sp);
                    self.registers.sethl(value);
                    2
                }
                // LD A, (HL-)
                0x3A => {
                    let hl = self.registers.hl_and_dec();
                    self.registers.a = self.read_byte(hl);
                    2
                }
                // DEC SP
                0x3B => {
                    self.registers.sp = self.registers.sp.wrapping_sub(1);
                    2
                }
                // INC A
                0x3C => {
                    self.registers.a = self.alu_inc(self.registers.a);
                    1
                }
                // DEC A
                0x3D => {
                    self.registers.a = self.alu_dec(self.registers.a);
                    1
                }
                //LD A, d8
                0x3E => {
                    self.registers.a = self.fetch_byte();
                    2
                }
                // CCF
                0x3F => {
                    self.alu_ccf();
                    1
                }
                // Register Movements
                0x40..=0x75 => self.register_movement(instruction),
                // HALT TODO
                0x76 => handle_unimplemented_instruction(instruction, false),
                // More Register Movements
                0x77..=0x7F => self.register_movement(instruction),
                // ADD A, Reg
                0x80..=0x87 => self.alu_add8(instruction),
                // ADC A, Reg
                0x88..=0x8F => self.alu_adc8(instruction),
                // SUB Reg
                0x90..=0x97 => self.sub8_opcode(instruction),
                // SBC Reg
                0x98..=0x9F => self.alu_sbc8(instruction),
                // AND Reg
                0xA0..=0xA7 => self.and_opcode(instruction),
                // XOR Reg
                0xA8..=0xAF => self.xor_opcode(instruction),
                // OR Reg
                0xB0..=0xB7 => self.or_opcode(instruction),
                // RET N>
                0xC0 => {
                    if !self.registers.get_flag(CpuFlags::Z) {
                        self.ret(); 
                        5
                    } else {
                        2
                    }
                },
                // POP BC
                0xC1 => {
                    let value = self.pop_from_stack();
                    self.registers.setbc(value);
                    3
                }
                // JP a16
                0xC3 => {
                    let address = self.fetch_word();
                    self.jump_to(address);
                    4
                }
                // PUSH BC
                0xC5 => {
                    let value = self.registers.bc();
                    self.push_to_stack(value);
                    4
                }
                // RET Z
                0xC8 => self.ret_if_flag(CpuFlags::Z),
                // RET
                0xC9 => {
                    self.ret();
                    4
                }
                // JP Z, a16
                0xCA => {
                    let address = self.fetch_word();
                    self.jp_if_flag(address, CpuFlags::Z)
                }
                // CB
                0xCB => {
                    self.cb_prefix = true;
                    1
                }
                // CALL a16
                0xCD => {
                    let value = self.fetch_word();
                    self.call(value);
                    6
                }
                // POP DE
                0xD1 => {
                    let value = self.pop_from_stack();
                    self.registers.setde(value);
                    3
                }
                // PUSH DE
                0xD5 => {
                    let value = self.registers.de();
                    self.push_to_stack(value);
                    4
                }
                // RETI
                0xD9 => {
                    self.ret();
                    self.interrupt_controller.enable_master_interrupt();
                    4
                }
                // LDH (a8), A
                0xE0 => {
                    let address = self.fetch_byte();
                    self.write_byte(0xFF00 + address as u16, self.registers.a);
                    3
                }
                // POP HL
                0xE1 => {
                    let value = self.pop_from_stack();
                    self.registers.sethl(value);
                    3
                }
                // LD (C), A
                0xE2 => {
                    let address = self.registers.c;
                    self.write_byte(0xFF00 + address as u16, self.registers.a);
                    2
                }
                // PUSH HL
                0xE5 => {
                    let value = self.registers.hl();
                    self.push_to_stack(value);
                    4
                }
                // AND d8
                0xE6 => {
                    let value = self.fetch_byte();
                    self.alu_and(value);
                    2
                }
                // JP (HL)
                0xE9 => {
                    let address = self.registers.hl();
                    self.jump_to(address);
                    1
                }
                // LD (a16), A
                0xEA => {
                    let address = self.fetch_word();
                    self.write_byte(address, self.registers.a);
                    4
                }
                // RST 0x28
                0xEF => {
                    self.rst(0x28);
                    4
                }
                // LDH A, (a8)
                0xF0 => {
                    let address = self.fetch_byte();
                    self.registers.a = self.read_byte(0xFF00 + address as u16);
                    3
                }
                // POP AF
                0xF1 => {
                    let value = self.pop_from_stack();
                    self.registers.setaf(value);
                    3
                }
                // DI
                0xF3 => {
                    self.interrupt_controller.disable_master_interrupt();
                    1
                }
                // PUSH AF
                0xF5 => {
                    let value = self.registers.af();
                    self.push_to_stack(value);
                    4
                }
                // LD A, (a16)
                0xFA => {
                    let address = self.fetch_word();
                    let value = self.read_byte(address);
                    self.registers.a = value;
                    4
                }
                // EI
                0xFB => {
                    self.interrupt_controller.enable_master_interrupt();
                    1
                }
                // CP, d8
                0xFE => {
                    let value = self.fetch_byte();
                    self.alu_cp(value);
                    2
                }
                _ => {
                    self.print_registers();
                    handle_unimplemented_instruction(instruction, false);
                    0
                } /*panic!("Instruction 0x{:2X} not implemented!\n
                  {:#4X?}", instruction, self.registers);},*/
            }
        }
    }

    fn rst(&mut self, offset: u8) {
        self.push_to_stack(self.registers.pc);
        self.jump_to(offset as u16);
    }

    fn call(&mut self, address: u16) {
        self.push_to_stack(self.registers.pc);
        self.jump_to(address);
    }

    fn ret_if_flag(&mut self, flag: CpuFlags) -> u8 {
        if self.registers.get_flag(flag) {
            self.ret();
            return 5;
        } else {
            return 2;
        }
    }

    fn ret(&mut self) {
        let address = self.pop_from_stack();
        self.jump_to(address);
    }

    pub fn push_to_stack(&mut self, value: u16) {
        let new_sp = self.registers.sp - 2;
        self.write_word(new_sp, value);
        self.registers.sp = new_sp;
    }

    fn pop_from_stack(&mut self) -> u16 {
        let value = self.read_word(self.registers.sp);
        self.registers.sp = self.registers.sp + 2;
        return value;
    }

    fn jr_if_nflag(&mut self, flag: CpuFlags) -> u8 {
        let address = self.calculate_jr_address();
        if !self.registers.get_flag(flag) {
            self.jump_to(address);
            return 3;
        } else {
            return 2;
        }
    }

    fn jr_if_flag(&mut self, flag: CpuFlags) -> u8 {
        let address = self.calculate_jr_address();
        if self.registers.get_flag(flag) {
            self.jump_to(address);
            return 3;
        } else {
            return 2;
        }
    }

    fn alu_and(&mut self, operand: u8) {
        self.registers.a = self.registers.a & operand;

        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, true);
        self.registers.set_flags(CpuFlags::C, false);
    }

    fn alu_xor(&mut self, operand: u8) {
        self.registers.a = self.registers.a ^ operand;

        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, false);
    }

    fn alu_or(&mut self, operand: u8) {
        self.registers.a = self.registers.a | operand;

        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, false);
    }

    fn alu_sub8(&mut self, operand: u8) {
        self.registers.a = self.registers.a.wrapping_sub(operand);

        self.registers.set_flags(CpuFlags::N, true);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_sub8(self.registers.a, operand));
        self.registers
            .set_flags(CpuFlags::C, is_carry_sub8(self.registers.a, operand));
        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);
    }

    fn res_opcode(&mut self, opcode: u8, bit: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let mut return_value = operand_cycles.1;

        let value = operand & (!(1 << bit));
        return_value = return_value + self.write_to_register(opcode, value);
        return return_value;
    }

    fn swap_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let mut return_value = operand_cycles.1;

        let swap_value = ((operand & 0xf0) >> 4) | ((operand & 0x0f) << 4);
        return_value = return_value + self.write_to_register(opcode, swap_value);

        return return_value;
    }

    fn rlc_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let mut return_value = operand_cycles.1;

        let r_value = operand.rotate_left(1);

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers
            .set_flags(CpuFlags::C, operand & 0x80 == 0x80);

        return_value = return_value + self.write_to_register(opcode, r_value);

        return return_value;
    }

    fn rrc_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let mut return_value = operand_cycles.1;

        let r_value = operand.rotate_right(1);

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers
            .set_flags(CpuFlags::C, operand & 0x01 == 0x01);

        return_value = return_value + self.write_to_register(opcode, r_value);

        return return_value;
    }

    fn xor_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;

        self.alu_xor(operand);

        return return_value;
    }

    fn and_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;

        self.alu_and(operand);

        return return_value;
    }
    fn or_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;

        self.alu_or(operand);

        return return_value;
    }

    fn sub8_opcode(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;

        self.alu_sub8(operand);

        return return_value;
    }

    fn alu_sbc8(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;
        let c: u8 = if self.registers.get_flag(CpuFlags::C) {
            1
        } else {
            0
        };

        self.registers.a = self.registers.a.wrapping_sub(operand.wrapping_add(c));

        self.registers.set_flags(CpuFlags::N, true);
        self.registers.set_flags(
            CpuFlags::H,
            is_half_carry_sub16(
                self.registers.a as u16,
                (operand as u16).wrapping_add(c as u16),
            ),
        );
        self.registers.set_flags(
            CpuFlags::C,
            is_carry_sub16(
                self.registers.a as u16,
                (operand as u16).wrapping_add(c as u16),
            ),
        );
        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);

        return return_value;
    }

    fn alu_adc8(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;
        let c: u8 = if self.registers.get_flag(CpuFlags::C) {
            1
        } else {
            0
        };

        self.registers.a = self.registers.a.wrapping_add(operand.wrapping_add(c));
        self.registers.set_flags(CpuFlags::N, false);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_add8(self.registers.a, operand));
        self.registers.set_flags(
            CpuFlags::C,
            is_carry_add16(self.registers.a as u16, (operand as u16) + (c as u16)),
        );
        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);

        return return_value;
    }

    fn alu_add8(&mut self, opcode: u8) -> u8 {
        let operand_cycles = self.get_operand_and_cycles(opcode);
        let operand = operand_cycles.0;
        let return_value = operand_cycles.1;

        self.registers.a = self.registers.a.wrapping_add(operand);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_add8(self.registers.a, operand));
        self.registers
            .set_flags(CpuFlags::C, is_carry_add8(self.registers.a, operand));
        self.registers.set_flags(CpuFlags::Z, self.registers.a == 0);

        return return_value;
    }

    fn alu_add16(&mut self, a: u16, b: u16) -> u16 {
        let sum = a.wrapping_add(b);

        self.registers.set_flags(CpuFlags::N, false);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_add16(a, b));
        self.registers.set_flags(CpuFlags::C, is_carry_add16(a, b));
        return sum;
    }

    fn get_operand_and_cycles(&mut self, opcode: u8) -> (u8, u8) {
        let source_register = parse_source_register_index(opcode);
        let mut return_value = 1;
        let operand;
        if source_register == 0x06 {
            operand = self.get_byte_at_hl();
            return_value += 1;
        } else {
            operand = self.registers.get_register_by_index(source_register);
        }

        return (operand, return_value);
    }

    fn write_to_register(&mut self, opcode: u8, value: u8) -> u8 {
        let destination_register = parse_source_register_index(opcode);
        if destination_register == 0x06 {
            self.write_byte(self.registers.hl(), value);
            return 1;
        } else {
            self.registers
                .set_register_by_index(destination_register, value);
            return 0;
        }
    }

    fn register_movement(&mut self, opcode: u8) -> u8 {
        let destination_register = parse_destination_register(opcode, 0x04);
        let source_register = parse_source_register_index(opcode);

        if destination_register == source_register {
            return 1;
        }

        // Case when reading from (HL)
        if source_register == 0x06 {
            self.registers
                .set_register_by_index(destination_register, self.read_byte(self.registers.hl()));
            return 2;
        }
        // Case when writing to (HL)
        else if destination_register == 0x06 {
            self.write_byte(
                self.registers.hl(),
                self.registers.get_register_by_index(source_register),
            );
            return 2;
        } else {
            self.registers.set_register_by_index(
                destination_register,
                self.registers.get_register_by_index(source_register),
            );
        }
        return 1;
    }

    fn alu_ccf(&mut self) {
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);

        self.registers
            .set_flags(CpuFlags::C, !self.registers.get_flag(CpuFlags::C));
    }

    fn alu_cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.set_flags(CpuFlags::N, true);
        self.registers.set_flags(CpuFlags::H, true);
    }

    fn jp_if_flag(&mut self, address: u16, flag: CpuFlags) -> u8 {
        if self.registers.get_flag(flag) {
            self.jump_to(address);
            return 4;
        } else {
            return 3;
        }
    }

    fn calculate_jr_address(&mut self) -> u16 {
        let offset: i32 = self.fetch_byte() as i8 as i32;
        let mut pc = self.registers.pc as u32 as i32;
        pc = pc + offset;
        return pc as u16;
    }

    fn jump_to(&mut self, address: u16) {
        //println!("Jumping to {:#4X?}", address);
        self.registers.pc = address;
    }

    fn alu_inc(&mut self, value: u8) -> u8 {
        let inc_value = value.wrapping_add(1);

        self.registers.set_flags(CpuFlags::Z, inc_value == 0);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_add8(value, 1));
        self.registers.set_flags(CpuFlags::N, false);
        return inc_value;
    }

    fn alu_dec(&mut self, value: u8) -> u8 {
        let dec_value = value.wrapping_sub(1);

        self.registers.set_flags(CpuFlags::Z, dec_value == 0);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_sub8(value, 1));
        self.registers.set_flags(CpuFlags::N, true);
        return dec_value;
    }

    fn alu_rlca(&mut self, value: u8) -> u8 {
        let r_value = value.rotate_left(1);

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x80 == 0x80);
        return r_value;
    }

    fn alu_rla(&mut self, value: u8) -> u8 {
        let mut r_value = value << 1;
        let carry_was_one: bool = self.registers.get_flag(CpuFlags::C);

        if carry_was_one {
            r_value |= 0x01;
        } else {
            r_value &= 0xFE;
        }

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x80 == 0x80);

        return r_value;
    }

    fn alu_rrca(&mut self, value: u8) -> u8 {
        let r_value = value.rotate_right(1);

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x01 == 0x01);
        return r_value;
    }

    fn alu_rra(&mut self, value: u8) -> u8 {
        let mut r_value = value >> 1;
        let carry_was_one: bool = self.registers.get_flag(CpuFlags::C);

        if carry_was_one {
            r_value |= 0x80;
        } else {
            r_value &= 0x7F;
        }

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x01 == 0x01);
        return r_value;
    }

    fn alu_cp(&mut self, value: u8) {
        self.alu_sub(value);
    }

    fn alu_sub(&mut self, value: u8) -> u8 {
        let result = self.registers.a.wrapping_sub(value);

        self.registers.set_flags(CpuFlags::Z, result == 0);
        self.registers.set_flags(CpuFlags::N, true);
        self.registers
            .set_flags(CpuFlags::H, is_half_carry_sub8(self.registers.a, value));
        self.registers
            .set_flags(CpuFlags::C, self.registers.a < value);
        return result;
    }

    fn get_word_at_hl(&mut self) -> u16 {
        return self.read_word(self.registers.hl());
    }
    fn get_byte_at_hl(&mut self) -> u8 {
        return self.read_byte(self.registers.hl());
    }
    fn set_word_at_hl(&mut self, value: u16) {
        return self.write_word(self.registers.hl(), value);
    }
    fn set_byte_at_hl(&mut self, value: u8) {
        return self.write_byte(self.registers.hl(), value);
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address as usize {
            VRAM_START..=VRAM_END => self
                .gpu
                .write_byte_vram(address as usize - VRAM_START, value),
            OAM_START..=OAM_END => self.gpu.write_byte_oam(address as usize - OAM_START, value),

            UNUSED_AREA_START..=UNUSED_AREA_END => {} // Do nothing

            JOYP => self.joypad.set_joyp(value),

            NR10..=WPR_START => self.apu.do_nothing(),
            LCDC => self.gpu.set_lcdc(value),
            STAT => self.gpu.set_stat(value),
            SCY => self.gpu.set_scy(value),
            SCX => self.gpu.set_scx(value),
            LYC => self.gpu.set_lyc(value),
            WY => self.gpu.set_wy(value),
            WX => self.gpu.set_wx(value),
            DMA => self.mmu.start_dma(value),
            BGP => self.gpu.set_bgp(value),
            OBP0 => self.gpu.set_obp0(value),
            OBP1 => self.gpu.set_obp1(value),
            SB => self.link_cable.set_sb(value),
            SC => self.link_cable.set_sc(value),

            IF => self.interrupt_controller.set_iflag(value),
            IE => self.interrupt_controller.set_ie(value),
            _ => self.mmu.write_byte(address, value),
        }
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address as usize {
            VRAM_START..=VRAM_END => return self.gpu.read_byte_vram(address as usize - VRAM_START),
            OAM_START..=OAM_END => return self.gpu.read_byte_oam(address as usize - OAM_START),
            UNUSED_AREA_START..=UNUSED_AREA_END => return 0xFF, // Default bus read
            JOYP => self.joypad.joyp(),
            LCDC => self.gpu.lcdc(),
            STAT => self.gpu.stat(),
            LY => self.gpu.ly(),
            IE => self.interrupt_controller.ie(),
            _ => return self.mmu.read_byte(address),
        }
    }

    fn read_word(&self, address: u16) -> u16 {
        return (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8);
    }
}

fn is_carry_sub8(a: u8, b: u8) -> bool {
    return a < b;
}

fn is_carry_sub16(a: u16, b: u16) -> bool {
    return a < b;
}

fn is_carry_add8(a: u8, b: u8) -> bool {
    return a > (0xFF - b);
}

fn is_carry_add16(a: u16, b: u16) -> bool {
    return a > (0xFFFF - b);
}

fn is_half_carry_add8(a: u8, value: u8) -> bool {
    return (((a & 0x0F) + (value & 0x0F)) & 0x10) == 0x10;
}

fn is_half_carry_add16(a: u16, value: u16) -> bool {
    return ((a & 0x07FF) + (value & 0x07FF)) > 0x07F0;
}

fn is_half_carry_sub8(a: u8, value: u8) -> bool {
    return ((a & 0xF) as i8 - (value & 0xF) as i8) < 0;
}

fn is_half_carry_sub16(a: u16, value: u16) -> bool {
    return ((a & 0x001F) as i16 - (value & 0x001F) as i16) < 0;
}

fn parse_destination_register(opcode: u8, offset: u8) -> u8 {
    return ((((opcode & 0xF0) >> 4) - offset) * 2) + ((opcode & 0x08) >> 3);
}

fn parse_source_register_index(opcode: u8) -> u8 {
    return opcode & 0x07;
}

fn handle_unimplemented_instruction(opcode: u8, prefixed: bool) -> u8 {
    let file = File::open("resources/opcodes.json").unwrap();
    let opcodes: serde_json::Value = serde_json::from_reader(file).unwrap();
    let final_opcodes;

    if prefixed {
        final_opcodes = opcodes.get("cbprefixed").unwrap();
    } else {
        final_opcodes = opcodes.get("unprefixed").unwrap();
    }
    let mut encoded_opcode = hex::encode(vec![opcode]);
    if encoded_opcode.starts_with("0") {
        encoded_opcode.remove(0);
    }
    let formatted_opcode = format!("0x{}", encoded_opcode);
    let missing_opcode = final_opcodes.get(formatted_opcode).unwrap();
    panic!(
        "Opcode Not implemented: {}, which corresponds to {} {}, {}",
        missing_opcode.get("addr").unwrap(),
        missing_opcode.get("mnemonic").unwrap(),
        missing_opcode.get("operand1").unwrap_or(&json!({"":""})),
        missing_opcode.get("operand2").unwrap_or(&json!({"":""}))
    );
    return 0;
}
