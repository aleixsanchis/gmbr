#![crate_name = "gmbr"]
#![crate_type = "lib"]

pub mod cli;
pub mod device;

mod apu;
mod cpu;
mod gpu;
mod interrupt_controller;
mod joypad;
mod link_cable;
mod mbc;
mod mbc0;
mod memory_map;
mod mmu;
mod registers;
