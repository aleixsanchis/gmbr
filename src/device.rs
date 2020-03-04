extern crate sfml;

const WINDOW_WIDTH: u32 = 160;
const WINDOW_HEIGTH: u32 = 144;
const PIXEL_SCALE: u32 = 3;

use crate::cli;
use crate::cpu::CPU;
use crate::interrupt_controller::InterruptFlags;
use crate::joypad::KeysPressed;
use sfml::graphics::{RenderWindow, Sprite, Texture, RenderTarget, Transformable, Color};
use sfml::system::Vector2f;
use sfml::window::{Event, Style};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
pub struct Device {
    cpu: CPU,
}

impl Device {
    pub fn new() -> Device {
        Device { cpu: CPU::new() }
    }
    pub fn run(&mut self) -> () {
        let mut debug_mode: bool = false;
        let cycles_per_frame: u32 = 69905;
        let mut window = RenderWindow::new(
            (WINDOW_WIDTH * PIXEL_SCALE, WINDOW_HEIGTH * PIXEL_SCALE),
            "GMBR Emulator",
            Style::CLOSE | Style::TITLEBAR,
            &Default::default(),
        );
        window.set_active(true);
        let mut texture = Texture::new(WINDOW_WIDTH, WINDOW_HEIGTH).unwrap();
        loop {
            let now = Instant::now();
            let mut total_cycles: u32 = 0;
            while total_cycles < cycles_per_frame {
                let cycles_elapsed = self.cpu.do_cycle() * 4;
                total_cycles += cycles_elapsed as u32;

                if !self.cpu.cb_prefix {
                    if self.cpu.mmu.dma_transfer {
                        for i in 0..0xA0 {
                            self.cpu.gpu.write_byte_oam(
                                i as usize,
                                self.cpu.mmu.read_byte(self.cpu.mmu.dma_address),
                            );
                        }
                        self.cpu.mmu.dma_transfer = false;
                    }
                    self.cpu.gpu.update_scanlines(cycles_elapsed);

                    // User input
                    while let Some(event) = window.poll_event() {
                        match event {
                            Event::Closed
                            | Event::KeyPressed {
                                code: sfml::window::Key::Escape,
                                ..
                            } => {
                                window.close();
                                std::process::exit(0)
                            }
                            Event::KeyPressed {
                                code: sfml::window::Key::Up,
                                ..
                            } => self.cpu.joypad.set_key_pressed(KeysPressed::Up),
                            Event::KeyPressed {
                                code: sfml::window::Key::Down,
                                ..
                            } => self.cpu.joypad.set_key_pressed(KeysPressed::Down),
                            Event::KeyPressed {
                                code: sfml::window::Key::Left,
                                ..
                            } => self.cpu.joypad.set_key_pressed(KeysPressed::Left),
                            Event::KeyPressed {
                                code: sfml::window::Key::Right,
                                ..
                            } => self.cpu.joypad.set_key_pressed(KeysPressed::Right),

                            _ => {}
                        }
                    }

                    if self.cpu.gpu.stat_interrupt_req {
                        self.cpu
                            .interrupt_controller
                            .set_interrupt_flag(InterruptFlags::LCDStat);
                        self.cpu.gpu.stat_interrupt_req = false;
                    }

                    if self.cpu.gpu.vblank_interrupt_req {
                        self.cpu
                            .interrupt_controller
                            .set_interrupt_flag(InterruptFlags::VBlank);
                        self.cpu.gpu.vblank_interrupt_req = false;
                    }

                    if self.cpu.joypad.joypad_interrupt_req {
                        self.cpu
                            .interrupt_controller
                            .set_interrupt_flag(InterruptFlags::Joypad);
                        self.cpu.joypad.joypad_interrupt_req = false;
                    }

                    if self.cpu.interrupt_controller.ime() {

                        match self.cpu.interrupt_controller.get_first_interrupt() {
                            InterruptFlags::VBlank => {
                                self.cpu.push_to_stack(self.cpu.registers.pc);
                                self.cpu.registers.pc = 0x0040;
                                self.cpu.interrupt_controller.clear_interrupt_flag(InterruptFlags::VBlank);
                                total_cycles += 5;
                            },
                            InterruptFlags::LCDStat => {
                                self.cpu.push_to_stack(self.cpu.registers.pc);
                                self.cpu.registers.pc = 0x0048;
                                self.cpu.interrupt_controller.clear_interrupt_flag(InterruptFlags::LCDStat);
                                total_cycles += 5;
                            },
                            InterruptFlags::Joypad => {
                                self.cpu.push_to_stack(self.cpu.registers.pc);
                                self.cpu.registers.pc = 0x0060;
                                self.cpu.interrupt_controller.clear_interrupt_flag(InterruptFlags::Joypad);
                                total_cycles += 5;
                            },
                            _ => {},
                        }
                    }

                    
                    if debug_mode == true {
                        self.cpu.print_registers();
                        cli::read_any_key();
                    }

                    if self.cpu.registers.pc == 0x02cd  {
                        debug_mode = true;
                    } 
                }
            }
            
            unsafe{
                texture.update_from_pixels(&self.cpu.gpu.framebuffer, WINDOW_WIDTH, WINDOW_HEIGTH, 0, 0);
            }
            let mut background_sprite = Sprite::with_texture(&texture);
            background_sprite.set_scale(Vector2f::new(PIXEL_SCALE as f32, PIXEL_SCALE as f32));
            window.clear(Color::BLACK);
            window.draw(&background_sprite);
            window.display();
            let time_spent = now.elapsed().as_millis();
            if time_spent < 16 {
                let time_to_sleep = 16 - time_spent;
                //eprintln!("Going to sleep for {} miliseconds", time_to_sleep);
                sleep(Duration::from_millis(time_to_sleep as u64));
            } else {
                eprintln!(
                    "Falling behind... last frame took {} miliseconds",
                    time_spent
                );
            }
        }
    }
    pub fn open_rom(&mut self, rom_path: PathBuf) {
        self.cpu.open_rom(rom_path);
    }
}
