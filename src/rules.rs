/// Represents a birth/survival rule for a binary-state cellular automaton.
///
/// # Fields
/// - `birth`: a list of neighbor counts that will cause a dead cell to become alive.
/// - `survive`: a list of neighbor counts that will allow a live cell to remain alive.
///
/// # Example
/// Conway's Game of Life is represented as:
/// ```ignore
/// Rule {
///     birth: vec![3],
///     survive: vec![2, 3],
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Rule {
    pub birth: Vec<u8>,
    pub survive: Vec<u8>,
}

/// Represents an error that occurred while parsing a rule string.
///
/// Possible cases might include:
/// - Missing the 'B' or 'S' prefix
/// - Invalid format (not exactly one '/' separator)
/// - Non-digit characters in the birth/survival sections
/// - Empty birth or survival part
///
/// You can later expand this to include specific variants like:
/// `MissingPrefix`, `InvalidDigit`, `EmptyPart`, etc.
#[derive(Debug)]
pub enum RuleError {
    MissingPrefix,
    InvalidDigit(char),
    InvalidFormat,
}

/// Parses a rule string in the format "Bx/Sy" into a `Rule` object.
///
/// # Arguments
/// - `rule_str`: A string in the format "Bx/Sy", e.g., "B3/S23"
///
/// # Returns
/// - `Ok(Rule)` if parsing is successful
/// - `Err(RuleError)` if:
///     - There isn’t exactly one '/'
///     - The 'B' or 'S' prefix is missing
///     - Either section contains invalid characters or digits > 8
///     - Either part is empty
///
/// # Invariants
/// - All numbers in `birth` and `survive` must be between 0 and 8
/// - Duplicates are allowed but not necessary to filter out
impl Rule {
    pub fn from_str(rule_str: &str) -> Result<Self, RuleError> {
        let parts: Vec<&str> = rule_str.trim().split("/").collect();

        if parts.len() != 2 {
            return Err(RuleError::InvalidFormat);
        }

        let birth_str = parts[0].strip_prefix("B").ok_or(RuleError::MissingPrefix)?;
        let survive_str = parts[1].strip_prefix("S").ok_or(RuleError::MissingPrefix)?;

        let birth: Result<Vec<u8>, RuleError> = birth_str
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|n| n as u8)
                    .ok_or(RuleError::InvalidDigit(c))
            })
            .collect();

        let survive: Result<Vec<u8>, RuleError> = survive_str
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|n| n as u8)
                .ok_or(RuleError::InvalidDigit(c))
        })
        .collect();

        Ok(Self {
            birth: birth?,
            survive: survive?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rule() {
        let rule = Rule::from_str("B3/S23").unwrap();
        assert_eq!(rule.birth, vec![3]);
        assert_eq!(rule.survive, vec![2, 3]);
    }

    #[test]
    fn test_valid_rule_multiple_digits() {
        let rule = Rule::from_str("B1357/S02468").unwrap();
        assert_eq!(rule.birth, vec![1, 3, 5, 7]);
        assert_eq!(rule.survive, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_invalid_format_no_slash() {
        let err = Rule::from_str("B3S23").unwrap_err();
        assert!(matches!(err, RuleError::InvalidFormat));
    }

    #[test]
    fn test_missing_b_prefix() {
        let err = Rule::from_str("3/S23").unwrap_err();
        assert!(matches!(err, RuleError::MissingPrefix));
    }

    #[test]
    fn test_missing_s_prefix() {
        let err = Rule::from_str("B3/23").unwrap_err();
        assert!(matches!(err, RuleError::MissingPrefix));
    }

    #[test]
    fn test_invalid_digit_in_birth() {
        let err = Rule::from_str("Bx/S23").unwrap_err();
        assert!(matches!(err, RuleError::InvalidDigit('x')));
    }
}
