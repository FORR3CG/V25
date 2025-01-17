use std::fmt::Display;

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}

enum Teikniform {
    Punktur(Point),
    Lina {p1: Point, p2: Point},
    Kassi {upphaf: Point, breidd: i32, haed: i32},
    Hringur {upphaf: Point, radius: f64},
}

impl Teikniform {
    fn teikna(&self) {
        match self {
            Teikniform::Punktur(point) 
                => println!("Teikna punkt í {}", point),
            Teikniform::Lina { p1, p2 } 
                => println!("Teikna línu frá {} til {}", p1, p2),
            Teikniform::Kassi { upphaf, breidd, haed } 
                => println!("Teikna kassa í {}, h: {}, b: {}", upphaf, haed, breidd),
            Teikniform::Hringur { upphaf, radius } 
                => println!("Teikna hring í {}, r: {}", upphaf, radius),
        }
    }
}

fn main() {
    let p = Point(10, 20);
    println!("{}", p);
    let strengur = p.to_string();
    let p = Teikniform::Punktur(Point::new(10, 20));
    let l = Teikniform::Lina 
            { p1: Point::new(10, 10), p2: Point::new(20,20) };
    let h = Teikniform::Hringur 
            { upphaf: Point::new(30,30), radius: 25.3 };
    let k = Teikniform::Kassi 
            { upphaf: Point(10,10), breidd: 35, haed: 20 };
    let t = [p, l, h, k];
    for tf in t {
        tf.teikna();
    }
}
