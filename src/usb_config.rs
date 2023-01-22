use rusb::{UsbContext, Device, DeviceHandle, Context};

#[derive(Debug)]
pub struct Endpoint {
    pub config: u8,
    pub iface: u8,
    pub setting: u8,
    pub address: u8,
}

// returns all readable endpoints for given usb device and descriptor
pub fn find_writable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Vec<Endpoint> {
    let device_desc = device.device_descriptor().unwrap();
    let mut endpoints = vec![];
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // println!("{:#?}", config_desc);
        for interface in config_desc.interfaces() {
            // println!("{:#?}", interface.number());
            for interface_desc in interface.descriptors() {
                // println!("{:#?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // println!("{:#?}", endpoint_desc);
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface.number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    });
                }
            }
        }
    }

    endpoints
}

pub fn configure_bulk_endpoint(
    handle: &mut DeviceHandle<Context>,
    endpoint: &Endpoint,
) {
    handle.set_active_configuration(endpoint.config).unwrap();

    match handle.claim_interface(endpoint.iface) {
        Ok(i) => i,
        Err(error) => panic!("error: {:?}", error)
    };
    handle.set_alternate_setting(endpoint.iface, endpoint.setting).unwrap();
}

pub fn configure_interrupt_endpoint(
    handle: &mut DeviceHandle<Context>,
    endpoint: &Endpoint
) {
    handle.set_active_configuration(endpoint.config).unwrap();

    match handle.claim_interface(endpoint.iface) {
        Ok(i) => i,
        Err(error) => panic!("error: {:?}", error)
    };
    handle.set_alternate_setting(endpoint.iface, endpoint.setting).unwrap();
}