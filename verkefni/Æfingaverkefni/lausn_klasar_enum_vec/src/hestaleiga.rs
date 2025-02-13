use std::fmt::Display;

use crate::hestur::Hestur;

pub struct Hestaleiga {
    hestar: Vec<Hestur>,
    id: u32,
}

impl Hestaleiga {
    pub fn new() -> Self {
        Self {
            hestar: Vec::new(),
            id: 1000,
        }
    }

    fn next_id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    pub fn skra_hest(&mut self, nafn: &str, aldur: u8, stada: &str) -> Result<(), String> {
        let id = self.next_id();
        Ok(self.hestar.push(Hestur::new(id, nafn, aldur, stada)?))
    }

    fn breyta_stodu(&mut self, id: u32, ny_stada: &str) -> Result<(), String> {
        for hestur in &mut self.hestar {
            if hestur.id == id {
                hestur.breyta_stodu(ny_stada)?;
                return Ok(());
            }
        }
        Err(format!(
            "Hestaleiga: breyta_stodu: Fann engan hest með id: {}",
            id
        ))
    }

    pub fn leigja_hest(&mut self, id: u32) -> Result<(), String> {
        self.breyta_stodu(id, "leigður")
    }

    pub fn afleigja_hest(&mut self, id: u32) -> Result<(), String> {
        self.breyta_stodu(id, "laus")
    }
}

impl Display for Hestaleiga {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut texti = String::new();
        for hestur in &self.hestar {
            texti.push_str(format!("{}\n", hestur).as_str());
        }
        // EÐA
        // self.hestar.iter().for_each(|hestur| texti.push_str(format!("{}\n", hestur).as_str()));
        write!(f, "{}", texti)
    }
}
