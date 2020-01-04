#[rustfmt::skip]
replacer::rust_struct!{pub point; Point{ x: i32, y: i32};}

impl replacer::rust_type!(point; Point;) {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

impl Default for replacer::rust_type!(point; Point;) {
    fn default() -> Self {
        Self::new()
    }
}

#[rustfmt::skip]
replacer::rust_struct!{rectangle; Square<'a>{ pos: &'a replacer::rust_type!(point; Point;), size: replacer::rust_type!(point; Point;)};}

impl<'a> replacer::rust_type!(rectangle_lifetime; Square<'a>;) {
    pub fn new(pos: &'a replacer::rust_type!(point; Point;)) -> Self {
        Self {
            pos,
            size: <replacer::rust_type!(point; Point;)>::new(),
        }
    }
}

fn main() {
    let pos = <replacer::rust_type!(point; Point;)>::new();
    let shape = <replacer::rust_type!(rectangle; Square;)>::new(&pos);
    println!(
        "({}, {}, {}, {})",
        shape.pos.x, shape.pos.y, shape.size.x, shape.size.y
    );
}
