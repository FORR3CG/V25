use std::fmt::Display;

use crate::gerd::Gerd;

pub struct Bill {
    id: u32,
    framleidandi: String,
    gerd: Gerd,
    verd: u32,
}

impl Bill {
    pub fn new(id: u32, framleidandi: &str, gerdin: &str, verd: u32) -> Self {
        Self {
            id,
            framleidandi: framleidandi.to_string(),
            gerd: Gerd::from(gerdin), 
            verd,
        }
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, frml.: {}, gerð: {}, verð: {}",
                   self.id, self.framleidandi, self.gerd, self.verd)
    }
}

