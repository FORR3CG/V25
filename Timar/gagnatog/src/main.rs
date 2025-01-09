use core::f64;

const AR: u16 = 2025;

fn main() {
    let x: i8 = 10;
    let ar_eitt = AR;
    let mut y = 10_u8;
    y = y + x as u8;
    let a = 0b1000001;
    let b = 0xABC_u64;
    let c = a as f64 / b as f64; // a // b
    println!("c: {:.4}", c);
    println!("b: {:b}, o: {:o}, x: {:x}", a, a, a);
    let sjalgefna_tagid = 32;
    println!("min: {}, max: {}", i32::MIN, i32::MAX);
    println!("min: {}, max: {}", u32::MIN, u32::MAX);
    // signed: i8, i16, i32, i64, i128, isize
    // unsigned: u8, u16, u32, u64, u128, usize

    // kommutölur, float: f32, f64
    let k = 1.23;
    let f: f32 = 1.24;
    let ff = 1.23_f32;
    let pi = f64::consts::PI;
    println!("Hello, world!");

    // char
    let stafur = 'A';
    for i in 0..26 { // for i in range(26)
        print!("{}", (stafur as u8 + i) as char);
    }
    
    let t = true;
    let f = false;
    println!();

    let t = (12, b'A', 3.14);
    let mut tt: (u8, usize, f32) = (8, 9, 1.123);
    tt.0 = 99;
    let (_, mut b, _) = tt;

    let mut fylki = [1_u8,2,3,4,5];
    fylki[0] = 99;
    println!("{:#?}", fylki)

}
