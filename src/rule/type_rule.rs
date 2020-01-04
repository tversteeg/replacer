use anyhow::Result;
use regex::Regex;

use crate::Rule;

/// Template macro for replacing a Rust type with a placeholder type that can be compiled.
///
/// ```rust
/// let some_type = <replacer::rust_type!(replace_with_type; String;)>::new();
/// # assert_eq!(some_type, "");
/// ```
#[macro_export]
macro_rules! rust_type {
    ($_name:ident; $placeholder:ty;) => {
        $placeholder
    };
}

/// Replace a Rust type.
/// ```rust
/// # use replacer::rule::{Rule, TypeRule};
/// # fn main() -> anyhow::Result<()> {
/// let rule = TypeRule::new("replace_with_type", "PathBuf")?;
/// assert_eq!(rule.convert("let some_type = <replacer::rust_type!(replace_with_type; String;)>::new();")?, "let some_type = <PathBuf>::new();");
/// # Ok(())
/// # }
/// ```
pub struct TypeRule {
    /// What the keyword will be replaced with.
    replace_with: String,
    /// Regex used to find the macro.
    regex: Regex,
}

impl Rule for TypeRule {
    fn convert(&self, template: &str) -> Result<String> {
        let replace_with: &str = &self.replace_with;
        let replace = self.regex.replace_all(template, replace_with);

        Ok(replace.into_owned())
    }
}

impl TypeRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        let regex = Regex::new(&format!(r"replacer::rust_type!\({};[^;]+;\)", matches))?;

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
    fn type_rule() -> Result<()> {
        assert_eq!(
            TypeRule::new("replace", "i32")?
                .convert("let some_type = <replacer::rust_type!(replace; String;)>::new();")?,
            "let some_type = <i32>::new();"
        );
        assert_eq!(
            TypeRule::new("replace", "i32")?.convert("Hello world!")?,
            "Hello world!"
        );
        assert_eq!(
            TypeRule::new("replace", "i32")?
                .convert("let some_type = Map<replacer::rust_type!(replace; String;), replacer::rust_type!(replace; String;)>::new();")?,
            "let some_type = Map<i32, i32>::new();"
        );

        Ok(())
    }
}
