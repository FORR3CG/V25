use std::fmt::Display;

pub enum Stada {
    Leigdur,
    Laus,
    EkkiTilLeigu,
}

impl Display for Stada {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stada = match self {
            Stada::Leigdur => "Leigður",
            Stada::Laus => "Laus",
            Stada::EkkiTilLeigu => "Ekki til útleigu",
        };
        write!(f, "{}", stada)
    }
}

impl TryFrom<&str> for Stada {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "leigður" => Ok(Stada::Leigdur),
            "laus" => Ok(Stada::Laus),
            "ekki" | "ekki til leigu" => Ok(Stada::EkkiTilLeigu),
            _ => Err(format!("Stada: Gat ekki breytt {} í stöðu!", value)),
        }
    }
}
