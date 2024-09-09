pub mod unsigned;
pub mod little_endian;
pub mod float;
pub mod integer;

/// Parser for decimal notation
/// It can not parse scientific notation
#[derive(Debug, Clone, Copy, Default)]
pub struct BiscuitParser {
    fraction_length: Option<usize>,
}

impl BiscuitParser {
    pub fn initialize() -> BiscuitParser {
        BiscuitParser {
            fraction_length: None,
        }
    }

    pub fn new(fraction_length: usize) -> Self {
        Self {
            fraction_length: Some(fraction_length),
        }
    }

    pub fn with_fraction_length(mut self, fraction_length: usize) -> Self {
        self.fraction_length = Some(fraction_length);
        self
    }
}