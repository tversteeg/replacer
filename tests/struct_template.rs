#[rustfmt::skip]
replacer::rust_struct!{point; Point{ x: i32, y: i32};}

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
replacer::rust_struct!{pub rectangle; Square{ pos: replacer::rust_type!(point; Point;), size: replacer::rust_type!(point; Point;)};}

impl replacer::rust_type!(rectangle; Square;) {
    pub fn new() -> Self {
        Self {
            pos: <replacer::rust_type!(point; Point;)>::new(),
            size: <replacer::rust_type!(point; Point;)>::new(),
        }
    }
}

impl Default for replacer::rust_type!(rectangle; Square;) {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let shape = <replacer::rust_type!(rectangle; Square;)>::new();
    println!(
        "({}, {}, {}, {})",
        shape.pos.x, shape.pos.y, shape.size.x, shape.size.y
    );
}
