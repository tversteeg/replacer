use anyhow::Result;
use regex::{Captures, Regex};

use crate::Rule;

/// Template macro for replacing a Rust struct with a placeholder struct that can be compiled.
///
/// ```rust
/// // Private
/// replacer::rust_struct!(replace_with_struct; Point2D { x: i32, y: i32 };);
/// // Public
/// replacer::rust_struct!(pub replace_with_other_struct; Point3D { x: i32, y: i32, z: i32 };);
/// // With a lifetime
/// replacer::rust_struct!(replace_with_struct; Point4D<'a> { x: i32, y: &'a i32, z: i32, w: i32 };);
/// ```
#[macro_export]
macro_rules! rust_struct {
    // No lifetime, private
    ($_name:ident; $placeholder:ident {$($element: ident: $ty: ty),*};) => {
        struct $placeholder { $($element: $ty),* }
    };
    // No lifetime, public
    (pub $_name:ident; $placeholder:ident {$($element: ident: $ty: ty),*};) => {
        pub struct $placeholder { $($element: $ty),* }
    };
    // Lifetime, private
    ($_name:ident; $placeholder:ident<$lifetime:lifetime>{$($element: ident: $ty: ty),*};) => {
        struct $placeholder<$lifetime> { $($element: $ty),* }
    };
    // Lifetime, public
    (pub $_name:ident; $placeholder:ident<$lifetime:lifetime>{$($element: ident: $ty: ty),*};) => {
        pub struct $placeholder<$lifetime> { $($element: $ty),* }
    };
}

/// Replace a Rust struct.
/// ```rust
/// # use replacer::rule::{Rule, StructRule};
/// # fn main() -> anyhow::Result<()> {
/// let rule = StructRule::new("point", "Point3D { x: i32, y: i32, z: i32 }")?;
/// assert_eq!(rule.convert("replacer::rust_struct!(point; Point2D{ x: i32, y: i32};}")?,
///     "struct Point3D { x: i32, y: i32, z: i32 }");
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
                "{}struct {}",
                &caps.name("pub").map_or("", |cap| cap.as_str()),
                replace_with,
            )
        });

        Ok(replace.into_owned())
    }
}

impl StructRule {
    /// Setup a new rule.
    pub fn new(matches: &str, replace_with: &str) -> Result<Self> {
        let regex = Regex::new(&format!(
            r"{}[\({{]{}[\)}}]",
            r"replacer::rust_struct!\s*",
            format!(r"(?P<pub>pub )?{};[^{{]+\{{[^;]+}};", matches)
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
            StructRule::new("replace", "Point2D { x: i32, y: i32 }")?
                .convert("replacer::rust_struct! {replace; Point { x: i32, y: i32};}")?,
            "struct Point2D { x: i32, y: i32 }"
        );
        assert_eq!(
            StructRule::new("replace", "Point2D { x: i32, y: i32 }")?
                .convert("replacer::rust_struct! {pub replace; Point{ x: i32, y: i32};}")?,
            "pub struct Point2D { x: i32, y: i32 }"
        );
        assert_eq!(
            StructRule::new("replace", "i32")?.convert("Hello world!")?,
            "Hello world!"
        );

        Ok(())
    }
}
