use crate::mbc::MBC;
use crate::memory_map::*;
use std::path::PathBuf;

const RAM_SIZE: usize = (RAM_END - RAM_START) + 1;

const HRAM_SIZE: usize = (HRAM_END - HRAM_START) + 1;

pub struct MMU {
    mbc: Box<dyn MBC>,
    ram: [u8; RAM_SIZE],
    high_ram: [u8; HRAM_SIZE],
    pub dma_transfer: bool,
    pub dma_address: u16,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            mbc: Box::new(crate::mbc0::MBC0::new()),
            ram: [0; RAM_SIZE as usize],
            high_ram: [0; HRAM_SIZE as usize],
            dma_transfer: false,
            dma_address: 0,
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address as usize {
            ROM_START..=ROM_END => return self.mbc.read_byte(address),
            RAM_START..=RAM_END => return self.ram[(address as usize - RAM_START)],
            HRAM_START..=HRAM_END => return self.high_ram[(address as usize - HRAM_START)],

            _ => {
                println!("Invalid Read! This memory section is not supported (yet?). The location was {:#6X}", address);
                0xFF
            }
        }
    }
    pub fn read_word(&self, address: u16) -> u16 {
        return (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8);
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address as usize {
            ROM_START..=ROM_END => return self.mbc.write_byte(address, value),
            RAM_START..=RAM_END => self.ram[(address as usize - RAM_START)] = value,
            HRAM_START..=HRAM_END => self.high_ram[(address as usize - HRAM_START)] = value,

            _ => println!("Invalid Write! This memory section is not supported (yet?). The location was {:#6X}", address),

        }
    }
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn open_rom(&mut self, rom_path: PathBuf) {
        self.mbc.open_rom(rom_path);
    }

    pub fn start_dma(&mut self, offset: u8) {
        self.dma_transfer = true;
        self.dma_address = ((offset as u16) << 8) & 0xFF00;
    }
}
