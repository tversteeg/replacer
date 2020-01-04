#[rustfmt::skip]
pub struct Point2D { x: i32, y: i32 }

impl Point2D {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

impl Default for Point2D {
    fn default() -> Self {
        Self::new()
    }
}

#[rustfmt::skip]
struct Rectangle<'a> { pos: &'a Point2D, size: Point2D }

impl<'a> Rectangle<'a> {
    pub fn new(pos: &'a Point2D) -> Self {
        Self {
            pos,
            size: <Point2D>::new(),
        }
    }
}

fn main() {
    let pos = <Point2D>::new();
    let shape = <Rectangle>::new(&pos);
    println!(
        "({}, {}, {}, {})",
        shape.pos.x, shape.pos.y, shape.size.x, shape.size.y
    );
}
