pub struct Joypad{
    buttons_pressed: u8,
    directions_pressed: u8,
    joyp: u8,
    pub joypad_interrupt_req: bool,
}

#[derive(Debug)]
pub enum KeysPressed{
    Down,
    Up,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}

impl Joypad{
    pub fn new() -> Joypad{
        Joypad{
            buttons_pressed: 0,
            directions_pressed: 0,
            joyp: 0,
            joypad_interrupt_req: false,
        }
    }

    pub fn set_key_pressed(&mut self, key: KeysPressed){
        match key{
            KeysPressed::Down => self.directions_pressed &= 0b0000_0111,
            KeysPressed::Up => self.directions_pressed &= 0b0000_1011,
            KeysPressed::Left => self.directions_pressed &= 0b0000_1101,
            KeysPressed::Right => self.directions_pressed &= 0b0000_1110,

            KeysPressed::Start => self.buttons_pressed &= 0b0000_0111,
            KeysPressed::Select => self.buttons_pressed &= 0b0000_1011,
            KeysPressed::B => self.buttons_pressed &= 0b0000_1101,
            KeysPressed::A => self.buttons_pressed &= 0b0000_1110,
        }

        println!("Key pressed! {:?}", key);
    }

    pub fn joyp(&self) -> u8{
        // Button Keys
        if self.joyp & 0b0010_0000 == 0b0010_0000 {
            return self.buttons_pressed;
        }
        else {
            return self.directions_pressed;
        }
    }

    pub fn set_joyp(&mut self, value: u8){
        self.joyp |= value & 0b0011_0000;
    }
}