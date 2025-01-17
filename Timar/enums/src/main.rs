
#[derive(Debug)]
enum Dagar {
    Manudagur, // 0
    Thridjudagur, // 1
    Midvikudagur, // 2
    Fimmtudagur,
    Fostudagur,
    // o.s.frv
}

enum IPtala {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

impl IPtala {
    fn prenta(&self) {
        match self {
            IPtala::IPv4(a, b, c, d) 
                => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
            IPtala::IPv6(ip) => println!("IPv6: {}", ip),
        }
    }
}


fn main() {
    let lh = IPtala::IPv4(127, 0, 0, 1);
    let loopback = IPtala::IPv6("::1".to_string());
    lh.prenta();
    loopback.prenta();
    let dagur = Dagar::Manudagur;
    match dagur {
        Dagar::Manudagur => println!("vikan að byrja"),
        Dagar::Thridjudagur | Dagar::Midvikudagur => println!("vikan byrjaði í gær"),
        Dagar::Midvikudagur => println!("vikan hálfnuð"),
        _ => println!("Einhver annar dagur"),
    }

    println!("{:?}", dagur as isize);
    println!("Hello, world!");
}
