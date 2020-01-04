use replacer::{rule::StringRule, TemplateBuilder};

const STRING_TEMPLATE: &str = include_str!("string_template.rs");
const STRING_RESULT: &str = include_str!("string_result.rs");

#[test]
fn test_string() {
    let template = TemplateBuilder::new()
        .rule(StringRule::new("replace_with_world", "world").unwrap())
        .build();

    assert_eq!(template.apply(STRING_TEMPLATE).unwrap(), STRING_RESULT);
}
