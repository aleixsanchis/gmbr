use crate::cpu::CPU;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::str;
pub struct Device{
    cpu: CPU,
    rom_size : usize,
    ram_size : usize,

}

impl Device{
    pub fn new() -> Device{
        Device{
            cpu: CPU::new(),
            rom_size : 0,
            ram_size : 0,
        }
    }
    pub fn run(&mut self) -> () {
        self.cpu.run();
    }
    pub fn open_rom(&mut self, rom_path: PathBuf) -> (){
        let mut rom_file = File::open(rom_path).expect("Error opening the ROM file");
        let mut rom_contents = Vec::new();
        rom_file.read_to_end(&mut rom_contents).unwrap();
        let header = &rom_contents[0x0100..0x014F];
        let mut game_title : String = str::from_utf8(&header[0x34..0x43]).unwrap().to_string();
        game_title.retain(|c| c != '\0');
        println!("Opening the game {:#?}", game_title);
        match header[0x47]{
            0x00 => {println!("The game uses no MBC, so it's supported. Continuing...");}
            _ => {panic!("This game uses an unsupported MBC! Closing the game")}
        }
        
    }
}