use replacer::{
    rule::{StructRule, TypeRule},
    TemplateBuilder,
};

const STRUCT_TEMPLATE: &str = include_str!("struct_template.rs");
const STRUCT_RESULT: &str = include_str!("struct_result.rs");

#[test]
fn test_struct() {
    let template = TemplateBuilder::new()
        .rule(StructRule::new("replace_with_struct", "Point2D").unwrap())
        .rule(TypeRule::new("replace_with_type", "Point2D").unwrap())
        .build();

    assert_eq!(template.apply(STRUCT_TEMPLATE).unwrap(), STRUCT_RESULT);
}
