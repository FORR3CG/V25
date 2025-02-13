use std::fmt::Display;

#[derive(Debug)]
struct Hestur {
    id: u32,
    nafn: String,
    aldur: u8,
}

impl Display for Hestur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, nafn: {}, aldur: {}", self.id, self.nafn, self.aldur)
    }
}

fn main() {
    
}
