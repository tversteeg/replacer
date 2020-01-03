#[rustfmt::skip]
replacer::rust_struct!{replace_with_struct; Point; x: i32, y: i32;}

impl replacer::rust_type!(replace_with_type; Point;) {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

fn main() {
    let p = <replacer::rust_type!(replace_with_type; Point;)>::new();
    println!("({}, {})", p.x, p.y);
}
