// from_le_bytes version for u128 (up to 16 bytes)
#[inline]
#[must_use]
pub(crate) fn le_bytes_to_u128(input: &[u8]) -> u128 {
    let mut bytes = [0u8; 16];
    let start = 16 - input.len();
    bytes[start..].copy_from_slice(input);
    u128::from_le_bytes(bytes)
}

// from_le_bytes version for u64 (up to 8 bytes)
#[inline]
#[must_use]
pub(crate) fn le_bytes_to_u64(input: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    let start = 8 - input.len();
    bytes[start..].copy_from_slice(input);
    u64::from_le_bytes(bytes)
}

// from_le_bytes version for u32 (up to 4 bytes)
#[inline]
#[must_use]
pub(crate) fn le_bytes_to_u32(input: &[u8]) -> u32 {
    let mut bytes = [0u8; 4];
    let start = 4 - input.len();
    bytes[start..].copy_from_slice(input);
    u32::from_le_bytes(bytes)
}

// from_le_bytes version for u16 (up to 2 bytes)
#[inline]
#[must_use]
pub(crate) fn le_bytes_to_u16(input: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    let start = 2 - input.len();
    bytes[start..].copy_from_slice(input);
    u16::from_le_bytes(bytes)
}