#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseIntErr {
    NonDecimal,
    Empty,
    Overflow,
    NegOverflow,
    Unknown,
}

impl std::fmt::Display for ParseIntErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseIntErr::NonDecimal => write!(f, "NonDecimal"),
            ParseIntErr::Empty => write!(f, "Empty"),
            ParseIntErr::Overflow => write!(f, "Overflow"),
            ParseIntErr::NegOverflow => write!(f, "NegOverflow"),
            ParseIntErr::Unknown => write!(f, "UnknownError"),
        }
    }
}

impl std::error::Error for ParseIntErr {}

// String으로 변환
impl From<ParseIntErr> for String {
    fn from(error: ParseIntErr) -> Self {
        error.to_string()
    }
}

impl ParseIntErr {
    pub fn as_str(&self) -> &str {
        match self {
            ParseIntErr::NonDecimal => "NonDecimal",
            ParseIntErr::Empty => "Empty",
            ParseIntErr::Overflow => "Overflow",
            ParseIntErr::Unknown => "UnknownError",
            ParseIntErr::NegOverflow => "NegOverflow",
        }
    }
}