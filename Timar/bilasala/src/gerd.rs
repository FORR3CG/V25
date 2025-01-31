use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Gerd {
    Folksbill,
    Jeppi,
    Annad,
}

impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gerd::Folksbill => write!(f, "Fólksbíll"),
            Gerd::Jeppi => write!(f, "Jeppi"),
            Gerd::Annad => write!(f, "Annað"),
        }
    }
}

impl From<&str> for Gerd {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "f" | "fb" | "fólksbíll" => Self::Folksbill,
            "j" => Self::Jeppi,
            _ => Self::Annad
        }
    }
}