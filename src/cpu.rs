use crate::registers::CpuFlags;
use crate::registers::Registers;
use crate::mmu::MMU;
pub struct CPU<'a>{
    registers: Registers,
    mmu: MMU<'a>,
}

impl CPU<'_>{

    pub fn new() -> CPU<'static>{
        CPU{
            registers: Registers::new(),
            mmu: MMU::new(),
        }
    }

    pub fn do_cycle(&mut self){
        let instruction : u8 = self.mmu.read_byte(self.registers.pc);
        self.registers.pc+=1;
        let cycles = self.execute_instruction(instruction);
        if cycles == 0{
            std::process::exit(0);
        }
    }

    fn fetch_word(&mut self) -> u16{
        let word = self.mmu.read_word(self.registers.pc);
        self.registers.pc+=2;
        return word;
    }
    
    fn fetch_byte(&mut self) -> u8{
        let byte = self.mmu.read_byte(self.registers.pc);
        self.registers.pc+=1;
        return byte;
    }

    // Executes an instruction, returns the cycles spent
    fn execute_instruction(&mut self, instruction: u8) -> u8{
        match instruction {
            //NOP
            0x00 => {1},
            //LD BC, d16
            0x01 => {let value = self.fetch_word(); self.registers.setbc(value); 3},
            //LD (BC), A
            0x02 => {self.mmu.write_byte(self.registers.bc(), self.registers.a); 2},
            //INC BC
            0x03 => {self.registers.setbc(self.registers.bc().wrapping_add(1)); 2},
            //INC B
            0x04 => {self.registers.b = self.alu_inc(self.registers.b); 1},
            //DEC B
            0x05 => {self.registers.b = self.alu_dec(self.registers.b); 1},
            //LD B, d8
            0x06 => {self.registers.b = self.fetch_byte(); 2},
            //RLCA
            0x07 => {self.registers.a = self.alu_rlca(self.registers.a); 1},
            //LD (a16), SP
            0x08 => {let value = self.fetch_word(); self.mmu.write_word(value, self.registers.sp); 5},
            //ADD HL,BC
            0x09 => {let value = self.alu_add16(self.registers.hl(), self.registers.bc()); self.registers.sethl(value); 2},
            //LD A, (BC)
            0x0A => {self.registers.a = self.mmu.read_byte(self.registers.bc()); 2},
            //DEC BC
            0x0B => {self.registers.setbc(self.registers.bc().wrapping_sub(1)); 2},
            //INC C
            0x0C => {self.registers.c = self.alu_inc(self.registers.c); 1},
            //DEC C
            0x0D => {self.registers.c = self.alu_dec(self.registers.c); 1},
            //LD C, d8
            0x0E => {self.registers.c = self.fetch_byte(); 2},
            //RRCA
            0x0F => {self.registers.a = self.alu_rrca(self.registers.a); 1},
            //STOP TODO
            0x10 => {(); 1},
            //LD DE, d16
            0x11 => {let value = self.fetch_word(); self.registers.setde(value); 3},
            //LD (DE), A
            0x12 => {self.mmu.write_byte(self.registers.de(), self.registers.a); 2},
            //INC DE
            0x13 => {self.registers.setde(self.registers.de().wrapping_add(1)); 2},
            //INC D
            0x14 => {self.registers.d = self.alu_inc(self.registers.d); 1},
            //DEC D
            0x15 => {self.registers.d = self.alu_dec(self.registers.d); 1},
            //LD D, d8
            0x16 => {self.registers.d = self.fetch_byte(); 2},
            //RLA
            0x17 => {self.registers.a = self.alu_rla(self.registers.a); 1},
            //JR
            0x18 => {self.registers.pc = self.calculate_jr_address();3}
            //ADD HL, DE
            0x19 => {let value = self.alu_add16(self.registers.hl(), self.registers.de()); self.registers.sethl(value); 2},
            //LD A, (DE)
            0x1A => {self.registers.a = self.mmu.read_byte(self.registers.de()); 2},
            //DEC DE
            0x1B => {self.registers.setde(self.registers.de().wrapping_sub(1)); 2},
            //INC E
            0x1C => {self.registers.e = self.alu_inc(self.registers.e); 1},
            //DEC E
            0x1D => {self.registers.e = self.alu_dec(self.registers.e); 1},
            //LD E, d8
            0x1E => {self.registers.e = self.fetch_byte(); 2},
            //RRA
            0x1F => {self.registers.a = self.alu_rra(self.registers.a); 1},
            //JR NZ, r8
            0x20 => {let took_jump = self.jr_if_nflag(CpuFlags::Z); if took_jump {3} else {2}},
            //LD HL, d16
            0x21 => {let value = self.fetch_word(); self.registers.sethl(value); 3},
            //LD (HL+), A
            0x22 => {self.mmu.write_byte(self.registers.hl(), self.registers.a); self.registers.increment_hl();2},
            //INC HL
            0x23 => {self.registers.increment_hl(); 2},
            //INC H
            0x24 => {self.registers.h = self.alu_inc(self.registers.h); 1},
            //DEC H
            0x25 => {self.registers.h = self.alu_dec(self.registers.h); 1},
            //LD H, d8
            0x26 => {self.registers.h = self.fetch_byte(); 2},
            //DAA TODO
            0x27 => {(); 1},
            _ => {println!("Instruction {:2X} not implemented!", instruction);0},
        }
    }

    fn jr_if_nflag(&mut self, flag: CpuFlags) -> bool{
        if !self.registers.get_flag(flag){
            let address = self.calculate_jr_address();
            self.registers.pc = address;
            return true;
        }
        else{
            return false;
        }
    }

    fn calculate_jr_address(&mut self) -> u16{
        let mut pc = self.registers.pc as u32 as i32;

        pc += self.fetch_byte() as i32;
        return pc as u16;
    }

    fn alu_inc(&mut self, value: u8) -> u8{
        let inc_value = value.wrapping_add(1);

        self.registers.set_flags(CpuFlags::Z, inc_value == 0);
        self.registers.set_flags(CpuFlags::H, is_half_carry_add8(value, 1));
        self.registers.set_flags(CpuFlags::N, false);
        return inc_value;
    }

    fn alu_dec(&mut self, value: u8) -> u8{
        let dec_value = value.wrapping_sub(1);

        self.registers.set_flags(CpuFlags::Z, dec_value == 0);
        self.registers.set_flags(CpuFlags::H, is_half_carry_sub8(value, 1));
        self.registers.set_flags(CpuFlags::N, true);
        return dec_value;
    }

    fn alu_rlca(&mut self, value: u8) -> u8{
        let r_value = value.rotate_left(1);

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x80 == 0x80);
        return r_value;
    }

    fn alu_rla(&mut self, value: u8) -> u8{
        
        let mut r_value = value << 1;
        let carry_was_one : bool = self.registers.get_flag(CpuFlags::C);

        if carry_was_one {
            r_value |= 0x01;
        }
        else{
            r_value &= 0xFE;
        }

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x80 == 0x80);

        return r_value;
    }

    fn alu_rrca(&mut self, value: u8) -> u8{
        let r_value = value.rotate_right(1);

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x01 == 0x01);
        return r_value;
    }

    fn alu_rra(&mut self, value: u8) -> u8{
        let mut r_value = value >> 1;
        let carry_was_one : bool = self.registers.get_flag(CpuFlags::C);

        if carry_was_one {
            r_value |= 0x80;
        }

        else{
            r_value &= 0x7F;
        }

        self.registers.set_flags(CpuFlags::Z, r_value == 0);
        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, false);
        self.registers.set_flags(CpuFlags::C, value & 0x01 == 0x01);
        return r_value;
    }

    fn alu_add16(&mut self, a: u16, b: u16) -> u16{
        let sum = a.wrapping_add(b);

        self.registers.set_flags(CpuFlags::N, false);
        self.registers.set_flags(CpuFlags::H, is_half_carry_add16(a, b));
        self.registers.set_flags(CpuFlags::C, is_carry_add16(a,b));
        return sum;
    }

    pub fn run(mut self){
        println!("CPU Running!\n Register a value is : {}\n", self.registers.a);
        loop{
            self.do_cycle();
            
        }
    }
}

fn is_carry_add16(a: u16, b: u16) -> bool{
    return a > (0xFFFF - b);
}

fn is_half_carry_add16(a: u16, value: u16) -> bool{
    return ((a & 0x07FF) + (value & 0x07FF)) > 0x07F0;
}

fn is_half_carry_add8(a: u8, value: u8) -> bool{
    return (((a & 0x0F) + (value & 0x0F)) & 0x10) == 0x10;
}

fn is_half_carry_sub8(a: u8, value: u8) -> bool{
    return ((a & 0xF) - (value & 0xF)) < 0;
}