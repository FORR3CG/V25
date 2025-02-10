/* enum Result<T, E> {
    Ok(T),
    Err(E),
} */


fn deila(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err("Deila: Ekki má deila með núll!".to_string())
    }
}

fn deiling(a: i32, b: i32) -> Result<i32, String> {
    // gerum fullt af hlutum
    if a > 10 {
        return Err("Deiling: a má ekki vera stærra en 10!".to_string());
    }
    let mut k = deila(a, b)?;
/*     let mut k = match  {
        Ok(t) => return t,
        Err(e) => return Err(e),
    }; */
    // gerum eitthvað meira
    k += 10;
    Ok(k)
}

fn main() {
    let a = 11;
    let b = 0;
    let c = deila(a, b);
    match c {
        Ok(tala) => println!("{} / {} = {}", a, b, tala),
        Err(villa) => println!("Villa: {}", villa),
        //Err(_) => panic!("Ekki má deila með núll!"),
    }

    if let Ok(d) = deila(a, b) {
        println!("{}/{} = {}", a, b, d);
    } else {
        println!("Villa kom upp!");
    }

    let f = deiling(a, b);
    match f {
        Ok(t) => println!("tala: {}", t),
        Err(e) => println!("{}", e),
    }

    println!("Hello, world!");
}
