#![crate_name = "gmbr"]

use gmbr::device::Device;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let device : Device = Device::new();
    device.run();
}
