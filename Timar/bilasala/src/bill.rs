use std::fmt::Display;

use crate::gerd::Gerd;

#[derive(Eq)]
pub struct Bill {
    pub id: u32,
    framleidandi: String,
    pub gerd: Gerd,
    pub verd: u32,
}

impl Bill {
    pub fn str_to_u32(ord: &str) -> Result<u32, String> {
        if let Ok(t) = ord.parse() {
            Ok(t)
        } else {
            Err(format!("Gat ekki breytt {} í tölu!", ord))
        }
    }
}

// "1005 bmw f 200000"
impl TryFrom<&str> for Bill {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        // "1005", "bmw", "f", "200000"
        if lidir.len() != 4 {
            return Err("Ekki réttur fjöldi orða!!".to_string());
        }
        let id = Bill::str_to_u32(lidir[0])?;
        let gerd = Gerd::try_from(lidir[2])?;
        let verd = Bill::str_to_u32(lidir[3])?;
        Ok(Self {
            id,
            framleidandi: lidir[1].to_string(),
            gerd,
            verd,
        })
    }
}

impl TryFrom<(u32, &str)> for Bill {
    type Error = String;

    fn try_from(value: (u32, &str)) -> Result<Self, Self::Error> {
        let lidir = value.1.split_whitespace().collect::<Vec<&str>>();
        // "bmw", "f", "200000"
        if lidir.len() != 3 {
            Err("Ekki réttur fjöldi orða!!".to_string())
        } else {
            Ok(Self {
                id: value.0,
                framleidandi: lidir[0].to_string(),
                gerd: Gerd::try_from(lidir[1])?,
                verd: Bill::str_to_u32(lidir[2])?,
            })
        }
    }
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
        write!(
            f,
            "id: {}, frml.: {}, gerð: {}, verð: {}",
            self.id, self.framleidandi, self.gerd, self.verd
        )
    }
}
