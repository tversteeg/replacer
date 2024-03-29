fn main() {
    // Unfortunately the type needs to be wrapped with angle brackets here
    let some_type = <replacer::rust_type!(replace_with_type; String;)>::new();
    println!("{:?}", some_type);

    let some_generic_type: Vec<replacer::rust_type!(replace_with_type_in_vec; i32;)> = vec![];
    println!("{:?}", some_generic_type);
}
