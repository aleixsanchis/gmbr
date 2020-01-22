const VRAM_SIZE : usize = 0x2000;
pub struct GPU{
    pub vram: [u8; VRAM_SIZE],
    scy: u8,
    scx: u8,
    lcdc: u8,

}

impl GPU{
    pub fn new() -> GPU{
        GPU{
            vram: [0; VRAM_SIZE],
            scy : 0,
            scx : 0,
            lcdc: 0,
        }
    }

    pub fn set_scy(&mut self, value: u8){
        self.scy = value;
    }

    pub fn set_scx(&mut self, value: u8){
        self.scx = value;
    }

    pub fn set_lcdc(&mut self, value: u8){
        // Only modifying Read/Write values
        self.lcdc = value&0b01111000
    }
}