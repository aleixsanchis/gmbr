use crate::mbc::MBC;
use std::path::PathBuf;



const ROM_START : u16 = 0x0000;
const ROM_END : u16 = 0x7FFF;

const VRAM_START : u16 = 0x8000;
const VRAM_SIZE : u16 = 0x9FFF;

const RAM_START : u16 = 0xC000;
const RAM_END : u16 = 0xDFFF;
const RAM_SIZE : u16 = RAM_END - RAM_START;

const HRAM_START: u16 = 0xFF80;
const HRAM_END : u16 = 0xFFFE;
const HRAM_SIZE : u16 = HRAM_END - HRAM_START;

pub struct MMU{
    mbc: Box<dyn MBC>,
    ram: [u8; (RAM_SIZE+1) as usize],
    high_ram: [u8; (HRAM_SIZE+1) as usize],
}

impl MMU{
    pub fn new() -> MMU{
        MMU{
            mbc: Box::new(crate::mbc0::MBC0::new()),
            ram: [0; (RAM_SIZE+1) as usize],
            high_ram: [0; (HRAM_SIZE+1) as usize]
        }
    }
    pub fn read_byte(&self, address: u16) -> u8{
        match address {
            ROM_START..=ROM_END => return self.mbc.read_byte(address),
            RAM_START..=RAM_END => return self.ram[(address - RAM_START) as usize],

            HRAM_START..=HRAM_END => return self.high_ram[(address - HRAM_START) as usize],
            _ => panic!("This memory section is not supported yet. The location was {:#6X}", address),

        }
        
    }
    pub fn read_word(&self, address: u16) -> u16{
        return (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8);
    }
    pub fn write_byte(&mut self, address: u16, value: u8){
        match address {
            ROM_START..=ROM_END => return self.mbc.write_byte(address, value),

            RAM_START..=RAM_END => self.ram[(address - RAM_START)as usize] = value,

            HRAM_START..=HRAM_END => self.high_ram[(address - HRAM_START) as usize] = value,
            _ => panic!("This memory section is not supported yet. The location was {:#6X}", address),

        }
    }
    pub fn write_word(&mut self, address: u16, value: u16){
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn open_rom(&mut self, rom_path : PathBuf){
        self.mbc.open_rom(rom_path);
    }
}