use crate::cpu::CPU;
use std::path::PathBuf;
use crate::cli;
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
        let debug_mode : bool = false;
        loop{
            let cycles_elapsed = self.cpu.do_cycle();
            if debug_mode {
                self.cpu.print_debug_info();
                cli::read_any_key();
            }
            //Interruptions, video sound etc.
        }

    }
    pub fn open_rom(&mut self, rom_path: PathBuf){
        self.cpu.open_rom(rom_path);
    }
}