use std::fmt::Display;

#[derive(Debug)]
pub enum Gerd {
    Folksbill,
    Jeppi,
    Annad,
}

impl Gerd {
    pub fn new(gerd: &str) -> Self {
        match gerd.to_lowercase().trim() {
            "fb" | "fólksbíll" | "folksbill" => Self::Folksbill,
            "jepp" | "j" => Self::Jeppi,
            _ => Self::Annad,
        }
    }
}

impl From<&str> for Gerd {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "fb" | "fólksbíll" => Self::Folksbill,
            "jeppi" => Self::Jeppi,
            _ => Self::Annad,
        }
    }
}

impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gerd = match self {
            Gerd::Folksbill => "Fólksbíll",
            Gerd::Jeppi => "Jeppi",
            Gerd::Annad => "Annað",
        };
        write!(f, "gerð: {}", gerd)
    }
}