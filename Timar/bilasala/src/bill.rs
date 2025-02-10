use std::fmt::Display;

use crate::gerd::Gerd;

pub struct Bill {
    pub id: u32,
    framleidandi: String,
    pub gerd: Gerd,
    verd: u32,
}

impl Bill {
    pub fn new(id: u32, framleidandi: &str, gerdin: &str, verd: u32) -> Result<Self, String> {
        Ok(Self {
            id,
            framleidandi: framleidandi.to_string(),
            gerd: Gerd::try_from(gerdin)?, 
            verd,
        })
    }

    pub fn verd(&self) -> u32 {
        self.verd
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id: {}, frml.: {}, gerð: {}, verð: {}",
                   self.id, self.framleidandi, self.gerd, self.verd)
    }
}

