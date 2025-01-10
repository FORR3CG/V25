// Æfingaverkefni
// Búið til 100 staka einvítt fylki (i32), frumstillt með -1 og fyllið það 
// svo með random tölum á bilinu i32::MIN til i32::MAX.
// Skrifið fylkið á skjáinn.
// Farið svo í gegnum fylkið og finnið minnstu og stærstu töluna.
// Leggið saman allar tölurnar og reiknið út meðaltal þeirra.
// Sýnið svo minnstu og stærstu töluna. Sýnið svo meðaltalið með tveimur 
// aukastöfum.
// Prentið að lokum fylkið út í öfugri röð og sýnið bara annaðhvert stakið.


fn main() {
    let f = [1_u16,2,3,4,5];
    let mut fylki = [0_i128; 5];
    let idx = 2;
    fylki[idx] = 24;
    for i in 0..5 {
        fylki[i] = 3 + i as i128;
    }
    for i in (0..5).rev().step_by(2) { // range(4, 0, -2)
        print!("{} - ", fylki[i]);
    }
    println!();
    let mut fylki_2d = [[0_i32; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            fylki_2d[i][j] = ((1 + i) * (1 + j)) as i32;
            if fylki_2d[i][j] > 50 {
                fylki_2d[i][j] = -fylki_2d[i][j];
            } else if fylki_2d[i][j] == 10 {
                fylki_2d[i][j] = 20;
            } else {
                fylki_2d[i][j] *= 2;
            }
        }
    }
    println!("{:?}", fylki_2d);
    println!("{:p}", &fylki);
    println!("{:p}", &fylki[0]);
    println!("{:p}", &fylki[1]);
    println!("{:p}", &fylki[2]);





}
