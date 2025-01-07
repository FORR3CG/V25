use rand::Rng;
use std::io::{stdin, stdout, Write};

fn main() {
    let r_tala = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Ég hugsa mér tölu á milli 1 og 100, giskaðu hver hún er: ");
        stdout().flush().unwrap();
        let mut inntak = String::new();
        
        stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá lyklaborði, enda forrit!!");
        // let gisk: i32 = inntak.trim().parse().expect("Gat ekki breytt í tölu!!");
        let gisk = inntak
            .trim()
            .parse::<i32>()
            .expect("Gat ekki breytt í tölu!!");

        match gisk.cmp(&r_tala) {
            std::cmp::Ordering::Less => println!("Of lágt!"),
            std::cmp::Ordering::Equal => {
                println!("Þú giskaðir á rétta tölu!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Of hátt!"),
        }

        /*
        if gisk > r_tala {
            println!("Of hátt!");
        } else if gisk < r_tala {
            println!("Of lágt!");
        } else {
            println!("Þú giskaðir á rétta tölu!!!");
        }
        */
    }
}
