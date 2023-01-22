use std::{fs::read};

use kraken_controller::KrakenController;

extern crate rusb;
extern crate hex;


mod usb_config;
mod kraken_controller;
mod kraken_driver;
mod kraken_driver_utils;

fn main() {
    let mai_gif = read("./img/maisan.gif").unwrap();
    let controller = KrakenController::new();
    controller.set_image(mai_gif);
    // controller.set_blank();

}