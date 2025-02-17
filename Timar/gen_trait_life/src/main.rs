use std::fmt::Display;

struct Punktur<T, U> { // T, U, V, W, 
    x: T,
    y: U,
}

impl<T, U> Punktur<T, U> {
    fn new(x: T, y: U) -> Self {
        Self {
            x, y
        }
    }
}

/* 
Compilerinn býr til: 
struct Punkur_f32 {
    x: f32,
    y: f32,
}

struct Punktur_i32 {
    x: i32,
    y: i32,
} */

fn prenta<T>(texti: T) where T: Display {
    println!("{}", texti)
}

struct Hundur<'a> { // má vera <'b> <'geir> 
    nafn: &'a str,
    hlydnieinkunn: u8,
}

impl<'a> Dyrahljod for Hundur<'a> {
    fn segir(&self) {
        println!("{} segir voff!", self.nafn)
    }
}

struct Kottur<'a> {
    nafn: &'a str,
    eigandi: &'a str,
}

impl<'a> Dyrahljod for Kottur<'a> {
    fn segir(&self) {
        println!("{} segir mjá!", self.nafn)
    }

    fn hallo(&self) {
        println!("Mjálló")
    }
}

trait Dyrahljod {
    // verður að útfæra
    fn segir(&self);

    // valkvæmt að útfæra
    fn hallo(&self) {
        println!("halló")
    }
}

fn lengri<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let a = "tskóli";
    let b = "abc";
    let q = lengri(a, b);
    let h = Hundur {nafn: "Snati", hlydnieinkunn: 8};
    let k = Kottur {nafn: "Grettir", eigandi: "Jón"};
    h.segir();
    k.segir();
    h.hallo();
    k.hallo();
    let j = "28".parse::<u8>().unwrap();
    prenta("Geir");
    prenta(25);
    let p = Punktur {x: 32f32, y: 29. };
    //prenta(p);
    let p2 = Punktur {x: 23, y: 34};
    println!("Hello, world!");
}
