use pretty_assertions::assert_eq;

use replacer::{rule::ExprRule, TemplateBuilder};

const EXPR_TEMPLATE: &str = include_str!("expr_template.rs");
const EXPR_RESULT: &str = include_str!("expr_result.rs");

#[test]
fn test_expr() {
    let template = TemplateBuilder::new()
        .rule(ExprRule::new("replace_with_expression", "1 + 1").unwrap())
        .build();

    assert_eq!(template.apply(EXPR_TEMPLATE).unwrap(), EXPR_RESULT);
}
