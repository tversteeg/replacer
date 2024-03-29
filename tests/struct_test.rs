use pretty_assertions::assert_eq;

use replacer::{
    rule::{StructRule, TypeRule},
    TemplateBuilder,
};

const STRUCT_TEMPLATE: &str = include_str!("struct_template.rs");
const STRUCT_RESULT: &str = include_str!("struct_result.rs");

#[test]
fn test_struct() {
    let template = TemplateBuilder::new()
        .rule(StructRule::new("point", "Point2D { x: i32, y: i32 }").unwrap())
        .rule(TypeRule::new("point", "Point2D").unwrap())
        .rule(
            StructRule::new(
                "rectangle",
                "Rectangle<'a> { pos: &'a Point2D, size: Point2D }",
            )
            .unwrap(),
        )
        .rule(TypeRule::new("rectangle", "Rectangle").unwrap())
        .rule(TypeRule::new("rectangle_lifetime", "Rectangle<'a>").unwrap())
        .build();

    assert_eq!(template.apply(STRUCT_TEMPLATE).unwrap(), STRUCT_RESULT);
}
