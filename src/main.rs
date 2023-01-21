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
    let mai_gif = read("./img/maisan.gif").unwrap();
    KrakenController::setImage(mai_gif);
}