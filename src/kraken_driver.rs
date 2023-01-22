
use rusb::{Context, UsbContext, Device, DeviceHandle};
use std::time::Duration;
use crate::usb_config::{configure_interrupt_endpoint, find_writable_endpoints, Endpoint};
use crate::kraken_driver_utils;
use hex::FromHex;

const VID: u16 = 0x1e71;
const PID: u16 = 0x3008;

const WRITE_BULK_LENGTH: usize = 512;
const WRITE_LENGTH: usize = 64;

const SWITCH_ADDRESS: u8 = 0x38;

pub struct KrakenDriver {
    handle: DeviceHandle<Context>,
    bulk_endpoint: Endpoint,
    interrupt_write_endpoint: Endpoint,
    interrupt_read_endpoint: Endpoint,
    image_index: u8
}
impl KrakenDriver {
    pub fn new() -> Self {
        let mut context = Context::new().unwrap();
        let (mut device, mut handle) = Self::open_device(&mut context, VID, PID).unwrap();
        Self::print_device_info(&mut handle);
        let endpoints = find_writable_endpoints(&mut device);
        let mut bulk_endpoint = None;
        let mut interrupt_write_endpoint = None;
        let mut interrupt_read_endpoint = None;
        for endpoint in endpoints {
            if endpoint.address == 0x02 {
                bulk_endpoint = Some(endpoint);
            }else if endpoint.address == 0x01 {
                interrupt_write_endpoint = Some(endpoint);
            } else if endpoint.address == 0x81 {
                interrupt_read_endpoint = Some(endpoint);
            }
            // Maybe we don't need on windows????
            // let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
            //     Ok(true) => {
            //         handle.detach_kernel_driver(endpoint.iface).unwrap();
            //         true
            //     }
            //     _ => false,
            // };
            // println!("has kernel driver? {}", has_kernel_driver);
        }
        let bulk_endpoint = bulk_endpoint.unwrap();
        let interrupt_write_endpoint = interrupt_write_endpoint.unwrap();
        let interrupt_read_endpoint = interrupt_read_endpoint.unwrap();

        let image_index = 1;
        KrakenDriver {
            handle,
            bulk_endpoint,
            interrupt_write_endpoint,
            interrupt_read_endpoint,
            image_index
        }
   
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

    pub fn send_switch(&mut self, index: u8, mode: u8) {
        let timeout = Duration::from_secs(2);
        let endpoint = &self.interrupt_write_endpoint;
        let handle = &mut self.handle;
        configure_interrupt_endpoint(handle, endpoint);
        let mut buffer: [u8; WRITE_LENGTH] = [0x0; WRITE_LENGTH];
        
        buffer[0] = SWITCH_ADDRESS;
        buffer[1] = 0x1;
        buffer[2] = mode;
        buffer[3] = index;
        let write_result = handle.write_interrupt(endpoint.address, &buffer, timeout);
        match write_result {
            Ok(r) => r,
            Err(error) => panic!("error: {:?}", error)
        };
        let release_result = handle.release_interface(endpoint.iface);
        match release_result {
            Ok(r) => r,
            Err(error) => panic!("Failed to Release Interface: {:?}", error)
        };
        ();
    }
    
    pub fn send_query(&mut self, index: u8, asset: u8) {
        let timeout = Duration::from_secs(2);
        let endpoint = &self.interrupt_write_endpoint;
        let handle = &mut self.handle;
        configure_interrupt_endpoint(handle, endpoint);

        let mut buffer: [u8; WRITE_LENGTH] = [0x0; WRITE_LENGTH];
        buffer[0] = 0x30;
        buffer[1] = 0x04;
        buffer[3] = index;
        buffer[5] = asset;
        let write_result = handle.write_interrupt(endpoint.address, &buffer, timeout);
        match write_result {
            Ok(r) => r,
            Err(error) => panic!("error: {:?}", error)
        };
        let release_result = handle.release_interface(endpoint.iface);
        match release_result {
            Ok(r) => r,
            Err(error) => panic!("Failed to Release Interface: {:?}", error)
        };
        ();
    }

    pub fn send_delete(&mut self, index: u8) {
        let timeout = Duration::from_secs(2);
        let endpoint = &self.interrupt_write_endpoint;
        let handle = &mut self.handle;
        configure_interrupt_endpoint(handle, endpoint);

        let mut buffer: [u8; WRITE_LENGTH] = [0x0; WRITE_LENGTH];
        buffer[0] = 0x32;
        buffer[1] = 0x2;
        buffer[2] = index;
        let write_result = handle.write_interrupt(endpoint.address, &buffer, timeout);
        match write_result {
            Ok(r) => r,
            Err(error) => panic!("error: {:?}", error)
        };
        let release_result = handle.release_interface(endpoint.iface);
        match release_result {
            Ok(r) => r,
            Err(error) => panic!("Failed to Release Interface: {:?}", error)
        };
        ();
    }

    pub fn send_bulk_out(&self, buffer: &[u8]) -> bool {
        let endpoint = &self.bulk_endpoint;
        let timeout = Duration::from_secs(2);
        let chunks = kraken_driver_utils::slice_into_512(buffer);
        let header_buffer = <[u8; 512]>::from_hex("12fa01e8abcdef9876543210010000009cb01900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    
        let header_buffer_vec = header_buffer.to_vec();
    
        let header_write_result = self.handle.write_bulk(endpoint.address, &header_buffer_vec, timeout);
        match header_write_result {
            Ok(buf) => buf,
            Err(error) => {
                panic!("Problem sending header! {:?}", error);
            }
        };
        for chunk in chunks {
            // dbg!(chunk);
            let write_result = self.handle.write_bulk(1, &chunk, timeout);
            match write_result {
                Ok(buffer) => buffer,
                Err(error) => {
                    panic!("Problem writing! {:?}", error);
                }
            };
        }
    
        return true;
    
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

    pub fn _control_response() {

    }
    

}






