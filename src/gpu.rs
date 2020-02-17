extern crate bit_field;
use bit_field::BitField;

const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0xA0;
const LYC_INTERRUPT_ENABLED: u8 = 0b0100_0000;
const OAM_INTERRUPT_ENABLED: u8 = 0b0010_0000;
const VBLANK_INTERRUPT_ENABLED: u8 = 0b001_0000;
const HBLANK_INTERRUPT_ENABLED: u8 = 0b0000_1000;
const FRAMEBUFFER_WIDTH: usize = 160;
const FRAMEBUFFER_HEIGTH: usize = 144;
const FRAMEBUFFER_SIZE: usize = FRAMEBUFFER_WIDTH*FRAMEBUFFER_HEIGTH;

pub struct GPU{
    pub vram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub framebuffer: [u8; FRAMEBUFFER_SIZE],
    mode_counter: u32,
    line: u8,
    mode: GPU_modes,
    pub vblank_interrupt_req: bool,
    pub stat_interrupt_req: bool,

    scy: u8,
    scx: u8,
    stat: u8,
    lcdc: u8,
    //ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
}

#[derive(PartialEq)]
enum GPU_modes{
    OAMSearch,
    ActivePicture,
    HBlank,
    VBlank,
}

impl GPU{
    pub fn new() -> GPU{
        GPU{
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            framebuffer: [0; FRAMEBUFFER_SIZE],
            mode_counter: 0,
            line: 0,
            mode: GPU_modes::OAMSearch,
            vblank_interrupt_req: false,
            stat_interrupt_req: false,

            lcdc: 0,
            stat: 0,
            scy : 0,
            scx : 0,
            //ly: 0,
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
        }
    }

    pub fn write_byte_vram(&mut self, address: usize, value: u8){
        self.vram[address] = value;
    }

    pub fn write_byte_oam(&mut self, address: usize, value: u8){
        self.oam[address] = value;
    }

    pub fn read_byte_vram(& self, address: usize) -> u8{
        return self.vram[address];
    }

    pub fn read_byte_oam(& self, address: usize) -> u8{
        return self.oam[address];
    }

    fn draw_scanline(&mut self){
        if self.lcd_on() && (self.line as usize) < FRAMEBUFFER_HEIGTH{

            // Background
            let framebuffer_offset = self.line as usize * FRAMEBUFFER_WIDTH;

            if self.sprites_on(){

            }
        }
    }
    pub fn update_scanlines(&mut self, cycles: u8){
        if !self.lcd_on() {
            return;
        }

        let mut dots_left = cycles;

        while dots_left > 0 {
            let iteration_dots;

            if dots_left >= 80 {
                iteration_dots = 80;
            }
            else {
                iteration_dots = dots_left;
            }

            self.mode_counter += iteration_dots as u32;
            dots_left -= iteration_dots;

            // We finished a line (114 cpu cycles or 456 dots)
            if self.mode_counter >= 456 {
                self.mode_counter -= 456;
                // 144 lines + 10 of VBLANK
                self.draw_scanline();
                self.line = self.line + 1;
                if self.line == 154 {
                    self.line = 0;
                }

                self.check_lyc_interrupt();
            }

            if self.line >= 144 && self.mode != GPU_modes::VBlank{
                self.change_mode(GPU_modes::VBlank)
            }

            else if self.line < 144 {
                if self.mode_counter <= 80 && self.mode != GPU_modes::OAMSearch {
                    self.change_mode(GPU_modes::OAMSearch);
                }
                // TODO maybe actually use the correct value based on window and scrolling register
                else if self.mode_counter <= 252 && self.mode != GPU_modes::ActivePicture {
                    self.change_mode(GPU_modes::ActivePicture);
                }
                else{
                    self.change_mode(GPU_modes::HBlank);
                }
            }
        }
    }

    fn lcd_on(&self) -> bool{
        return self.lcdc & 0x80 == 0x80;
    }

    fn sprites_on(&self) -> bool{
        return self.lcdc.get_bit(1);
    }

    fn lyc_interrupt_enabled(&self) -> bool{
        return self.stat & LYC_INTERRUPT_ENABLED == LYC_INTERRUPT_ENABLED;
    }

    fn check_lyc_interrupt(&mut self){
        if self.lyc == self.line && self.lyc_interrupt_enabled() {
            self.stat_interrupt_req = true;
        }
    }
    fn change_mode(&mut self, new_mode: GPU_modes){
        self.mode = new_mode;
        self.stat &= 0b0111_1100;
        match self.mode {
            // TODO 
            GPU_modes::VBlank => {
                self.stat |= 0b0000_0001;
                self.vblank_interrupt_req = true;
            },

            GPU_modes::OAMSearch => {
                self.stat |= 0b0000_0010;

            },

            GPU_modes::ActivePicture => {
                self.stat |= 0b0000_0011;
            },

            GPU_modes::HBlank => {
            },
        }

    }

    pub fn set_lcdc(&mut self, value: u8){
        self.lcdc = value;
    }

    pub fn set_stat(&mut self, value: u8){
        // Only modifying Read/Write values
        self.stat = value&0b0111_1000



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
        return self.line;
    }

    pub fn set_lyc(&mut self, value: u8){
        self.lyc = value;
    }

    pub fn bgp(&self) -> u8{
        return self.bgp;
    }

    pub fn set_bgp(&mut self, value: u8){
        self.bgp = value;
    }

    pub fn obp0(&self) -> u8{
        return self.obp0;
    }

    pub fn set_obp0(&mut self, value: u8){
        self.obp0 = value;
    }

    pub fn obp1(&self) -> u8{
        return self.obp1;
    }

    pub fn set_obp1(&mut self, value: u8){
        self.obp1 = value;
    }
}