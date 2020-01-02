fn main() {
    println!("Hello $$replace_with_world$$!");

    // Unfortunately the type needs to be wrapped with angle brackets here
    let some_type = <replacer::rust_type!(replace_with_type; String)>::new();

    let some_generic_type: Vec<replacer::rust_type!(replace_with_type_in_vec; i32)> = vec![];
}
