pub struct Timer{
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    timer_interrupt_req: bool,
    cycles_passed: u16,
}

impl Timer{
    pub fn new() -> Timer{
        Timer{
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            timer_interrupt_req: false,
            cycles_passed: 0,
        }
    }

    pub fn update_timer(&mut self, cycles: u16){
        self.cycles_passed+= cycles;
    }

    pub fn tma(&self) -> u8{
        return self.tma;
    }

    pub fn set_tma(&mut self, value: u8) {
        self.tma = value;
    }
}