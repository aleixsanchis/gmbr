pub struct Interrupts{
    interrupt_master_enable: bool,
    interrupt_enable: u8,
    interrupt_flag: u8
}

impl Interrupts{
    pub fn new() -> Interrupts{
        Interrupts {
            interrupt_master_enable : false,
            interrupt_enable : 0,
            interrupt_flag : 0,
        }
    }

    pub fn set_interrupt_flag(&mut self, value: u8){
        self.interrupt_flag = value;
    }
    pub fn get_interrupt_flag(&mut self) -> u8{
        return self.interrupt_flag;
    }
    pub fn set_interrupt_enable(&mut self, value: u8){
        self.interrupt_enable = value;
    }
    pub fn get_interrupt_enable(&mut self) -> u8{
        return self.interrupt_enable;
    }
}