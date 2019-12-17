use crate::cpu::CPU;
use std::path::PathBuf;

pub struct Device{
    cpu: CPU,

}

impl Device{
    pub fn new() -> Device{
        Device{
            cpu: CPU::new(),
        }
    }
    pub fn run(&mut self) -> () {
        loop{
            let cycles_elapsed = self.cpu.do_cycle();
            //Interruptions, video sound etc.
        }

    }
    pub fn open_rom(&mut self, rom_path: PathBuf){
        self.cpu.open_rom(rom_path);
    }
}