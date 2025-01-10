// Æfingaverkefni 1
// Búið til 100 staka einvítt fylki (i32), frumstillt með -1 og fyllið það 
// svo með random tölum á bilinu i32::MIN til i32::MAX.
// Skrifið fylkið á skjáinn.
// Farið svo í gegnum fylkið og finnið minnstu og stærstu töluna.
// Leggið saman allar tölurnar og reiknið út meðaltal þeirra.
// Sýnið svo minnstu og stærstu töluna. Sýnið svo meðaltalið með tveimur 
// aukastöfum.
// Prentið að lokum fylkið út í öfugri röð og sýnið bara annaðhvert stakið.s

use std::i32;

use rand::Rng;

fn main() {
    // Búið til 100 staka einvítt fylki (i32), frumstillt með -1 og fyllið það 
    // svo með random tölum á bilinu i32::MIN til i32::MAX.
    let mut fylki = [-1; 100];
    for i in 0..100 {
         fylki[i] = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);
    }
    // Skrifið fylkið á skjáinn.
    println!("{:?}", fylki);
    // Farið svo í gegnum fylkið og finnið minnstu og stærstu töluna.
    // Leggið saman allar tölurnar og reiknið út meðaltal þeirra.
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut summa: i64 = 0;
    for i in 0..100 {
        summa += fylki[i] as i64;
        if fylki[i] < min {
            min = fylki[i];
        }
        if fylki[i] > max {
            max = fylki[i];
        }
    }
    // Sýnið svo minnstu og stærstu töluna. Sýnið svo meðaltalið með tveimur 
    // aukastöfum.
    println!("Lægsta: {}\nStærsta: {}\nMeðaltal: {:?}", min, max, summa as f64 / 100.)
}
