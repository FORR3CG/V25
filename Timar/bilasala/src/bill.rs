use std::fmt::Display;

use crate::gerd::Gerd;

#[derive(Eq)]
pub struct Bill {
    pub id: u32,
    framleidandi: String,
    pub gerd: Gerd,
    pub verd: u32,
}

impl PartialEq for Bill {
    fn eq(&self, other: &Self) -> bool {
        self.framleidandi == other.framleidandi && self.gerd == other.gerd
    }
}

impl Ord for Bill {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.framleidandi == other.framleidandi {
            self.verd.cmp(&other.verd)
        } else {
            self.framleidandi.cmp(&other.framleidandi)
        }
    }
}

impl PartialOrd for Bill {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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

