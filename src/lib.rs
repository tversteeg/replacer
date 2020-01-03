//! A crate for creating templates from Rust source files.
//!
//! # Features
//!
//! ### Valid Rust
//!
//! - All templates can be compilable Rust source files
//! - Linting & language servers work
//!
//! ### Extensible
//!
//! - Implement [`Rule`] to add new rules
//!
//! # Example
//!
//! ```rust
//! use replacer::{Rule, StringRule, TemplateBuilder};
//!
//! fn main() -> anyhow::Result<()> {
//!     let template = TemplateBuilder::new()
//!         .rule(StringRule::new("replace", "world")?)
//!         .build();
//!
//!     assert_eq!(template.apply("Hello $$replace$$!")?, "Hello world!");
//!
//!     Ok(())
//! }
//! ```
//!
//! # Rules
//!
//! ### [`StringRule`]
//!
//! ```rust
//! let some_str = "Hello $$replace_with_world$$!";
//! # assert_eq!(some_str, "Hello $$replace_with_world$$!");
//!
//! // Also works in comments, hello $$replace_with_world$$
//! ```
//!
//! ### [`TypeRule`]
//!
//! ```rust
//! // Unfortunately the type needs to be wrapped with angle brackets here
//! let some_type = <replacer::rust_type!(replace_with_type; String;)>::new();
//! # assert_eq!(some_type, "");
//!
//! let some_generic_type: Vec<replacer::rust_type!(replace_with_type_in_vec; i32;)> = vec![];
//! # assert_eq!(some_generic_type, vec![]);
//! ```
//!
//! ### [`StructRule`]
//!
//! ```rust
//! replacer::rust_struct!(replace_with_struct; Point; x: i32, y: i32;);
//! ```
//!
//! ### [`ExprRule`]
//!
//! ```rust
//! println!("1 + 1 = {}", replacer::rust_expr!(replace_with_expression; 1 + 2;));
//! ```

use anyhow::Result;
use regex::{Captures, Regex};

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

/// Template macro for replacing a Rust struct with a placeholder struct that can be compiled.
///
/// ```rust
/// replacer::rust_struct!(replace_with_struct; Point; x: i32, y: i32;);
/// ```
#[macro_export]
macro_rules! rust_struct {
    ($_name:ident; $placeholder:ident; $($element: ident: $ty: ty),*;) => {
        struct $placeholder { $($element: $ty),* }
    };
}

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

/// Generic way to add rules for a single file.
///
/// This trait can be implemented on a struct or enum for custom template handling.
pub trait Rule {
    /// Convert the matched values to a string.
    fn convert(&self, template: &str) -> Result<String>;
}

/// Replace a string inside another string.
///
/// This will look for any code containing the `${..}` sequence where `..` is
/// filled with the matches.
/// ```rust
/// # use replacer::{Rule, StringRule};
/// # fn main() -> anyhow::Result<()> {
/// let rule = StringRule::new("replace", "world")?;
/// assert_eq!(rule.convert("Hello $$replace$$!")?, "Hello world!");
/// # Ok(())
/// # }
/// ```
pub struct StringRule {
    /// The keyword that will be matched with.
    /// This is the `${..}` part in the string.
    matches: String,
    /// What the keyword will be replaced with.
    replace_with: String,
}

impl Rule for StringRule {
    fn convert(&self, template: &str) -> Result<String> {
        Ok(template.replace(&self.matches, &self.replace_with))
    }
}

impl StringRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        Ok(Self {
            matches: format!("$${}$$", matches),
            replace_with: replace_with.to_string(),
        })
    }
}

/// Replace a Rust type.
/// ```rust
/// # use replacer::{Rule, TypeRule};
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

/// Replace a Rust struct.
/// ```rust
/// # use replacer::{Rule, StructRule};
/// # fn main() -> anyhow::Result<()> {
/// let rule = StructRule::new("replace_with_struct", "Point2D")?;
/// assert_eq!(rule.convert("replacer::rust_struct!(replace_with_struct; Point; x: i32, y: i32;}")?,
///     "struct Point2D { x: i32, y: i32 }");
/// # Ok(())
/// # }
/// ```
pub struct StructRule {
    /// What the keyword will be replaced with.
    replace_with: String,
    /// Regex used to find the macro.
    regex: Regex,
}

impl Rule for StructRule {
    fn convert(&self, template: &str) -> Result<String> {
        let replace_with: &str = &self.replace_with;
        let replace = self.regex.replace_all(template, |caps: &Captures| {
            format!("struct {} {{{} }}", replace_with, &caps[1])
        });

        Ok(replace.into_owned())
    }
}

impl StructRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        let regex = Regex::new(&format!(
            r"replacer::rust_struct!\s*[\({{]{};[^;]+;([^;]+);[\)}}]",
            matches
        ))?;

        Ok(Self {
            replace_with: replace_with.to_string(),
            regex,
        })
    }
}

/// Replace a Rust expression.
/// ```rust
/// # use replacer::{Rule, ExprRule};
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

/// Builder for the [`Template`] struct.
#[derive(Default)]
pub struct TemplateBuilder {
    rules: Vec<Box<dyn Rule>>,
}

impl TemplateBuilder {
    /// Start building a new [`Template`] struct.
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    /// Add a new rule that can be applied in batch.
    ///
    /// A rule is defined by anything that implements the [`Rule`] trait.
    ///
    /// ```rust
    /// # use replacer::{Rule, StringRule, TemplateBuilder};
    /// # fn main() -> anyhow::Result<()> {
    /// let template = TemplateBuilder::new()
    ///     .rule(StringRule::new("replace", "world")?)
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn rule<R>(mut self, rule: R) -> Self
    where
        R: Rule + 'static,
    {
        self.rules.push(Box::new(rule));

        self
    }

    /// Create the [`Template`] struct.
    pub fn build(self) -> Template {
        Template { rules: self.rules }
    }
}

/// Internal representation of the template file.
///
/// Use [`TemplateBuilder`] to instaniate a new Template.
///
/// ```rust
/// # use replacer::{Rule, StringRule, TemplateBuilder};
/// # fn main() -> anyhow::Result<()> {
/// let template = TemplateBuilder::new()
///     .rule(StringRule::new("replace", "world")?)
///     .build();
///
/// assert_eq!(template.apply("Hello $$replace$$")?, "Hello world");
/// # Ok(())
/// # }
/// ```
pub struct Template {
    rules: Vec<Box<dyn Rule>>,
}

impl Template {
    /// Apply all rules sequentially or return the first error.
    pub fn apply(&self, code: &str) -> Result<String> {
        self.rules
            .iter()
            .fold(Ok(code.to_string()), |code, rule| match code {
                // Apply the rule and return the string if there are no errors
                Ok(code) => rule.convert(&code),
                // Propagate errors further
                Err(err) => Err(err),
            })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn string_rule() -> Result<()> {
        assert_eq!(
            StringRule::new("replace", "world")?.convert("Hello $$replace$$!")?,
            "Hello world!"
        );
        assert_eq!(
            StringRule::new("replace", "world")?.convert("Hello world!")?,
            "Hello world!"
        );
        assert_eq!(
            StringRule::new("replace", "world")?.convert("Hello $$replace$$, bye $$replace$$!")?,
            "Hello world, bye world!"
        );

        Ok(())
    }

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

    #[test]
    fn struct_rule() -> Result<()> {
        assert_eq!(
            StructRule::new("replace", "Point2D")?
                .convert("replacer::rust_struct! {replace; Point; x: i32, y: i32;}")?,
            "struct Point2D { x: i32, y: i32 }"
        );
        assert_eq!(
            StructRule::new("replace", "i32")?.convert("Hello world!")?,
            "Hello world!"
        );

        Ok(())
    }
}
