#[rustfmt::skip]
replacer::rust_struct!{replace_with_point; Point; x: i32, y: i32;}

impl replacer::rust_type!(replace_with_point; Point;) {
    pub fn new() -> Self {
        Self { x: 10, y: 20 }
    }
}

#[rustfmt::skip]
replacer::rust_struct!{pub replace_with_rectangle; Square; x: i32, y: i32, width: i32, height: i32;}

fn main() {
    let p = <replacer::rust_type!(replace_with_point; Point;)>::new();
    println!("({}, {})", p.x, p.y);
}
