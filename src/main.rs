#![crate_name = "gmbr"]

use gmbr::device::Device;

fn main() {
    let device : Device = Device::new();
    device.run();
}
