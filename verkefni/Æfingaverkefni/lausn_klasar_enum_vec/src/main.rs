mod hestaleiga;
mod hestur;
mod stada;

use hestaleiga::Hestaleiga;
use std::io::Write;

fn main() {
    // prenta
    // skrá sleipnir 10 laus
    // leigja 10
    // afleigja 10
    // hætta
    let mut hl = Hestaleiga::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Gat ekki flush-að!");
        let mut inntak = String::new();
        std::io::stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá notanda!");
        let skipanir: Vec<&str> = inntak.split_whitespace().collect();
        if let Some(skipun) = skipanir.first() {
            match *skipun {
                "prenta" => println!("{}", hl),
                "skrá" => {
                    if skipanir.len() < 4 {
                        println!("Ekki réttur fjöldi orða til að skrá hest!");
                        continue;
                    }
                    let aldur = match skipanir[2].trim().parse::<u8>() {
                        Ok(a) => a,
                        Err(_) => {
                            println!("Gat ekki breytt {} í aldur!", skipanir[2]);
                            continue;
                        }
                    };
                    let stada = skipanir[3..].join(" ");
                    if let Err(villa) = hl.skra_hest(skipanir[1], aldur, stada.as_str()) {
                        println!("{}", villa)
                    }
                }
                "leigja" | "afleigja" | "skila" => {
                    if skipanir.len() == 2 {
                        if let Ok(id) = skipanir[1].trim().parse::<u32>() {
                            if let Err(villa) = hl.leigja_hest(id) {
                                println!("{}", villa);
                                continue;
                            }
                            if *skipun == "leigja" {
                                if let Err(villa) = hl.leigja_hest(id) {
                                    println!("{}", villa)
                                }
                            } else if let Err(villa) = hl.afleigja_hest(id) {
                                println!("{}", villa)
                            }
                        }
                    } else {
                        println!("Vantar id!")
                    }
                }
                "hætta" => {
                    println!("Takk fyrir!");
                    break;
                }
                _ => println!("Þekki ekki skipunina {}, reyndu aftur!", skipun),
            }
        } else {
            println!("Ekkert slegið inn, reyndu aftur!")
        }
    }
}
