
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
    pub fn set_image(mut self, image: Vec<u8>) {
       self.driver.send_query(1, 0);
       self.driver.send_delete(1);
       
    }

    pub fn set_blank(mut self) {
        self.driver.send_switch(0, 0);
    }
    
}