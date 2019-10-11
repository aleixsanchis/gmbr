#![crate_name = "gmbr"]

use gmbr::device::Device;

fn main() {
    let device: Device::new();
    device.run();
    println!("Hello, world!");
}
