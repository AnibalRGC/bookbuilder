pub const UDP_DEST_PORT: u16 = 26477;
pub const UDP_DEST_PORT_OFFSET: usize = 36;
pub const PACKET_HEADER_OFFSET: usize = 42;
pub const PACKET_HEADER_LENGTH: usize = 20;

pub fn as_u64(bytes: &[u8]) -> u64 {
    (bytes[7] as u64)
        + ((bytes[6] as u64) << 8)
        + ((bytes[5] as u64) << 16)
        + ((bytes[4] as u64) << 24)
        + ((bytes[3] as u64) << 32)
        + ((bytes[2] as u64) << 40)
        + ((bytes[1] as u64) << 48)
        + ((bytes[0] as u64) << 56)
}

pub fn as_u48(bytes: &[u8]) -> u64 {
    (bytes[5] as u64)
        + ((bytes[4] as u64) << 8)
        + ((bytes[3] as u64) << 16)
        + ((bytes[2] as u64) << 24)
        + ((bytes[1] as u64) << 32)
        + ((bytes[0] as u64) << 40)
}

pub fn as_u32(bytes: &[u8]) -> u32 {
    (bytes[3] as u32)
        + ((bytes[2] as u32) << 8)
        + ((bytes[1] as u32) << 16)
        + ((bytes[0] as u32) << 24)
}

pub fn as_u16(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) + bytes[1] as u16
}
