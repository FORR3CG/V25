use std::fmt::Display;

#[derive(Debug)]
pub struct Litur(u8, u8, u8);

impl Litur {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "litur: r: {}, g: {}, b: {}", self.0, self.1, self.2)
    }
}