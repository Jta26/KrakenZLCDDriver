
use rusb::{Context, UsbContext, Device, DeviceHandle};
use std::time::Duration;
use crate::UsbConfig::{configure_bulk_endpoint, find_writable_endpoints, Endpoint};
use crate::KrakenDriverUtils;
use hex::FromHex;

const VID: u16 = 0x1e71;
const PID: u16 = 0x3008;

const WRITE_BULK_LENGTH: u32 = 512;
const WRITE_LENGTH: u32 = 64;

let mut kraken_device;

pub fn initialize() {
    let mut context = Context::new().unwrap();
    let (mut device, mut handle) = open_device(&mut context, VID, PID).unwrap();
    print_device_info(&mut handle);
    let endpoints = find_writable_endpoints(&mut device);

    for endpoint in endpoints {
        if endpoint.address == 0x02 {
            let mut bulk_endpoint = endpoint;

            let has_kernel_driver = match handle.kernel_driver_active(bulk_endpoint.iface) {
                Ok(true) => {
                    handle.detach_kernel_driver(bulk_endpoint.iface).unwrap();
                    true
                }
                _ => false,
            };
            println!("has kernel driver? {}", has_kernel_driver);
        }
    }
    return handle;
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

pub fn send_bulk_out<T: UsbContext>(endpoint: u8,handle: &mut DeviceHandle<T>, buffer: &[u8]) -> bool {
    let timeout = Duration::from_secs(2);
    let chunks = KrakenDriverUtils::slice_into_512(buffer);
    let header_buffer = <[u8; 512]>::from_hex("12fa01e8abcdef9876543210010000009cb01900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();

    let header_buffer_vec = header_buffer.to_vec();

    let header_write_result = handle.write_bulk(endpoint, &header_buffer_vec, timeout);
    match header_write_result {
        Ok(buf) => buf,
        Err(error) => {
            panic!("Problem sending header! {:?}", error);
        }
    };

    for chunk in chunks {
        // dbg!(chunk);
    let write_result = handle.write_bulk(1, &chunk, timeout);
    match write_result {
        Ok(buffer) => buffer,
        Err(error) => {
            panic!("Problem writing! {:?}", error);
        }
    };
    }

    return true;

}

pub fn control_response() {

}








pub fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) {
    let device_desc = handle.device().device_descriptor().unwrap();
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout).unwrap();

    println!("Active configuration: {}", handle.active_configuration().unwrap());

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }

}