use anyhow::Result;
use regex::Regex;

use crate::Rule;

/// Template macro for replacing a Rust expression with a placeholder expression that can be compiled.
///
/// ```rust
/// let two = replacer::rust_expr!(replace_with_expression; 1 + 1;);
/// ```
#[macro_export]
macro_rules! rust_expr {
    ($_name:ident; $placeholder:expr;) => {
        $placeholder
    };
}

/// Replace a Rust expression.
/// ```rust
/// # use replacer::rule::{Rule, ExprRule};
/// # fn main() -> anyhow::Result<()> {
/// let rule = ExprRule::new("replace_with_expression", "1 + 1")?;
/// assert_eq!(rule.convert("let two = replacer::rust_expr!(replace_with_expression; 2 + 2;);")?,
///     "let two = 1 + 1;");
/// # Ok(())
/// # }
/// ```
pub struct ExprRule {
    /// What the keyword will be replaced with.
    replace_with: String,
    /// Regex used to find the macro.
    regex: Regex,
}

impl Rule for ExprRule {
    fn convert(&self, template: &str) -> Result<String> {
        let replace_with: &str = &self.replace_with;
        let replace = self.regex.replace_all(template, replace_with);

        Ok(replace.into_owned())
    }
}

impl ExprRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        let regex = Regex::new(&format!(r"replacer::rust_expr!\({};[^;]+;\)", matches))?;

        Ok(Self {
            replace_with: replace_with.to_string(),
            regex,
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn expr_rule() -> Result<()> {
        assert_eq!(
            ExprRule::new("replace", "1 + 1")?
                .convert("println!(\"{}\", replacer::rust_expr!(replace; true;));")?,
            "println!(\"{}\", 1 + 1);"
        );
        assert_eq!(
            ExprRule::new("replace", "1 + 1")?.convert("Hello world!")?,
            "Hello world!"
        );

        Ok(())
    }
}
