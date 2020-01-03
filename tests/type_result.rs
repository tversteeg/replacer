fn main() {
    // Unfortunately the type needs to be wrapped with angle brackets here
    let some_type = <std::path::PathBuf>::new();
    println!("{:?}", some_type);

    let some_generic_type: Vec<String> = vec![];
    println!("{:?}", some_generic_type);
}
