/* 
enum Option<T> {
    Some(T),
    None,
} 
*/

// breyta = None

fn deila(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        Some(a / b)
    } else {
        None
    }
}

fn fall_sem_tekur_inn_vector(v: &[i32]) {
    println!("{:?}", v)
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("len: {}, cap: {}, v: {:?}", v.len(), v.capacity(), v);
    v.push(10);
    v.push(11);
    v.push(12);
    fall_sem_tekur_inn_vector(&v);
    v.push(13);
    v.push(14);
    v.pop(); // Last in first out (LIFO)
    v.pop();
    println!("len: {}, cap: {}, v: {:?}", v.len(), v.capacity(), v);
    for i in 0..9 {
        if let Some(t) = v.get(i) {
            println!("{}", t)
        }
    }

    for t in &v {
        print!("{} - ", t)
    }

    //v[8] = 99;
    println!("{:?}", v.get(1).replace(&99));

    for t in &mut v {
        *t += 10;
    }

    // nota first í stað get(0)
    if let Some(f) = v.first() {
        println!("1: {}", f)
    }

    // nota last í stað get(v.len() - 1)
    if let Some(s) = v.last() {
        println!("Síðasti: {}", s)
    }

    let mut v2 = vec![34, 23, 88, 12, 56];
    for w in v2.windows(2) {
        println!("{:?}", w)
    }
    for c in v2.chunks(2) {
        println!("{:?}", c)
    }
    v2.sort();
    println!("{:?}", v2);

    let v3 = &mut v2[2..];

/*
    let i = 10;
    let j = 0;

    match deila(i, j) {
        Some(d) => println!("{} / {} = {}", i, j, d),
        None => println!("j má ekki vera núll"),
    }

    if let Some(d) = deila(i, j) {
        println!("{} / {} = {}", i, j, d)
    }

    let mut k: Option<i32> = None;
    let j = k.unwrap_or_default() + 5;
    match k {
        Some(j) => println!("{} + 5 = {}", j, j + 5),
        None => println!("k er None"),
    }

    if let Some(j) = k {
        println!("{} + 5 = {}", j, j + 5)
    } else { // optional
        println!("k er None")
    }

    println!("Hello, world!");
 */
}
