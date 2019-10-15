use crate::registers::CpuFlags::{C, N, H, Z};
use crate::registers::Registers;
use crate::mmu::MMU;
pub struct CPU {
    registers: Registers,
    mmu: MMU,
}

impl CPU{

    pub fn new() -> CPU{
        CPU{
            registers: Registers::new(),
            mmu: MMU::new(),
        }
    }

    pub fn do_cycle(&mut self){
        let instruction : u8 = self.mmu.read_byte(self.registers.pc);
        self.registers.pc+=1;
        self.execute_instruction(instruction);
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
            0x01 => {self.registers.setbc(self.fetch_word()); 3},
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
            0x08 => {self.mmu.write_word(self.fetch_word(), self.registers.sp); 5},
            //ADD HL,BC
            0x09 => {self.registers.sethl(self.alu_add16(self.registers.hl(), self.registers.bc())); 2},
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
            0x0F => {self.registers.a = self.alu_rrca(self.registers.a); 4},

            _ => {println!("Instruction {:2X} not implemented!", instruction);std::process::exit(1);0},
        }
    }

    fn alu_inc(&mut self, value: u8) -> u8{
        let inc_value = value.wrapping_add(1);

        self.registers.set_flags(Z, inc_value == 0);
        self.registers.set_flags(H, is_half_carry_add8(value, 1));
        self.registers.set_flags(N, false);
        return inc_value;
    }

    fn alu_dec(&mut self, value: u8) -> u8{
        let dec_value = value.wrapping_sub(1);

        self.registers.set_flags(Z, dec_value == 0);
        self.registers.set_flags(H, is_half_carry_sub8(value, 1));
        self.registers.set_flags(N, true);
        return dec_value;
    }

    fn alu_rlca(&mut self, value: u8) -> u8{
        let r_value = value.rotate_left(1);

        self.registers.set_flags(Z, r_value == 0);
        self.registers.set_flags(N, false);
        self.registers.set_flags(H, false);
        self.registers.set_flags(C, value & 0x80 == 0x80);
        return r_value;
    }

    fn alu_rrca(&mut self, value: u8) -> u8{
        let r_value = value.rotate_right(1);

        self.registers.set_flags(Z, r_value == 0);
        self.registers.set_flags(N, false);
        self.registers.set_flags(H, false);
        self.registers.set_flags(C, value & 0x01 == 0x01);
        return r_value;
    }

    fn alu_add16(&mut self, a: u16, b: u16) -> u16{
        let sum = a.wrapping_add(b);

        self.registers.set_flags(N, false);
        self.registers.set_flags(H, is_half_carry_add16(a, b));
        self.registers.set_flags(C, is_carry_add16(a,b));
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