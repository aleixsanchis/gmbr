#![crate_name = "gmbr"]

extern crate config;


use gmbr::device::Device;


fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("conf/conf.toml")).unwrap();
    let mut device : Device = Device::new();
    let rom_path = gmbr::cli::choose_rom(&settings);
    device.open_rom(rom_path);
    device.run();
}

