use crate::mbc;

const ROM_SIZE : usize = 0x8000;
const VRAM_SIZE : usize = 0x4000;
pub struct MMU<'a>{
    mbc: &'a mut dyn mbc::MBC,
}

impl MMU<'_>{
    pub fn new() -> MMU<'static>{
        MMU{
            mbc: &mut mbc::MBC0::new(),
        }
    }
    pub fn read_byte(&self, address: u16) -> u8{
        return self.mbc.read_byte(address);
    }
    pub fn read_word(&self, address: u16) -> u16{
        return self.mbc.read_word(address);
    }
    pub fn write_byte(&self, address: u16, value: u8){
        self.mbc.write_byte(address, value);
    }
    pub fn write_word(&self, address: u16, value: u16){
        self.mbc.write_word(address, value);
    }
}