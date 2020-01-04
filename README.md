# replacer

Creating compilable Rust source code templates.

<a href="https://actions-badge.atrox.dev/tversteeg/replacer/goto"><img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Ftversteeg%2Freplacer%2Fbadge&style=flat" alt="Build Status"/></a>
<a href="https://crates.io/crates/replacer"><img src="https://img.shields.io/crates/v/replacer.svg" alt="Version"/></a>
<a href="https://docs.rs/replacer"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
<img src="https://img.shields.io/crates/l/replacer.svg" alt="License"/>

## Example

Rust source template:

```rust
fn main() {
	println!("Hello $$replace_with_string$$!");

    let some_type = <replacer::rust_type!(replace_with_type; String;)>::new();
}
```

Rust script to parse the template:

```rust
use replacer::{rule::{StringRule, TypeRule}, TemplateBuilder};

fn main() {
    let template = TemplateBuilder::new()
        .rule(StringRule::new("replace_with_string", "world").unwrap())
        .rule(TypeRule::new("replace_with_type", "Vec").unwrap())
        .build();

    println!(template.apply(include_str!(SOURCE_TEMPLATE_FROM_ABOVE)).unwrap());
}
```

Rust template that will be printed:

```rust
fn main() {
	println!("Hello world!");

	let some_type = <Vec>::new();
}
```
