struct Joypad{
    buttons_pressed: u8,
    keys_pressed: u8,
    joyp: u8,
}

impl Joypad{
    pub fn new() -> Joypad{
        Joypad{
            buttons_pressed: 0,
            keys_pressed: 0,
            joyp: 0,
        }
    }
}