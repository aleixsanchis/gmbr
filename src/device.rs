<<<<<<< HEAD
extern crate sdl2;

use crate::cpu::CPU;

use std::path::PathBuf;
use std::time::{Duration,Instant};
use std::thread::sleep;
use crate::cli;
use crate::interrupt_controller::InterruptFlags;
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
        let _sdl = sdl2::init().unwrap();
        let debug_mode : bool = false;
        let cycles_per_frame : u32 = 69905;
        loop{
            let now = Instant::now();
            let mut total_cycles : u32 = 0;
            while total_cycles < cycles_per_frame{
                let cycles_elapsed = self.cpu.do_cycle() * 4;
                total_cycles += cycles_elapsed as u32;
                self.cpu.gpu.update_scanlines(cycles_elapsed);

                if self.cpu.gpu.stat_interrupt_req{
                    self.cpu.interrupt_controller.set_interrupt_flag(InterruptFlags::LCDStat);
                    self.cpu.gpu.stat_interrupt_req = false;
                }

                if self.cpu.gpu.vblank_interrupt_req{
                    self.cpu.interrupt_controller.set_interrupt_flag(InterruptFlags::VBlank);
                    self.cpu.gpu.vblank_interrupt_req = false;
                }

                if debug_mode {
                    self.cpu.print_registers();
                    //cli::read_any_key();
                }

                // User input
            }


            let time_spent = now.elapsed().as_millis();
            if time_spent < 16{
                let time_to_sleep = 16 - time_spent;
                eprintln!("Going to sleep for {} miliseconds", time_to_sleep);
                sleep(Duration::from_millis(time_to_sleep as u64));
            }
            else{
                eprintln!("Falling behind... last frame took {} miliseconds", time_spent);
            }
        }

    }
    pub fn open_rom(&mut self, rom_path: PathBuf){
        self.cpu.open_rom(rom_path);
    }
=======
extern crate sfml;

use crate::cpu::CPU;
use sfml::window::{Window, Event, Style};
use std::path::PathBuf;
use std::time::{Duration,Instant};
use std::thread::sleep;
use crate::cli;
use crate::interrupt_controller::InterruptFlags;
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
        let cycles_per_frame : u32 = 69905;
        let mut window = Window::new((640, 576), "GMBR Emulator", Style::CLOSE|Style::TITLEBAR, &Default::default());
        loop{
            let now = Instant::now();
            let mut total_cycles : u32 = 0;
            while total_cycles < cycles_per_frame{
                let cycles_elapsed = self.cpu.do_cycle() * 4;
                total_cycles += cycles_elapsed as u32;
                self.cpu.gpu.update_scanlines(cycles_elapsed);

                if self.cpu.gpu.stat_interrupt_req{
                    self.cpu.interrupt_controller.set_interrupt_flag(InterruptFlags::LCDStat);
                    self.cpu.gpu.stat_interrupt_req = false;
                }

                if self.cpu.gpu.vblank_interrupt_req{
                    self.cpu.interrupt_controller.set_interrupt_flag(InterruptFlags::VBlank);
                    self.cpu.gpu.vblank_interrupt_req = false;
                }

                if debug_mode {
                    self.cpu.print_registers();
                    //cli::read_any_key();
                }

                // User input

                while let Some(event) = window.poll_event() {
                    
                    match event {
                        Event::Closed | Event::KeyPressed {code: sfml::window::Key::Escape, ..} => window.close(),
                        _ => {},
                    }
                }
                window.set_active(true);
                    
                
            }

            window.display();
            let time_spent = now.elapsed().as_millis();
            if time_spent < 16{
                let time_to_sleep = 16 - time_spent;
                eprintln!("Going to sleep for {} miliseconds", time_to_sleep);
                sleep(Duration::from_millis(time_to_sleep as u64));
            }
            else{
                eprintln!("Falling behind... last frame took {} miliseconds", time_spent);
            }
        }

    }
    pub fn open_rom(&mut self, rom_path: PathBuf){
        self.cpu.open_rom(rom_path);
    }
>>>>>>> 563abd1b5ce6eec4fd88137b319a58fc465abb8f
}