mod bill;
use bill::Bill;

// tuple struct
#[derive(Debug)]
struct Punktur(i32, i32);

impl Punktur {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    fn prenta(&self) {
        println!("x: {}, y: {}", self.0, self.1)
    }
}

// unit struct, á engin gögn
struct Reikniadgerdir;

impl Reikniadgerdir {
    fn leggja_saman(a: i32, b: i32) -> i32 {
        a + b
    }
}

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
