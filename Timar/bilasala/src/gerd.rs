use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Gerd {
    Folksbill,
    Jeppi,
}

impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gerd::Folksbill => write!(f, "Fólksbíll"),
            Gerd::Jeppi => write!(f, "Jeppi"),
        }
    }
}

impl TryFrom<&str> for Gerd {
    type Error = String;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "f" | "fb" => Ok(Self::Folksbill),
            "j" => Ok(Self::Jeppi),
            _ => Err(format!("Get ekki breytt {} í gerð!", value))
        }
    }
    
}