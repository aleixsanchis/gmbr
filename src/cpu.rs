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

    fn execute_instruction(&mut self, instruction: u8){
        match instruction {
            //NOP
            0x00 => (),
            _ => println!("Instruction {:2X} not implemented!", instruction),
        }
    }
    pub fn run(mut self){
        println!("CPU Running!\n Register a value is : {}\n", self.registers.a);
        loop{
            self.do_cycle();
            
        }
    }
}