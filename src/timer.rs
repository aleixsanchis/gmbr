pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    timer_interrupt_req: bool,
    cycles_passed: u16,
    _div: u16,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            timer_interrupt_req: false,
            cycles_passed: 0,
            _div: 0,
        }
    }

    pub fn update_timer(&mut self, cycles: u16) {
        self._div += cycles;

        if self._div >= 256 {
            self._div = 0;
            self.div.wrapping_add(1);
        }
    }

    pub fn tma(&self) -> u8 {
        return self.tma;
    }

    pub fn set_tma(&mut self, value: u8) {
        self.tma = value;
    }

    pub fn div(&mut self) -> u8 {
        return self.div;
    }

    pub fn set_div(&mut self, value: u8) {
        self.div = 0;
    }
}
