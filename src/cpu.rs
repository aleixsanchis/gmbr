use crate::registers::Registers;
pub struct CPU {
    registers: Registers,
}

impl CPU{

    pub fn new() -> CPU{
        CPU{
            registers: Registers::new(),
        }
    }

    pub fn run(self){
        println!("CPU Running!\n Register a value is : {}\n", self.registers.a);
    }
}