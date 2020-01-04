use replacer::{
    rule::{StructRule, TypeRule},
    TemplateBuilder,
};

const STRUCT_TEMPLATE: &str = include_str!("struct_template.rs");
const STRUCT_RESULT: &str = include_str!("struct_result.rs");

#[test]
fn test_struct() {
    let template = TemplateBuilder::new()
        .rule(StructRule::new("point", "Point2D").unwrap())
        .rule(TypeRule::new("point", "Point2D").unwrap())
        .rule(StructRule::new("rectangle", "Rectangle").unwrap())
        .rule(TypeRule::new("rectangle", "Rectangle").unwrap())
        .build();

    assert_eq!(template.apply(STRUCT_TEMPLATE).unwrap(), STRUCT_RESULT);
}
