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
//! - Implement [`rule::Rule`] to add new rules
//!
//! # Example
//!
//! ```rust
//! use replacer::{rule::StringRule, TemplateBuilder};
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
//! ### [`rule::StringRule`]
//!
//! ```rust
//! let some_str = "Hello $$replace_with_world$$!";
//! # assert_eq!(some_str, "Hello $$replace_with_world$$!");
//!
//! // Also works in comments, hello $$replace_with_world$$
//! ```
//!
//! ### [`rule::TypeRule`]
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
//! ### [`rule::StructRule`]
//!
//! ```rust
//! replacer::rust_struct!(replace_with_struct; Point{ x: i32, y: i32 };);
//! ```
//!
//! ### [`rule::ExprRule`]
//!
//! ```rust
//! println!("1 + 1 = {}", replacer::rust_expr!(replace_with_expression; 1 + 2;));
//! ```

pub mod rule;

use anyhow::Result;

use rule::Rule;

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
    /// A rule is defined by anything that implements the [`rule::Rule`] trait.
    ///
    /// ```rust
    /// # use replacer::{rule::StringRule, TemplateBuilder};
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
/// # use replacer::{rule::StringRule, TemplateBuilder};
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
