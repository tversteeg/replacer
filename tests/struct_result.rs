#[rustfmt::skip]
struct Point2D { x: i32, y: i32 }

impl Point2D {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

#[rustfmt::skip]
pub struct Rectangle { pos: Point2D, size: Point2D }

impl Rectangle {
    pub fn new() -> Self {
        Self {
            pos: <Point2D>::new(),
            size: <Point2D>::new(),
        }
    }
}

fn main() {
    let shape = <Rectangle>::new();
    println!(
        "({}, {}, {}, {})",
        shape.pos.x, shape.pos.y, shape.size.x, shape.size.y
    );
}
