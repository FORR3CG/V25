use std::fmt::Display;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

trait Dyrin {
    fn segir(&self);
}

#[derive(Deserialize, Serialize)]
struct Hundur {
    nafn: String,
    hlydnieinkunn: u32,
}

impl Dyrin for Hundur {
    fn segir(&self) {
        println!("{} segir voff!", self.nafn)
    }
}

impl Hundur {
    fn new(nafn: &str, hlydnieinkunn: u32) -> Self {
        Self {
            nafn: nafn.to_string(),
            hlydnieinkunn,
        }
    }
}

impl Display for Hundur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nafn: {}, hlýðnieinkunn {}", self.nafn, self.hlydnieinkunn)
    }
}

#[derive(Deserialize, Serialize)]
struct Kottur {
    nafn: String,
    aldur: u8,
}

impl Dyrin for Kottur {
    fn segir(&self) {
        println!("{} segir mjá!", self.nafn)
    }
}

impl Kottur {
    fn new(nafn: &str, aldur: u8) -> Self {
        Self {
            nafn: nafn.to_string(),
            aldur,
        }
    }
}

impl Display for Kottur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nafn: {}, aldur {}", self.nafn, self.aldur)
    }
}

#[derive(Deserialize, Serialize)]
enum Dyr {
    Kotturinn(Kottur),
    Hundurinn(Hundur),
}

impl Display for Dyr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dyr::Kotturinn(kottur) => write!(f, "{}", kottur),
            Dyr::Hundurinn(hundur) => write!(f, "{}", hundur),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Dyragardur {
    dyrin: Vec<Dyr>,
}

impl Dyragardur {
    fn new() -> Self {
        Self {
            dyrin: Vec::new(),
        }
    }

    fn skra_dyr(&mut self, dyr: Dyr) {
        self.dyrin.push(dyr);
    }

    fn skra_hund(&mut self, hundur: Hundur) {
        self.dyrin.push(Dyr::Hundurinn(hundur));
    }

    fn skra_kott(&mut self, kottur: Kottur) {
        self.dyrin.push(Dyr::Kotturinn(kottur));
    }

    fn prenta_hunda(&self) {
        self.dyrin.iter().for_each(|d| {
            match d {
                Dyr::Kotturinn(kottur) => (),
                Dyr::Hundurinn(hundur) => println!("{}", hundur),
            }
        });
    }
}

// fyrir json: keyra: cargo add serde -F derive
//                    cargo add serde_json

fn main() {
    let mut dg = Dyragardur::new();
    dg.skra_hund(Hundur::new("Snati", 9));
    dg.skra_kott(Kottur::new("Grettir", 20));

    // búa til json og skrifa í skrá
/*     let json_gogn = serde_json::to_string_pretty(&dg).unwrap(); // 
    println!("{}", json_gogn);
    let mut skra = std::fs::File::create("gogn.json").unwrap();
    write!(skra, "{}", json_gogn); */

    // lesa json skrá og búa til dýragarð
    let mut skra = std::fs::File::open("gogn.json").unwrap();
    let mut fra_skra = String::new();
    skra.read_to_string(&mut fra_skra).unwrap();
    println!("{}", fra_skra);
    let mut dg2 = serde_json::from_str::<Dyragardur>(&fra_skra).unwrap();
    dg2.prenta_hunda();



/*     let mut dyr: Vec<Dyr> = Vec::new();
    dyr.push(Dyr::Hundurinn(Hundur::new("Snati", 9)));
    dyr.push(Dyr::Kotturinn(Kottur::new("Grettir", 20)));

    dyr.iter().for_each(|d| {
        match d {
            Dyr::Kotturinn(kottur) => println!("{}", kottur),
            Dyr::Hundurinn(hundur) => println!("{}", hundur),
        }
    }); */

    // usize
    //let dyrin = vec![Hundur::new("Snati", 8), Kottur::new("Grettir", 20)];
/*     let mut dyrin: Vec<Box<dyn Dyrin>> = Vec::new();

    dyrin.push(Box::new(Hundur::new("Snati", 7)));
    dyrin.push(Box::new(Kottur::new("Grettir", 20)));
    dyrin.iter().for_each(|d| d.segir()); */


}
