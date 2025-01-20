/*
    Bill: id, tegund (toyota, bwm), gerd (Jeppi, Folksbill, Annad), 
          Litur(r, g, b), Orkugjafi (bensin, dísel, rafmagn, annað)

*/

use std::fmt::Display;
use crate::litur::Litur;
use crate::gerd::Gerd;
use crate::orkugjafi::Orkugjafi;


#[derive(Debug)]
pub struct Bill {
    pub id: u32,
    pub tegund: String,
    pub gerd: Gerd,
    pub litur: Litur,
    pub orkugjafi: Orkugjafi,
}

impl Bill {
    pub fn new(id: u32, tegund: &str, gerd: &str, r: u8, g: u8, b: u8, orka: &str) -> Self {
        Self {
            id,
            tegund: tegund.to_string(),
            gerd: Gerd::new(gerd),
            litur: Litur::new(r, g, b),
            orkugjafi: Orkugjafi::new(orka),
        }
    }

    pub fn breyta_orkugjafa(&mut self, nyr_orkugjafi: &str) {
        self.orkugjafi = Orkugjafi::new(nyr_orkugjafi)
    }

    pub fn breyta_gerd(&mut self, gerd: Gerd) {
        self.gerd = gerd
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, teg: {}, {}, {}, orka: {}", 
            self.id, self.tegund, self.gerd, self.litur, self.orkugjafi)
    }
}
