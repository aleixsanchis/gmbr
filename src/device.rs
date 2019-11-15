use crate::cpu::CPU;
pub struct Device<'a>{
    cpu: CPU<'a>,
}

impl Device<'_>{
    pub fn new() -> Device<'static>{
        Device{
            cpu: CPU::new(),
        }
    }
    pub fn run(self) -> () {
        self.cpu.run();
    }
}