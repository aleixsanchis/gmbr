use crate::mbc::MBC;
const ROM_START : usize = 0x0000;
const VRAM_START : usize = 0x8000;
const ROM_SIZE : usize = 0x8000;
const VRAM_SIZE : usize = 0x2000;
pub struct MMU{
    mbc: Box<dyn MBC>,
}

impl MMU{
    pub fn new() -> MMU{
        MMU{
            mbc: Box::new(crate::mbc0::MBC0::new()),
        }
    }
    pub fn read_byte(&self, address: u16) -> u8{
        return self.mbc.read_byte(address);
    }
    pub fn read_word(&self, address: u16) -> u16{
        return self.mbc.read_word(address);
    }
    pub fn write_byte(&mut self, address: u16, value: u8){
        self.mbc.write_byte(address, value);
    }
    pub fn write_word(&mut self, address: u16, value: u16){
        self.mbc.write_word(address, value);
    }
}