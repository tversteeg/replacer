use replacer::{StringRule, TemplateBuilder, TypeRule};

const STRING_TEMPLATE: &'static str = include_str!("templates/string.rs");
const STRING_RESULT: &'static str = include_str!("results/string.rs");

const TYPE_TEMPLATE: &'static str = include_str!("templates/type.rs");
const TYPE_RESULT: &'static str = include_str!("results/type.rs");

#[test]
fn test_string() {
    let template = TemplateBuilder::new()
        .rule(StringRule::new("replace_with_world", "world").unwrap())
        .build();

    assert_eq!(template.apply(STRING_TEMPLATE).unwrap(), STRING_RESULT);
}

#[test]
fn test_type() {
    let template = TemplateBuilder::new()
        .rule(TypeRule::new("replace_with_type", "PathBuf").unwrap())
        .rule(TypeRule::new("replace_with_type_in_vec", "String").unwrap())
        .build();

    assert_eq!(template.apply(TYPE_TEMPLATE).unwrap(), TYPE_RESULT);
}
