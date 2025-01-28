fn main() {
    let s1 = "hello world".to_string();
    println!("1.: {}", &s1[0..1]);
    println!("1. orðið: {}", &s1[0..5]);
    let s2 = "abc".to_string();
    let s3 = "def".to_string();
    //let s4 = s1 + &s2 + &s3;
    let s4 = format!("{} {} {}", &s1, &s2, &s3);
    println!("{}", s4);

    let s5 = "þæö".to_string();
    println!("{}", &s5[0..2]);
    println!("{}", s5.len());
    for s in s5.as_bytes() {
        println!("{}", s);
    }

    let em = "😊".to_string();
    println!("{}", em.len());
    for k in em.as_bytes() {
        println!("{}", k);
    }
    //println!("{}", &em[0..1]);

    let s6 = "😊 þæð hello".to_string();

    for stafur in s6.chars() {
        println!("{}", stafur);
    }
    let mut is6 = s6.chars();
    println!("{:?}", is6);
    println!("{:?}", is6.next());
    println!("{:?}", is6.next());
    println!("{:?}", is6.next());
    println!("{:?}", is6);

    for _ in 0..10 {
        println!("{:?}", is6.next());
    }
    println!("{:?}", is6);
    println!("{}", s6);

}
