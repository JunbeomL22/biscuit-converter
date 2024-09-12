#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    NonDecimal,
    Empty,
    Overflow,
    Unknown,
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::NonDecimal => write!(f, "NonDecimal"),
            CheckError::Empty => write!(f, "Empty"),
            CheckError::Overflow => write!(f, "Overflow"),
            CheckError::Unknown => write!(f, "UnknownError"),
        }
    }
}

impl std::error::Error for CheckError {}

// String으로 변환
impl From<CheckError> for String {
    fn from(error: CheckError) -> Self {
        error.to_string()
    }
}

impl CheckError {
    pub fn as_str(&self) -> &str {
        match self {
            CheckError::NonDecimal => "NonDecimal",
            CheckError::Empty => "Empty",
            CheckError::Overflow => "Overflow",
            CheckError::Unknown => "UnknownError",
        }
    }
}