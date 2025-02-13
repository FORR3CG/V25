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
    let mut hestar: Vec<Hestur> = Vec::new();
    hestar.push(Hestur {id: 99, nafn: "Sæla".to_string(), aldur: 14});
    hestar.push(Hestur {id: 23, nafn: "Blési".to_string(), aldur: 9});
    hestar.push(Hestur {id: 45, nafn: "Rauður".to_string(), aldur: 11});
    hestar.push(Hestur {id: 21, nafn: "Sæla".to_string(), aldur: 13});   
}
