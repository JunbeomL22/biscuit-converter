pub mod unsigned;
pub mod little_endian;
pub mod stubs;
pub mod integer;
pub mod float;
/// Parser for decimal notation
/// It can not parse scientific notation
#[derive(Debug, Clone, Copy)]
pub struct FlashParser {
    // if it is given for a known format, it can parse faster
    // when it is not given, it will find the length and parse the number
    fraction_length: Option<usize>, 
}

impl Default for FlashParser {
    fn default() -> Self {
        Self {
            fraction_length: None,
        }
    }
}

impl FlashParser {
    pub fn new(fraction_length: usize) -> Self {
        Self {
            fraction_length: Some(fraction_length),
        }
    }

    pub fn with_fraction_length(&mut self, fraction_length: usize) -> &mut Self {
        self.fraction_length = Some(fraction_length);
        self
    }

  
}