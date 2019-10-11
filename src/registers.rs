pub struct Registers{
    pub a: u8,
}

impl Registers{
    pub fn new() -> Registers{
        Registers{
            a: 0x01,
        }
    }
}