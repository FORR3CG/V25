fn main() {
    let mut k = 5;
    while k > 0 {
        println!("{}", k);
        k -= 1;
    }

    let mut teljari = 5;
    'ytri: loop { // sama og while True: í python
        'innri: loop {
            teljari -= 1;
            if teljari == 0 {
                break 'ytri;
            } else if teljari == 3 {
                continue 'ytri;
            } else {
                break 'innri;
            }
        }
    }
}
