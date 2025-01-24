use std::fmt::Display;

use crate::stada::Stada;
use crate::eigandi::{self, Eigandi};

pub struct Hestur {
    id: u32,
    nafn: String,
    aldur: u8,
    stada: Stada,
    eigandi: Option<Eigandi>,
}

impl Hestur {
    pub fn new(id: u32, nafn: &str, aldur: u8, stada: &str, eigandi: Option<Eigandi>) -> Self {
        Self {
            id,
            nafn: nafn.to_string(),
            aldur,
            stada: Stada::from(stada),
            eigandi,
        }
    }

    pub fn breyta_stodu(&mut self, stada: &str) {
        self.stada = Stada::from(stada)
    }

    pub fn skra_eiganda(&mut self, nafn: &str, email: &str) {
        self.eigandi = Some(Eigandi::new(nafn, email))
    }

    pub fn breyta_netfangi_eiganda(&mut self, email: &str) {
        if let Some(e) = &mut self.eigandi {
            e.breyta_netfangi(email);
        }
    }
}

impl Display for Hestur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // ef það er eigandi:
    // id, nafn, aldur stada, eigandi: nafn, email
    // ef það er enginn eigandi:
    // id, nafn, aldur, stada, Engin skráður eigandi
        match &self.eigandi {
            Some(e) => write!(
                f,
                "id: {}, nafn: {}, aldur: {}, staða: {}, eigandi: {}",
                self.id, self.nafn, self.aldur, self.stada, e
                ),
            None => write!(
                f, 
                "id: {}, nafn: {}, aldur: {}, staða: {}, enginn eigandi",
                self.id, self.nafn, self.aldur, self.stada
            ),
        }
    }
}