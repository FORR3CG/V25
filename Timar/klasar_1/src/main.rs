mod bill;
use bill::Bill;

fn main() {
    let mut volvo = Bill::new(12, "Volvo");
    volvo.prenta();
    volvo.set_id(99);
    volvo.prenta();
    //volvo.tegund = "Nissan".to_string();
    println!("Tegund: {}", volvo.tegund());
    let bmw = Bill::new(23,"BMW");
    //let toyota = bua_til_bil(34, "Toyota");
    println!("id: {}, tegund: {}", bmw.id(), bmw.tegund());
    println!("{:?}", bmw);
}

/* fn bua_til_bil(id: u32, tegund: &str) -> Bill {
    Bill {
        id,
        tegund: tegund.to_string(),
    }
} */
