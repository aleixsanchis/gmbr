use crate::cpu::CPU;
use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
use std::path::PathBuf;
use std::time::{Duration,Instant};
use std::thread::sleep;
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
        let cycles_per_frame : u32 = 69905;
        loop{
            let now = Instant::now();
            let mut total_cycles : u32 = 0;
            while total_cycles < cycles_per_frame{
                let cycles_elapsed = self.cpu.do_cycle() * 4;
                total_cycles += cycles_elapsed as u32;
                self.cpu.gpu.update_scanlines(cycles_elapsed);
                if debug_mode {
                    self.cpu.print_registers();
                    cli::read_any_key();
                }
            }

            let time_spent = now.elapsed().as_millis();
            if time_spent < 16{
                let time_to_sleep = 16 - time_spent;
                eprintln!("Going to sleep for {} miliseconds", time_to_sleep);
                sleep(Duration::from_millis(time_to_sleep as u64));
            }
            else{
                eprintln!("Falling behind... last frame took {}", time_spent);
            }
        }

    }
    pub fn open_rom(&mut self, rom_path: PathBuf){
        self.cpu.open_rom(rom_path);
    }
}