//! A crate for creating Rust templates.
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
//! use project_template::{Rule, StringRule, TemplateBuilder};
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
//! ```
//!
//! ### [`TypeRule`]
//!
//! ```rust
//! // Unfortunately the type needs to be wrapped with angle brackets here
//! let some_type = <project_template::rust_type!(replace_with_type; String)>::new();
//!
//! let some_generic_type: Vec<project_template::rust_type!(replace_with_type_in_vec; i32)> = vec![];
//! ```

use anyhow::Result;

/// Template macro for replacing a Rust type with a default type that can be compiled.
#[macro_export]
macro_rules! rust_type {
    ($_name:ident; $replacement_type:ty) => {
        $replacement_type
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
/// # use project_template::{Rule, StringRule};
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
        todo!();
    }
}

impl StringRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        Ok(Self {
            matches: matches.to_string(),
            replace_with: replace_with.to_string(),
        })
    }
}

/// Builder for the [`Template`] struct.
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
    /// # use project_template::{Rule, StringRule, TemplateBuilder};
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
/// # use project_template::{Rule, StringRule, TemplateBuilder};
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
    use super::*;

    #[test]
    fn string_rule() {
        assert_eq!(
            StringRule::new("replace", "world")
                .unwrap()
                .convert("Hello $$replace$$!")
                .unwrap(),
            "Hello world!"
        );
        assert_eq!(
            StringRule::new("replace", "world")
                .unwrap()
                .convert("Hello world!")
                .unwrap(),
            "Hello world!"
        );
        assert_eq!(
            StringRule::new("replace", "world")
                .unwrap()
                .convert("Hello $$replace$$, bye $$replace$$!")
                .unwrap(),
            "Hello world, bye world!"
        );
    }
}
