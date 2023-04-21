use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput(String),
    // Other error cases can be added here
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidInput(input) => write!(f, "Invalid input: {}", input),
        }
    }
}
