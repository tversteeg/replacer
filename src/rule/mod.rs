pub mod expr_rule;
pub mod string_rule;
pub mod struct_rule;
pub mod type_rule;

pub use expr_rule::*;
pub use string_rule::*;
pub use struct_rule::*;
pub use type_rule::*;

use anyhow::Result;

/// Generic way to add rules for a single file.
///
/// This trait can be implemented on a struct or enum for custom template handling.
pub trait Rule {
    /// Convert the matched values to a string.
    fn convert(&self, template: &str) -> Result<String>;
}
