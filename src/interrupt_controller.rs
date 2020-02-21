pub struct InterruptController {
    interrupt_master_enable: bool,
    ie: u8,
    iflag: u8,
}

pub enum InterruptFlags {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl InterruptController {
    pub fn new() -> InterruptController {
        InterruptController {
            interrupt_master_enable: false,
            ie: 0,
            iflag: 0,
        }
    }

    pub fn set_iflag(&mut self, value: u8) {
        self.iflag = value;
    }
    pub fn iflag(&self) -> u8 {
        return self.iflag;
    }
    pub fn set_ie(&mut self, value: u8) {
        self.ie = value;
    }
    pub fn ie(&self) -> u8 {
        return self.ie;
    }
    pub fn disable_master_interrupt(&mut self) {
        self.interrupt_master_enable = false;
    }
    pub fn enable_master_interrupt(&mut self) {
        self.interrupt_master_enable = true;
    }
    pub fn set_interrupt_flag(&mut self, flag: InterruptFlags) {
        match flag {
            InterruptFlags::VBlank => self.iflag |= 0b0000_0001,
            InterruptFlags::LCDStat => self.iflag |= 0b0000_0010,
            InterruptFlags::Timer => self.iflag |= 0b0000_0100,
            InterruptFlags::Serial => self.iflag |= 0b0000_1000,
            InterruptFlags::Joypad => self.iflag |= 0b0001_0000,
        }
    }
}
