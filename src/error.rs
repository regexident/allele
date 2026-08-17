use thiserror::Error;

/// Returned by constructors when a parameter value is outside its valid range.
#[derive(Error, Clone, Debug, PartialEq)]
#[error("invalid argument `{parameter}`: {message}")]
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

#[derive(Error, Copy, Clone, Debug)]
pub enum Error {
    #[error("sum of the configured weights must not be zero")]
    ZeroWeightSum,
    #[error("parents must not be empty")]
    EmptyParents,
    #[error("tournament size must be at least 2, got {value}")]
    InvalidTournamentSize { value: usize },
    #[error("selection iteration limit of {limit} exceeded")]
    SelectionIterationLimitExceeded { limit: usize },
    #[error("selection ratio must be in (0.0, 1.0], got {value}")]
    InvalidSelectionRatio { value: f64 },
    #[error("probability must be in [0.0, 1.0], got {value}")]
    InvalidProbability { value: f64 },
    #[error("number of individuals per parents must be at least 1, got {value}")]
    InvalidNumIndividualsPerParents { value: usize },
    #[error("zero parents were selected")]
    ZeroParentsSelected,
    #[error("number of cut points must be at least 1, got {value}")]
    InvalidNumCutPoints { value: usize },
    #[error("mutation rate must be in [0.0, 1.0], got {value}")]
    InvalidMutationRate { value: f64 },
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::ZeroWeightSum, Error::ZeroWeightSum) => true,
            (Error::EmptyParents, Error::EmptyParents) => true,
            (
                Error::InvalidTournamentSize { value: a },
                Error::InvalidTournamentSize { value: b },
            ) => a == b,
            (
                Error::SelectionIterationLimitExceeded { limit: a },
                Error::SelectionIterationLimitExceeded { limit: b },
            ) => a == b,
            (
                Error::InvalidSelectionRatio { value: a },
                Error::InvalidSelectionRatio { value: b },
            ) => a.to_bits() == b.to_bits(),
            (Error::InvalidProbability { value: a }, Error::InvalidProbability { value: b }) => {
                a.to_bits() == b.to_bits()
            }
            (Error::InvalidMutationRate { value: a }, Error::InvalidMutationRate { value: b }) => {
                a.to_bits() == b.to_bits()
            }
            (
                Error::InvalidNumIndividualsPerParents { value: a },
                Error::InvalidNumIndividualsPerParents { value: b },
            ) => a == b,
            (Error::ZeroParentsSelected, Error::ZeroParentsSelected) => true,
            (Error::InvalidNumCutPoints { value: a }, Error::InvalidNumCutPoints { value: b }) => {
                a == b
            }
            _ => false,
        }
    }
}

impl Eq for Error {}

impl std::hash::Hash for Error {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Error::ZeroWeightSum | Error::EmptyParents | Error::ZeroParentsSelected => {}
            Error::InvalidTournamentSize { value } => value.hash(state),
            Error::SelectionIterationLimitExceeded { limit } => limit.hash(state),
            Error::InvalidSelectionRatio { value } => value.to_bits().hash(state),
            Error::InvalidProbability { value } => value.to_bits().hash(state),
            Error::InvalidMutationRate { value } => value.to_bits().hash(state),
            Error::InvalidNumIndividualsPerParents { value } => value.hash(state),
            Error::InvalidNumCutPoints { value } => value.hash(state),
        }
    }
}
