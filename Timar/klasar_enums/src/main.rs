mod bill;
mod gerd;
mod litur;
mod orkugjafi;

use bill::Bill;
use gerd::Gerd;
use litur::Litur;
use orkugjafi::Orkugjafi;

fn main() {
    let toyota = Bill {
        id: 23,
        tegund: "Toyota".to_string(),
        gerd: Gerd::Folksbill,
        litur: Litur::new(255, 0, 0),
        orkugjafi: Orkugjafi::Bensin,
    };
    let mut bmw = Bill::new(25, "BMW", 
                "fb", 0, 0, 255, "bensín");
    bmw.breyta_gerd(Gerd::Jeppi);
    bmw.breyta_orkugjafa("rafmagn");
    println!("{}", bmw);
    println!("{}", toyota);
    let og_1 = Orkugjafi::from("b");
    let og_2 = Orkugjafi::from(3);
}
