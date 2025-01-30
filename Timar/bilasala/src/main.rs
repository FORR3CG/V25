mod bill;
mod gerd;
mod bilasala;

use bilasala::Bilasala;

fn main() {
    let mut bs = Bilasala::new(1000);
    bs.skra("BMW", "f", 1_250_000);
    bs.skra("Toyota", "j", 12_345_673);
    bs.skra("Nissan", "veit ekki", 650);
    println!("{}", bs)
}
