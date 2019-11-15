
pub struct Registers{
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    flags: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}

pub enum CpuFlags
{
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl Registers{
    pub fn new() -> Registers{
        Registers{
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            flags: 0xB0,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc:0x0100
        }
    }

    pub fn af(&self) -> u16 {
        return ((self.a as u16) << 8) | ((self.flags & 0xF0) as u16);
    }

    pub fn bc(&self) -> u16{
        return ((self.b as u16) << 8) | (self.c as u16);
    }

    pub fn de(&self) -> u16{
        return ((self.d as u16) << 8) | (self.e as u16);
    }

    pub fn hl(&self) -> u16{
        return ((self.h as u16) << 8) | (self.l as u16);
    }

    pub fn setaf(&mut self, value: u16) -> (){
        self.a = (value >> 8) as u8;
        self.flags = (value & 0x00F0) as u8;
    }

    pub fn setbc(&mut self, value: u16) -> (){
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn setde(&mut self, value: u16) -> (){
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn sethl(&mut self, value: u16) -> (){
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn increment_hl(&mut self) -> (){
        let value = self.hl();
        value.wrapping_add(1);
        self.sethl(value);
    }

    pub fn decrement_hl(&mut self) -> (){
        let value = self.hl();
        value.wrapping_sub(1);
        self.sethl(value);
    }

    pub fn set_flags(&mut self, flags: CpuFlags, set: bool){
        let mask = flags as u8;
        match set {
            true => self.flags |= mask,
            false => self.flags &= !mask,
        }
        self.flags &= 0xF0;
    }

    pub fn get_flag(&mut self, flag: CpuFlags) -> bool{
        return self.flags & (flag as u8) != 0;
    }
}