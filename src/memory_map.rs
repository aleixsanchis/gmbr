// MEMORY
pub const ROM_START: usize = 0x0000;
pub const ROM_END: usize = 0x7FFF;

pub const RAM_START: usize = 0xC000;
pub const RAM_END: usize = 0xDFFF;

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;

pub const MIRROR_START: usize = 0xE000;
pub const MIRROR_END: usize = 0xFDFF;

pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;

pub const UNUSED_AREA_START: usize = 0xFEA0;
pub const UNUSED_AREA_END: usize = 0xFEFF;

pub const HRAM_START: usize = 0xFF80;
pub const HRAM_END: usize = 0xFFFE;

// IO REGISTERS

// JOYPAD
pub const JOYP: usize = 0xFF00;


// LINK CABLE
pub const SB: usize = 0xFF01;
pub const SC: usize = 0xFF02;

// TIMER
pub const DIV: usize = 0xFF04;
pub const TIMA: usize = 0xFF05;
pub const TMA: usize = 0xFF06;
pub const TAC: usize = 0xFF07;

// SOUND
pub const NR10: usize = 0xFF10;
pub const NR11: usize = 0xFF11;
pub const NR12: usize = 0xFF12;
pub const NR13: usize = 0xFF13;
pub const NR14: usize = 0xFF14;
pub const NR16: usize = 0xFF21;
pub const NR17: usize = 0xFF22;
pub const NR18: usize = 0xFF23;
pub const NR19: usize = 0xFF24;
pub const NR30: usize = 0xFF1A;
pub const NR31: usize = 0xFF1B;
pub const NR32: usize = 0xFF1C;
pub const NR33: usize = 0xFF1D;
pub const NR34: usize = 0xFF1E;
pub const NR41: usize = 0xFF20;
pub const NR42: usize = 0xFF21;
pub const NR43: usize = 0xFF22;
pub const NR44: usize = 0xFF23;
pub const NR50: usize = 0xFF24;
pub const NR51: usize = 0xFF25;
pub const NR52: usize = 0xFF26;
pub const WPR_START: usize = 0xFF30;
pub const WPR_END: usize = 0xFF3F;

// VIDEO
pub const LCDC: usize = 0xFF40;
pub const STAT: usize = 0xFF41;
pub const SCY: usize = 0xFF42;
pub const SCX: usize = 0xFF43;
pub const LY: usize = 0xFF44;
pub const LYC: usize = 0xFF45;
pub const DMA: usize = 0xFF46;
pub const BGP: usize = 0xFF47;
pub const OBP0: usize = 0xFF48;
pub const OBP1: usize = 0xFF49;
pub const WY: usize = 0xFF4A;
pub const WX: usize = 0xFF4B;

// INTERRUPTS
pub const IF: usize = 0xFF0F;
pub const IE: usize = 0xFFFF;
