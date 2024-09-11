#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonDecimal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverFlow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionOverflow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtractionOverflow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    NonDecimal(NonDecimal),
    Empty(Empty),
    OverFlow(OverFlow),
    AdditionOverflow(AdditionOverflow),
    SubtractionOverflow(SubtractionOverflow),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::NonDecimal(_) => write!(f, "NonDecimal"),
            CheckError::Empty(_) => write!(f, "Empty"),
            CheckError::OverFlow(_) => write!(f, "OverFlow"),
            CheckError::AdditionOverflow(_) => write!(f, "AdditionOverflow"),
            CheckError::SubtractionOverflow(_) => write!(f, "SubtractionOverflow"),
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