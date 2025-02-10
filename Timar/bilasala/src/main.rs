mod bill;
mod gerd;
mod bilasala;

use std::io::Write;

use bilasala::Bilasala;
use gerd::Gerd;

fn main() {
    let g = Gerd::try_from("fb");
    let mut bs = Bilasala::new(1000);
    bs.skra("BMW", "f", 1_250_000);
    bs.skra("Toyota", "j", 12_345_673);
    bs.skra("Nissan", "veit ekki", 650);
    println!("{}", bs);

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
                   "skrá" => {
                        if skipanir.len() != 4 {
                            println!("Ekki réttur fjöldi orða til að búa til bíl!");
                            continue;
                        }
                        let framleidandi = skipanir[1];
                        let gerd = skipanir[2];
                        if let Ok(verd) = skipanir[3].parse::<u32>() {
                            match bs.skra(framleidandi, gerd, verd) {
                                Ok(_) => println!("Skráði nýjan bíl"),
                                Err(e) => println!("{}", e),
                            }
                            
                        } else {
                            println!("Gat ekki breytt {} í tölu", skipanir[3]);
                        }
                   },
                   "afskrá" | "selja" => {
                        if let Some(id) = skipanir.get(1) {
                            if let Ok(id) = id.trim().parse::<u32>() {
                                bs.afskra(id);
                            } else {
                                println!("Gat ekki breytt {} í tölu", skipanir[1]);
                                continue;
                            }
                        } else {
                            println!("Þú verður að gefa upp id!");
                            continue;
                        }
                   },
                   "prenta" => {
                        if skipanir.len() > 1 {
                            match skipanir[1].to_lowercase().trim() {
                                "allt" => println!("{}", bs),
                                "gerð" => {
                                    if let Some(gerd) = skipanir.get(2) {
                                        match bs.prenta_gerd(gerd) {
                                            Ok(_) => (),
                                            Err(e) => println!("{}", e),
                                        }
                                    } else {
                                        println!("Þú verður að segja hvaða gerð þú villt!");
                                    }
                                },
                                "bíl" => {
                                    // prenta bíl 1001
                                    if let Some(id) = skipanir.get(2) {
                                        if let Ok(id) = id.parse::<u32>() {
                                            bs.prenta_bil(id);
                                        } else {
                                            println!("Gat ekki breytt {} í tölu!", skipanir[2]);
                                        }
                                    } else {
                                        println!("Þú verður að segja mér id-ið á bílnum!")                                        
                                    }

                                },
                                _ => println!("Get ekki prentað {}", skipanir[1]),
                            }
                        } else {
                            println!("Þú verður að segja hvað þú villt prenta!");
                        }
                   },
                   _ => println!("Skil ekki skipunina!"),
                }
            },
            None => println!("Þú verður að slá eitthvað inn!"),
        }
    }
}
