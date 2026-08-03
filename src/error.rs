use std::fmt;

/// Returned by constructors when a parameter value is outside its valid range.
#[derive(Clone, Debug, PartialEq)]
pub struct InvalidArgumentError {
    pub parameter: &'static str,
    pub message: String,
}

impl InvalidArgumentError {
    pub fn new(parameter: &'static str, message: impl Into<String>) -> Self {
        InvalidArgumentError {
            parameter,
            message: message.into(),
        }
    }
}

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid argument `{}`: {}", self.parameter, self.message)
    }
}

impl std::error::Error for InvalidArgumentError {}
