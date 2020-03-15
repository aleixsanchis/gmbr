pub struct Joypad {
    buttons_pressed: u8,
    directions_pressed: u8,
    joyp: u8,
    pub joypad_interrupt_req: bool,
}

#[derive(Debug)]
pub enum KeyValue {
    Down,
    Up,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            buttons_pressed: 0x0F,
            directions_pressed: 0x0F,
            joyp: 0,
            joypad_interrupt_req: false,
        }
    }

    pub fn set_key_pressed(&mut self, key: KeyValue) {
        // 0 is pressed
        match key {
            KeyValue::Down => self.directions_pressed &= !0b0000_1000,
            KeyValue::Up => self.directions_pressed &= !0b0000_0100,
            KeyValue::Left => self.directions_pressed &= !0b0000_0010,
            KeyValue::Right => self.directions_pressed &= !0b0000_0001,

            KeyValue::Start => self.buttons_pressed &= !0b0000_1000,
            KeyValue::Select => self.buttons_pressed &= !0b0000_0100,
            KeyValue::B => self.buttons_pressed &= !0b0000_0010,
            KeyValue::A => self.buttons_pressed &= !0b0000_0001,
        }

        self.joypad_interrupt_req = true;

        println!("Key pressed! {:?}", key);
    }

    pub fn set_key_released(&mut self, key: KeyValue) {
        // 1 is not pressed
        match key {
            KeyValue::Down => self.directions_pressed |= 0b0000_1000,
            KeyValue::Up => self.directions_pressed |= 0b0000_0100,
            KeyValue::Left => self.directions_pressed |= 0b0000_0010,
            KeyValue::Right => self.directions_pressed |= 0b0000_0001,

            KeyValue::Start => self.buttons_pressed |= 0b0000_1000,
            KeyValue::Select => self.buttons_pressed |= 0b0000_0100,
            KeyValue::B => self.buttons_pressed |= 0b0000_0010,
            KeyValue::A => self.buttons_pressed |= 0b0000_0001,
        }
        println!("Key released! {:?}", key);
    }

    pub fn joyp(&self) -> u8 {
        // Button Keys
        if self.joyp & 0b0010_0000 == 0 {
            return self.buttons_pressed;
        } else {
            return self.directions_pressed;
        }
    }

    pub fn set_joyp(&mut self, value: u8) {
        self.joyp |= value & 0b0011_0000;
    }
}
