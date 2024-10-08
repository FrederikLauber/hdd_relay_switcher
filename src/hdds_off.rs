#![windows_subsystem = "windows"]

use hidapi::{HidApi};

const R1_OFF: [u8; 5] = [0x0,0xA0,0x01,0x00,0xA1];
const R2_OFF: [u8; 5] = [0x0,0xA0,0x02,0x00,0xA2];
const R3_OFF: [u8; 5] = [0x0,0xA0,0x03,0x00,0xA3];
const R4_OFF: [u8; 5] = [0x0,0xA0,0x04,0x00,0xA4];

const VID: u16 = 0x5131;
const PID: u16 = 0x2007;


fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance");

    let device = api.open(VID, PID).ok().expect("Failed to open device");
    device.write(&R1_OFF).expect("Failed to write buffer");
    device.write(&R2_OFF).expect("Failed to write buffer");
    device.write(&R3_OFF).expect("Failed to write buffer");
    device.write(&R4_OFF).expect("Failed to write buffer");
}