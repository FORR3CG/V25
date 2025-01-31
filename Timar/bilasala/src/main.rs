mod bill;
mod gerd;
mod bilasala;

use std::io::Write;

use bilasala::Bilasala;

fn main() {
    let mut bs = Bilasala::new(1000);
    bs.skra("BMW", "f", 1_250_000);
    bs.skra("Toyota", "j", 12_345_673);
    bs.skra("Nissan", "veit ekki", 650);
    println!("{}", bs)

    // hætta
    // hjálp
    // "skrá" "bmw" "f" "200000"
    // afskrá 1001
    // prenta allt
    // prenta gerd j
    // prenta bíl 1001

    loop {
        print!("Sláðu inn skipun: ");
        std::io::stdout().flush().expect("Gat ekki flush-að!");
        let mut inntak = String::new();
        std::io::stdin().read_line(&mut inntak).expect("Gat ekki lesið frá notanda!");
        let skipanir: Vec<&str> = inntak.split_whitespace().collect(); 
        match skipanir.first() {
            Some(skipun) => {
                match skipun.to_lowercase().trim() {
                   "hætta" => break, 
                   "hjálp" | "h" => println!("Prentum út hjálpina."),
                   "skrá" => 
                }
            },
            None => println!("Þú verður að slá eitthvað inn!"),
        }
    }
}
