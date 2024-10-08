#![windows_subsystem = "windows"]

use hidapi::{HidApi};

const R1_ON: [u8; 5] = [0x0,0xA0,0x01,0x01,0xA2];
const R2_ON: [u8; 5] = [0x0,0xA0,0x02,0x01,0xA3];
const R3_ON: [u8; 5] = [0x0,0xA0,0x03,0x01,0xA4];
const R4_ON: [u8; 5] = [0x0,0xA0,0x04,0x01,0xA5];

const VID: u16 = 0x5131;
const PID: u16 = 0x2007;


fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance");

    let device = api.open(VID, PID).ok().expect("Failed to open device");
    device.write(&R1_ON).expect("Failed to write buffer");
    device.write(&R2_ON).expect("Failed to write buffer");
    device.write(&R3_ON).expect("Failed to write buffer");
    device.write(&R4_ON).expect("Failed to write buffer");
}
