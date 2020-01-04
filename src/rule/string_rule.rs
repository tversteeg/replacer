use anyhow::Result;

use crate::Rule;

/// Replace a string inside another string.
///
/// This will look for any code containing the `${..}` sequence where `..` is
/// filled with the matches.
/// ```rust
/// # use replacer::rule::{Rule, StringRule};
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
}
