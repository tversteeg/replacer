#[rustfmt::skip]
struct Point2D { x: i32, y: i32 }

impl Point2D {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

fn main() {
    let p = <Point2D>::new();
    println!("({}, {})", p.x, p.y);
}
