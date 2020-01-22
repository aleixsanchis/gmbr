pub const ROM_START : usize = 0x0000;
pub const ROM_END : usize = 0x7FFF;

pub const RAM_START : usize = 0xC000;
pub const RAM_END : usize = 0xDFFF;

pub const VRAM_START : usize = 0x8000;
pub const VRAM_END : usize = 0x9FFF;

pub const HRAM_START: usize = 0xFF80;
pub const HRAM_END : usize = 0xFFFE;

// IO REGISTERS

// VIDEO
pub const LCDC : usize = 0xFF41;
pub const SCY : usize = 0xFF42;
pub const SCX : usize = 0xFF43;

// INTERRUPTS
pub const IE : usize = 0xFF0F;
pub const IF : usize = 0xFFFF;

// LINK CABLE
pub const SB : usize = 0xFF01;
pub const SC : usize = 0xFF02; 

