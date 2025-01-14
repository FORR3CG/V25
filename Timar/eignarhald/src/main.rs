use std::io::Write;

fn main() {
    // Fyrir skilaverkefni
    println!("Sláðu inn tölu: ");
    std::io::stdout().flush().expect("Gat ekki flush-að");
    let mut inntak = String::new();
    std::io::stdin().read_line(&mut inntak).expect("Gat ekki lesið!");
    let inntak = inntak.trim();
    if inntak == "1" {
        // gera eitthvað
    }


/*  
    let mut fylki = Box::new([0_i128; 1_000_000]);
    let tala_a_heap = Box::new(10);
    let t1 = 10;
    let t2 = t1; // copy, let t2 = 10
    println!("{}", t1);
*/ 
    let mut s1 = "Hallo".to_string();
    let mut s2 = &mut s1;
    s2.clear(); // tæmir strenginn
    let mut s3 = &s1;
    let mut s4 = &s1;
    println!("{}", s1);
    println!("{}", s2);
    let s5 = &s1;

    /* python
    s1 = [1,2,3,4]
    s2 = s1
    s2.append(5)
    print(s1)
     */

}
