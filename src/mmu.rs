use crate::mbc::MBC;

const ROM_SIZE : usize = 0x8000;
const VRAM_SIZE : usize = 0x4000;
pub struct MMU{
    rom: [u8; ROM_SIZE],
    mbc: MBC,
}

impl MMU{
    pub fn new() -> MMU{
        MMU{
            
        }
    }
    pub fn read_byte(&self, address: u16) -> u8{
        return 0;
    }
    pub fn read_word(&self, address: u16) -> u16{
        return 0;
    }
}