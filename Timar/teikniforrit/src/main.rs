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
            Teikniform::Punktur(point) => todo!(),
            Teikniform::Lina { p1, p2 } => todo!(),
            Teikniform::Kassi { upphaf, breidd, haed } => todo!(),
            Teikniform::Hringur { upphaf, radius } => todo!(),
        }
    }
}

fn main() {
    let p = Point(10, 20);
    let strengur = p.to_string();
    println!("{}", p);
    println!("Hello, world!");
}
