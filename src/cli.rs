extern crate config;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub fn choose_rom(settings : &config::Config) -> PathBuf{
    let roms_folder = fs::read_dir(settings.get_str("roms_folder").unwrap()).unwrap();
    let roms_number = roms_folder.count();
    if roms_number < 1 {
        panic!("Error: the folder has no ROMS");
    }
    let roms_folder = fs::read_dir(settings.get_str("roms_folder").unwrap()).unwrap();
    println!("Select which ROM to open: \n");
    
    for (i, rom) in roms_folder.enumerate() {
        if let Ok(rom) = rom{
            println!("{}: {:?}", i, rom.file_name());
        }
        
    }
    print!("Please write the entry number between 0 and {}: ", roms_number-1);
    io::stdout().flush().unwrap();
    let mut scanned_line = String::new();
    let mut chosen_entry: usize = 0;
    let mut valid_input = false;
    while !valid_input {
        match io::stdin().read_line(&mut scanned_line){
            Ok(_nbytes) => {
                match scanned_line.trim().parse::<usize>() {
                    Ok(parsed_number) => {
                        if parsed_number < roms_number {
                            chosen_entry = parsed_number;
                            valid_input = true;
                        }
                        else{
                            eprintln!("Entry chosen not in range, please select an entry number between 0 and {}", roms_number-1);
                            scanned_line.clear();
                        }
                    }
                    Err(_error) => {eprintln!("Not a number, please try again.");}
                }
                
            }
            Err(_error) => {eprintln!("Failed to read from stdin... Sorry. Opening first ROM in the folder");}
        }
    }
    let roms_folder = fs::read_dir(settings.get_str("roms_folder").unwrap()).unwrap();
    let mut rom_path = PathBuf::new();
    for (i, rom) in roms_folder.enumerate() {
        if i == chosen_entry {
            rom_path = rom.unwrap().path();
        }
        
    }
    return rom_path;
}