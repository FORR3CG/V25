fn main() {
    let mut listi = vec![1,2,3,4,5,6];
    listi.iter().for_each(|tala| println!("{}", tala));

    for tala in listi.iter() {
        println!("{}", tala);
    }

    let nyr_listi = listi
                                .iter()
                                .map(|tala| tala * 2)
                                .collect::<Vec<i32>>();
    println!("{:?}", nyr_listi);
    let mut nyr_listi2 = Vec::new();
    for tala in listi.iter() {
        nyr_listi2.push(tala);
    }

    listi
        .iter()
        .zip(nyr_listi)
        .for_each(|t| println!("{:?}", t));

    let k: i32 = listi.iter().sum();
    println!("{}", listi.iter().sum::<i32>());

    let k = listi.iter().fold(50, |summa, tala| summa * tala);

    let x = listi
                        .iter()
                        .filter(|tala| **tala > 3)
                        .collect::<Vec<&i32>>();

    println!("x: {:?}\nlisti: {:?}", x, listi);

    listi.retain(|tala| *tala > 3);
    println!("{:?}", listi);
    
    listi.iter_mut().for_each(|tala| *tala *= 2);
    println!("{:?}", listi);
    
    let mut listi = vec![3, 7, 2, 1];
    println!("{:?}", listi);
    listi.sort();
    println!("{:?}", listi);
}
