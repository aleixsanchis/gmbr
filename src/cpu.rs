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

    pub fn run(mut self){
        println!("CPU Running!\n Register a value is : {}\n", self.registers.a);
        let mut i: u16 = 0;
        loop{
            self.registers.setaf(i);
            i+=1;
            println!("CPU Running!\n Register a value is : {}\n", self.registers.af());
        }
    }
}