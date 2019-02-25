#[derive(Clone, EnumString, Eq, Debug, PartialEq, ToString)]
pub enum Literal {
    #[strum(default = "true")]
    Immediate(String),
}

impl Literal {
    /// Check whether the immediate has a valid prefix.
    pub fn is_valid(&mut self) -> bool {
        match self {
            Literal::Immediate(immed) => {
                let mut is_valid = immed.starts_with("#");
                immed.remove(0); // Remove prefix #.
                if immed.starts_with("'") {
                    // Value encoded as an Ascii character.
                    immed.remove(0); // Remove prefix '.
                    immed.pop(); // Remove suffix '.
                    return immed.contains(char::is_alphabetic) && immed.len() == 1;
                }
                let immed = immed.trim_start_matches("0x");
                // Ensure value is parable to u32.
                if let Err(_) = immed.parse::<u32>() {
                    is_valid = false
                }
                return is_valid;
            }
        }
    }
    /// Get the immediate value.
    pub fn get_value(self) -> u32 {
        match self {
            Literal::Immediate(immed) => {
                if immed.contains("0x") {
                    // Get the value encoded as base 16.
                    let immed = immed.trim_start_matches("0x");
                    return u32::from_str_radix(&immed, 16).unwrap();
                }
                if immed.contains(char::is_alphabetic) {
                    //  Get the value encoded as an Ascii character.
                    return (immed.chars().next().unwrap() as char) as u32;
                }
                return immed.parse::<u32>().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_valid_with_base_ascii() {
        assert!(Literal::Immediate(String::from("#'D'")).is_valid())
    }

    #[test]
    fn test_is_valid_with_base_10() {
        assert!(Literal::Immediate(String::from("#1234")).is_valid())
    }

    #[test]
    fn test_is_valid_with_base_16() {
        assert!(Literal::Immediate(String::from("#0x1234")).is_valid())
    }

    #[test]
    fn test_is_valid_out_of_bounds() {
        assert!(!Literal::Immediate(String::from("#0x1FFFFFFFF")).is_valid())
    }

    #[test]
    fn test_get_value_with_base_ascii() {
        assert_eq!(Literal::Immediate(String::from("d")).get_value(), 100)
    }

    #[test]
    fn test_get_value_with_base_10() {
        assert_eq!(Literal::Immediate(String::from("1234")).get_value(), 1234)
    }

    #[test]
    fn test_get_value_with_base_16() {
        assert_eq!(
            Literal::Immediate(String::from("0x1234")).get_value(),
            0x1234
        )
    }

}
