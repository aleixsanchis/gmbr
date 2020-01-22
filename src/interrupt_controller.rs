pub struct InterruptController{
    interrupt_master_enable: bool,
    interrupt_enable: u8,
    interrupt_flag: u8
}

impl InterruptController{
    pub fn new() -> InterruptController{
        InterruptController {
            interrupt_master_enable : false,
            interrupt_enable : 0,
            interrupt_flag : 0,
        }
    }

    pub fn set_interrupt_flag(&mut self, value: u8){
        self.interrupt_flag = value;
    }
    pub fn interrupt_flag(& self) -> u8{
        return self.interrupt_flag;
    }
    pub fn set_interrupt_enable(&mut self, value: u8){
        self.interrupt_enable = value;
    }
    pub fn interrupt_enable(& self) -> u8{
        return self.interrupt_enable;
    }
    pub fn disable_master_interrupt(&mut self){
        self.interrupt_master_enable = false;
    }
    pub fn enable_master_interrupt(&mut self){
        self.interrupt_master_enable = true;
    }
}