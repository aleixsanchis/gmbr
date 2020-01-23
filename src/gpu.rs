use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
const VRAM_SIZE : usize = 0x2000;
pub struct GPU{
    pub vram: [u8; VRAM_SIZE],
    scy: u8,
    scx: u8,
    stat: u8,
    lcdc: u8,
    ly: u8,
    lyc: u8,

}

impl GPU{
    pub fn new() -> GPU{
        GPU{
            vram: [0; VRAM_SIZE],
            lcdc: 0,
            stat: 0,
            scy : 0,
            scx : 0,
            ly: 0,
            lyc: 0,
        }
    }

    pub fn set_lcdc(&mut self, value: u8){
        self.lcdc = value;
    }

    pub fn set_stat(&mut self, value: u8){
        // Only modifying Read/Write values
        self.stat = value&0b01111000

        // TODO Game Boy makes the LCD interrupt sometimes trigger when writing to STAT 
        // (including writing $00) during OAM scan, H-Blank, V-Blank, or LY=LYC. 
        // It behaves as if $FF were written for one cycle, and then the written 
        // value were written the next cycle.
    }

    pub fn stat(&self) -> u8{
        return self.stat;
    }

    pub fn set_scy(&mut self, value: u8){
        self.scy = value;
    }

    pub fn set_scx(&mut self, value: u8){
        self.scx = value;
    }

    pub fn ly(&self) -> u8{
        return self.ly;
    }

    pub fn set_lyc(&mut self, value: u8){
        self.lyc = value;
    }
}