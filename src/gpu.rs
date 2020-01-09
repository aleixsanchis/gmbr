const VRAM_SIZE : usize = 0x2000;
pub struct GPU{
    pub vram: [u8; VRAM_SIZE]
}

impl GPU{
    pub fn new() -> GPU{
        GPU{
            vram: [0; VRAM_SIZE],
        }
    }
}