use std::fmt::Display;

#[derive(Debug, Eq)]
struct Hestur {
    id: u32,
    nafn: String,
    aldur: u8,
}

impl PartialEq for Hestur {
    fn eq(&self, other: &Self) -> bool {
        self.nafn == other.nafn && self.aldur == other.aldur
    }
}

impl Ord for Hestur {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.nafn == other.nafn {
            self.aldur.cmp(&other.aldur)
        } else  {
            self.nafn.cmp(&other.nafn)
        }
    }
}

impl PartialOrd for Hestur {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
} 

impl Display for Hestur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, nafn: {}, aldur: {}", self.id, self.nafn, self.aldur)
    }
}

fn prenta_hesta(hestar: &[Hestur]) {
    hestar.iter().for_each(|hestur| println!("{}", hestur))
}

fn main() {
    let mut hestar: Vec<Hestur> = Vec::new();
    hestar.push(Hestur {id: 99, nafn: "Sæla".to_string(), aldur: 14});
    hestar.push(Hestur {id: 23, nafn: "Blési".to_string(), aldur: 9});
    hestar.push(Hestur {id: 45, nafn: "Rauður".to_string(), aldur: 11});
    hestar.push(Hestur {id: 21, nafn: "Sæla".to_string(), aldur: 13});  

    prenta_hesta(&hestar);
    hestar.sort();
    println!("----------------------------------");
    prenta_hesta(&hestar);
    
    println!("----------------------------------");
    println!("{:?}", 3.cmp(&2)); 
    println!("{:?}", 3.cmp(&3));
    println!("{:?}", 3.cmp(&4));
}

/* enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
} */
