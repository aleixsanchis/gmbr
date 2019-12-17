#![crate_name = "gmbr"]
#![crate_type = "lib" ]

pub mod device;
pub mod cli;

mod cpu;
mod registers;
mod mmu;
mod mbc;
mod mbc0;
