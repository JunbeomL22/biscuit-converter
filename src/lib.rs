pub mod unsigned;
pub mod little_endian;
pub mod arithematics;
pub mod integer;
pub mod float;
pub mod integer_string;

/// Parser for decimal notation
/// It can not parse scientific notation
#[derive(Debug, Clone, Copy)]
pub struct FlashParser {
    // if it is given for a known format, it can parse faster
    // when it is not given, it will find the length and parse the number
    fraction_length: Option<usize>, 
    u64_buffer: [u8; 64],
    u32_buffer: [u8; 32],
    i64_buffer: [u8; 64],
    i32_buffer: [u8; 32],
}

const END_MARKER: u8 = 255;
impl Default for FlashParser {
    fn default() -> Self {
        Self {
            fraction_length: None,
            u64_buffer: [END_MARKER; 64],
            u32_buffer: [END_MARKER; 32],
            i64_buffer: [END_MARKER; 64],
            i32_buffer: [END_MARKER; 32],
        }
    }
}

impl FlashParser {
    pub fn new(fraction_length: usize) -> Self {
        Self {
            fraction_length: Some(fraction_length),
            u64_buffer: [END_MARKER; 64],
            u32_buffer: [END_MARKER; 32],
            i64_buffer: [END_MARKER; 64],
            i32_buffer: [END_MARKER; 32],
        }
    }

    pub fn with_fraction_length(&mut self, fraction_length: usize) -> &mut Self {
        self.fraction_length = Some(fraction_length);
        self
    }

  
}