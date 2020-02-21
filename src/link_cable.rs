pub struct LinkCable {
    sb: u8,
    sc: u8,
}

impl LinkCable {
    pub fn new() -> LinkCable {
        LinkCable { sb: 0, sc: 0 }
    }

    pub fn sb(&self) -> u8 {
        return self.sb;
    }

    pub fn set_sb(&mut self, value: u8) {
        self.sb = value;
    }

    pub fn sc(&self) -> u8 {
        return self.sc;
    }

    pub fn set_sc(&mut self, value: u8) {
        self.sc = value;
    }
}
