use std::fmt::Display;

fn main() {
    let k = 10;
    let f = 3.1234; 
    let k = "Geir";
    let mut j = 20u8;
    let i: u8 = 10;
    let a = 999;
    let b = 1000;
    let c = a as f32 / b as f32;

    let f = 3.1234;
    let e: f32 = 3.1234;
    let g = 3.1234f32;

    let stafur = b'a';
    let texti = "a";

    let t = (1, 3.14, 'a');
    let k = t.0;
    let (x, y, z) = t;

    let a = [1,2,3,4,5];
    dbg!(a[4]);
    let f = [0u8; 5];
    let f2 = [[0; 5]; 6];

    for i in (0..=4).rev().step_by(2) {
        print!("{}", f[i]);
    }

    let mut i = 5;
    while i > 0 {
        println!("{}", i);
        i -= 1;
    }
    /* 
    loop { // while True:
        loop {
            break;
        }
        break;
    }
    */
    let k = 10;
    let j = 9;
    if k == 10 && j != 10 || j == k {
        // gera eitthvað
    } else if k < 10 {
        // gera eitthvað
    } else {
        // gera eitthvað
    }

    let b = if k == 10 { 9 } else { 8 };

    match k {
        100 => print!("gerum eitthvað"),
        0..=99 => {
            print!("gerum eitthvað");
            print!("gerum eitthvað meira");
        },
        _ => print!("gerum eitthvað"),
    }

    let j = match k {
        0..100 | 101 => 99,
        _ => 88,
    };

    let k = 10;
    let j = k;
    println!("{}", k);

    let mut s1 = "Tskóli".to_string();
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;
    s1 = abc(s1);
    prenta(&s1);
    println!("{}", s1);

    let mut k = Box::new(10);
    *k += 1;
}

struct StaerdarVilla;

struct Litur(u8, u8, u8);

struct Staerd {
    lengd: u32,
    breidd: u32,
}

impl Staerd {
    fn new(lengd: u32, b: u32) -> Self {
        Self {
            lengd,
            breidd: b,
        }
    }

    fn fm(&self) -> u64 {
        self.lengd as u64 * self.breidd as u64
    }

    fn ny_breidd(&mut self, b: u32) {
        self.breidd = b
    }
}

impl Display for Staerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.breidd, self.lengd)
    }
}

enum Cpu {
    Risc,
    Cisc,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cpu::Cisc => write!(f, "Cisc örgjörvi!"),
            Cpu::Risc => write!(f, "Risc örgjörvi!"),
        }
    }
}

impl From<&str> for Cpu {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "cisc" | "c" => Self::Cisc,
            _ => Self::Risc,
        }
    }
}


fn prenta(texti: &String) {
    println!("{}", texti)
}

fn abc(texti: String) -> String {
    println!("{}", texti);
    texti
}

fn margfalda(a: u8, b: u8) -> u16 {
    a as u16 * b as u16
}
