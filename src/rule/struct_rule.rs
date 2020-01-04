use anyhow::Result;
use regex::{Captures, Regex};

use crate::Rule;

/// Template macro for replacing a Rust struct with a placeholder struct that can be compiled.
///
/// ```rust
/// // Private
/// replacer::rust_struct!(replace_with_struct; Point2D; x: i32, y: i32;);
/// // Public
/// replacer::rust_struct!(pub replace_with_other_struct; Point3D; x: i32, y: i32, z: i32;);
/// ```
#[macro_export]
macro_rules! rust_struct {
    ($_name:ident; $placeholder:ident; $($element: ident: $ty: ty),*;) => {
        struct $placeholder { $($element: $ty),* }
    };
    (pub $_name:ident; $placeholder:ident; $($element: ident: $ty: ty),*;) => {
        pub struct $placeholder { $($element: $ty),* }
    };
}

/// Replace a Rust struct.
/// ```rust
/// # use replacer::rule::{Rule, StructRule};
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
            format!(
                "{}struct {} {{{} }}",
                &caps.name("pub").map_or("", |cap| cap.as_str()),
                replace_with,
                &caps.name("type").unwrap().as_str(),
            )
        });

        Ok(replace.into_owned())
    }
}

impl StructRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        let regex = Regex::new(&format!(
            r"replacer::rust_struct!\s*[\({{](?P<pub>pub )?{};[^;]+;(?P<type>[^;]+);[\)}}]",
            matches
        ))?;

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
    fn struct_rule() -> Result<()> {
        assert_eq!(
            StructRule::new("replace", "Point2D")?
                .convert("replacer::rust_struct! {replace; Point; x: i32, y: i32;}")?,
            "struct Point2D { x: i32, y: i32 }"
        );
        assert_eq!(
            StructRule::new("replace", "Point2D")?
                .convert("replacer::rust_struct! {pub replace; Point; x: i32, y: i32;}")?,
            "pub struct Point2D { x: i32, y: i32 }"
        );
        assert_eq!(
            StructRule::new("replace", "i32")?.convert("Hello world!")?,
            "Hello world!"
        );

        Ok(())
    }
}
