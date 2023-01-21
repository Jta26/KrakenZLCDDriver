use std::{fs::read};

use rusb::{UsbContext, DeviceHandle, Device, Context};


extern crate rusb;
extern crate hex;

mod UsbConfig;
mod KrakenController;
mod KrakenDriver;
mod KrakenDriverUtils;

fn main() {
    KrakenDriver::initialize();

    let sexy_bunny_gif = read("./src/bunnyge.gif").unwrap();
    KrakenController::setImage(sexy_bunny_gif);
}