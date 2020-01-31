pub const ROM_BANK_SIZE : usize = 0x4000; 
pub const BANK0_START : usize = 0x0000;
pub const BANK0_END : usize = 0x3FFF;
pub const BANKN_START : usize = 0x4000;
pub const BANKN_END : usize = 0x7FFF;
use std::path::PathBuf;


pub trait MBC{
    fn read_byte(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn write_byte(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16);
    fn open_rom(&mut self, rom_path: PathBuf);
}

