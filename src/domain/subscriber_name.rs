use regex::Regex;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        let mut errors: Vec<String> = vec![];

        if s.trim().is_empty() {
            errors.push("Name is required and cannot be empty or whitespace.".into())
        }

        let name_regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
        if !name_regex.is_match(&s) {
            errors.push("Name must contain only letters.".into());
        }

        if s.len() > 256 {
            errors.push("Name must be 256 characters or fewer.".into());
        };

        if !errors.is_empty() {
            return Err(format!("Invalid input: {}", errors.join(" | ")));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn test_subscriber_name_valid() {
        let name = SubscriberName::parse("Alice".into());
        assert_ok!(&name);
    }

    #[test]
    fn test_subscriber_name_invalid() {
        let name = SubscriberName::parse("Alice123".into());
        assert_err!(&name);
    }

    #[test]
    fn test_subscriber_name_empty() {
        let name = SubscriberName::parse("".into());
        assert_err!(&name);
    }

    #[test]
    fn test_subscriber_name_whitespace() {
        let name = SubscriberName::parse("  ".into());
        assert_err!(&name);
    }

    #[test]
    fn test_subscriber_name_too_long() {
        let name = SubscriberName::parse("A".repeat(257));
        assert_err!(&name);
    }
}
