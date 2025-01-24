use core::net;
use std::fmt::Display;

#[derive(Debug)]
pub struct Eigandi {
    nafn: String,
    netfang: String,
}

impl Eigandi {
    pub fn new(nafn: &str, netfang: &str) -> Self {
        Self {
            nafn: nafn.to_string(),
            netfang: netfang.to_string(),
        }
    }

    pub fn breyta_netfangi(&mut self, netfang: &str) {
        self.netfang = netfang.to_string()
    }
}

impl Display for Eigandi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nafn: {}, email: {}", self.nafn, self.netfang)
    }
}