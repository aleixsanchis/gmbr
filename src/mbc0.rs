use crate::mbc;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str;
const MBC0_ROM_SIZE: usize = 0x8000;
pub struct MBC0 {
    rom: Vec<u8>,
}

impl mbc::MBC for MBC0 {
    fn read_byte(&self, address: u16) -> u8 {
        return self.rom[address as usize];
    }
    fn read_word(&self, address: u16) -> u16 {
        return (self.rom[address as usize] as u16)
            | ((self.rom[(address + 1) as usize] as u16) << 8);
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        // There are no write operations defined for MBC0 because we can't swap banks
        return; //self.rom[address as usize] = value;
    }
    fn write_word(&mut self, address: u16, value: u16) {
        self.rom[address as usize] = value as u8;
        self.rom[(address + 1) as usize] = (value >> 8) as u8;
    }

    fn open_rom(&mut self, rom_path: PathBuf) {
        let mut rom_file = File::open(rom_path).expect("Error opening the ROM file");
        rom_file.read_to_end(&mut self.rom).unwrap();

        self.check_magic_number();

        let header = &self.rom[0x0100..0x014F];
        let mut game_title: String = str::from_utf8(&header[0x34..0x43]).unwrap().to_string();
        game_title.retain(|c| c != '\0');
        println!("Opening the game {:#?}", game_title);
        match header[0x47] {
            0x00 => {
                println!("The game uses no MBC, so it's supported. Continuing...");
            }
            _ => panic!(
                "This game uses an unsupported MBC (MBC{}) Closing the emulator...",
                header[0x47]
            ),
        }
        //Rom size is indicated in byte 0x148
        let rom_size: usize = 32768 << header[0x48];
        assert_eq!(self.rom.len(), rom_size, "The ROM size reported by the header and the ROM file size don't match. Maybe the file is corrupted. Closing the emulator...");
        if header[0x49] != 0 {
            panic!("External RAM not implemented yet! Closing the emulator...")
        }
    }
}

impl MBC0 {
    pub fn new() -> MBC0 {
        MBC0 { rom: Vec::new() }
    }

    fn check_magic_number(&self) {
        assert_eq!(self.rom[0x0100], 0x00 as u8, "This is not a GameBoy Game!");
        assert_eq!(self.rom[0x0101], 0xC3 as u8, "This is not a GameBoy Game!");
    }
}
