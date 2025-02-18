use std::fmt::Display;

trait Dyrin {
    fn segir(&self);
}
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

fn main() {
    let mut dyr: Vec<Dyr> = Vec::new();
    dyr.push(Dyr::Hundurinn(Hundur::new("Snati", 9)));
    dyr.push(Dyr::Kotturinn(Kottur::new("Grettir", 20)));

    dyr.iter().for_each(|d| {
        match d {
            Dyr::Kotturinn(kottur) => println!("{}", kottur),
            Dyr::Hundurinn(hundur) => println!("{}", hundur),
        }
    });

    // usize
    //let dyrin = vec![Hundur::new("Snati", 8), Kottur::new("Grettir", 20)];
    let mut dyrin: Vec<Box<dyn Dyrin>> = Vec::new();

    dyrin.push(Box::new(Hundur::new("Snati", 7)));
    dyrin.push(Box::new(Kottur::new("Grettir", 20)));
    dyrin.iter().for_each(|d| d.segir());


}
