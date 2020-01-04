use pretty_assertions::assert_eq;

use replacer::{rule::TypeRule, TemplateBuilder};

const TYPE_TEMPLATE: &str = include_str!("type_template.rs");
const TYPE_RESULT: &str = include_str!("type_result.rs");

#[test]
fn test_type() {
    let template = TemplateBuilder::new()
        .rule(TypeRule::new("replace_with_type", "std::path::PathBuf").unwrap())
        .rule(TypeRule::new("replace_with_type_in_vec", "String").unwrap())
        .build();

    assert_eq!(template.apply(TYPE_TEMPLATE).unwrap(), TYPE_RESULT);
}
