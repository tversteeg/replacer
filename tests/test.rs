use replacer::{StringRule, TemplateBuilder};

const STRING_TEMPLATE: &'static str = include_str!("templates/string.rs");
const STRING_RESULT: &'static str = include_str!("results/string.rs");

#[test]
fn test_string() {
    let template = TemplateBuilder::new()
        .rule(StringRule::new("replace_with_world", "world").unwrap())
        .build();

    assert_eq!(template.apply(STRING_TEMPLATE).unwrap(), STRING_RESULT);
}
