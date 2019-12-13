use crate::mbc;
const MBC0_ROM_SIZE : usize = 0x8000;
pub struct MBC0{
    rom: [u8; mbc::ROM_BANK_SIZE],
}

impl mbc::MBC for MBC0{
    fn read_byte(&self, address: u16) -> u8{
        return self.rom[address as usize];
    }
    fn read_word(&self, address: u16) -> u16{

        let mut value : u16 = self.rom[address as usize] as u16;
        value = value | ((self.rom[(address+1) as usize] as u16) <<8);

        return value;
    }
    fn write_byte(&mut self, address: u16, value: u8){

        self.rom[address as usize] = value;
    }
    fn write_word(&mut self, address: u16, value: u16){

        self.rom[address as usize ] = value as u8;
        self.rom[address as usize +1] = (value>>8) as u8;
    }
}

impl MBC0{
    pub fn new() -> MBC0{
        MBC0{
            rom: [0; mbc::ROM_BANK_SIZE],
        }
    }
}