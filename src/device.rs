use crate::cpu::CPU;
pub struct Device{
    cpu: CPU,
}

impl Device{
    pub fn new() -> Device{
        Device{
            cpu: CPU::new(),
        }
    }
    pub fn run(self) -> () {
        self.cpu.run();
    }
}