#[rustfmt::skip]
struct Point2D { x: i32, y: i32 }

impl Point2D {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

#[rustfmt::skip]
pub struct Rectangle { x: i32, y: i32, width: i32, height: i32 }

fn main() {
    let p = <Point2D>::new();
    println!("({}, {})", p.x, p.y);
}
