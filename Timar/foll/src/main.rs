// expression = segð
// statement = setning
fn main() {
    hallo();
    let tupla = fall_sem_skilar_morgum_gildum();
    println!("{}", tupla.0);
    let (a, mut b, c) = fall_sem_skilar_morgum_gildum();
    let mut t = 10;
    println!("heimilisfang t: {:p}", &t);
    println!("t fyrir: {}", t);
    haekka_um_einn(&mut t);
    println!("t eftir: {}", t);
    prenta_ut_fra_ref(&t);
    let fylki = [1,2,3,4,5];
    let mut nafn_str = "Geir";
    let mut nafn_string = "Geir".to_string();
    prenta_nafn(&nafn_str);
    prenta_nafn(&nafn_string);
}

fn prenta_nafn(nafn: &str) {
    println!("Halló {nafn}!");
}

fn prenta_ut_fra_ref(tala: &i32) {
    println!("tala: {}", tala)
}

fn haekka_um_einn(tala: &mut i32) {
    println!("heimilisfang tala: {:p}", tala);
    *tala = *tala + 1;
}

fn fall_sem_skilar_morgum_gildum() -> (i32, f32, u8) {
    (-7, 3.1, 8)
}

fn leggja_saman(a: u32, b: u32) -> u32 {
    let c = a + b;
    //return c;
    c
    // a + b
}

fn sjo_ef_haerri_en_fimm(a: i32) -> i32 {
    if a > 5 {
        7
    } else {
        a
    }
}

fn fimm() -> u32 {
    5
}

fn fall_sem_a_breytur() {
    let a = 10;
    let b = 20;
}

// snake_case, SCREAMING_SNAKE_CASE

fn hallo() {
    println!("halló");
    let f = fimm();
    println!("fallið fimm skilar: {}", fimm());
}
