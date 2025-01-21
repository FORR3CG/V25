use std::fmt::Display;

#[derive(Debug)]
pub enum Orkugjafi {
    Bensin,
    Disel,
    Rafmagn,
    Annad,
}

impl Orkugjafi {
    pub fn new(orka: &str) -> Self {
        match orka.to_lowercase().trim() {
            "b" | "bensín" => Self::Bensin,
            "d" | "dísel" => Self::Disel,
            "r" | "rafmagn" => Self::Rafmagn,
            _ => Self::Annad,
        }
    }
}

impl From<u8> for Orkugjafi {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Bensin,
            2 => Self::Rafmagn,
            3 => Self::Disel,
            _ => Self::Annad,
        }
    }
}

impl From<&str> for Orkugjafi {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "b" | "bensín" => Self::Bensin,
            "d" | "dísel" => Self::Disel,
            "r" | "rafmagn" => Self::Rafmagn,
            _ => Self::Annad,
        }  
    }
}

impl Display for Orkugjafi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orkugjafi::Bensin => write!(f, "Bensín"),
            Orkugjafi::Disel => write!(f, "Dísel"),
            Orkugjafi::Rafmagn => write!(f, "Rafmagn"),
            Orkugjafi::Annad => write!(f, "Annað"),
        }
    }
}