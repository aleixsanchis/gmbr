use crate::cpu::CPU;
use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
use std::path::PathBuf;
use crate::cli;
pub struct Device{
    cpu: CPU,
    surface: GlfwSurface,
}



impl Device{
    pub fn new() -> Device{
        Device{
            cpu: CPU::new(),
            surface : GlfwSurface::new(WindowDim::Windowed(640, 576), "GMBR Emulator", WindowOpt::default()).unwrap(),
        }
    }
    pub fn run(&mut self) -> () {
        let debug_mode : bool = false;
        loop{
            let cycles_elapsed = self.cpu.do_cycle();

            if debug_mode {
                self.cpu.print_registers();
                cli::read_any_key();
            }
        }

    }
    pub fn open_rom(&mut self, rom_path: PathBuf){
        self.cpu.open_rom(rom_path);
    }
}