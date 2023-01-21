
use crate::kraken_driver::KrakenDriver;

pub struct KrakenController {
    driver: KrakenDriver
}
impl KrakenController {
    pub fn new() -> Self {

        let driver = KrakenDriver::new();
        // Do something with the driver (set gif?)
        KrakenController {
            driver
        }
    }
    pub fn set_image(&self, image: Vec<u8>) {
        self.driver.send_bulk_out(&image);
    }
    
}