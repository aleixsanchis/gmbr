use crate::mbc::MBC;
use crate::gpu::GPU;
use crate::interrupts::Interrupts;
use std::path::PathBuf;



const ROM_START : usize = 0x0000;
const ROM_END : usize = 0x7FFF;

const VRAM_START : usize = 0x8000;
const VRAM_END : usize = 0x9FFF;

const RAM_START : usize = 0xC000;
const RAM_END : usize = 0xDFFF;
const RAM_SIZE : usize = (RAM_END - RAM_START)+1;

const HRAM_START: usize = 0xFF80;
const HRAM_END : usize = 0xFFFE;
const HRAM_SIZE : usize = (HRAM_END - HRAM_START)+1;

const IE : usize = 0xFF0F;
const IF : usize = 0xFFFF;

pub struct MMU{
    mbc: Box<dyn MBC>,
    ram: [u8; RAM_SIZE],
    high_ram: [u8; HRAM_SIZE],
    pub gpu: GPU,
    pub interrupts: Interrupts,
}

impl MMU{
    pub fn new() -> MMU{
        MMU{
            mbc: Box::new(crate::mbc0::MBC0::new()),
            ram: [0; RAM_SIZE as usize],
            high_ram: [0; HRAM_SIZE as usize],
            gpu: GPU::new(),
            interrupts: Interrupts::new(),
        }
    }
    pub fn read_byte(&self, address: u16) -> u8{
        match address as usize {
            ROM_START..=ROM_END => return self.mbc.read_byte(address),
            VRAM_START..=VRAM_END => return self.gpu.vram[address as usize - VRAM_START],
            RAM_START..=RAM_END => return self.ram[(address as usize - RAM_START)],

            HRAM_START..=HRAM_END => return self.high_ram[(address as usize - HRAM_START)],

            IF => self.interrupts.get_interrupt_flag(),
            IE => self.interrupts.get_interrupt_enable(),
            
            _ => panic!("This memory section is not supported yet. The location was {:#6X}", address),

        }
        
    }
    pub fn read_word(&self, address: u16) -> u16{
        return (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8);
    }
    pub fn write_byte(&mut self, address: u16, value: u8){
        match address as usize {
            ROM_START..=ROM_END => return self.mbc.write_byte(address, value),

            RAM_START..=RAM_END => self.ram[(address as usize - RAM_START)] = value,

            HRAM_START..=HRAM_END => self.high_ram[(address as usize - HRAM_START)] = value,

            IF => self.interrupts.set_interrupt_flag(value),
            IE => self.interrupts.set_interrupt_enable(value),


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