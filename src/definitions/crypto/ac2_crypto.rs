pub struct AC2Crypto;

impl AC2Crypto {
    const CRYPTO_KEY: [u8; 64] = [
        0x38, 0x92, 0x84, 0xE5, 0x7D, 0x76, 0x2D, 0xEB, 0x03, 0xDD, 0x0F, 0xFC, 0xA6, 0xA6, 0x45,
        0xCB, 0xBB, 0xE6, 0x21, 0x7F, 0xC8, 0xCD, 0xA6, 0xF5, 0x12, 0xAD, 0xD9, 0x22, 0xDE, 0xD4,
        0xA3, 0xB2, 0xB7, 0x4D, 0x83, 0x3B, 0x1C, 0xC9, 0x06, 0xC4, 0x5E, 0x30, 0x92, 0x83, 0x99,
        0x00, 0x9F, 0x0A, 0x0C, 0xB9, 0x43, 0x09, 0x3C, 0x95, 0x6D, 0xA6, 0x1F, 0x9E, 0x92, 0x0D,
        0xD4, 0x9A, 0x2E, 0x0E,
    ];

    pub fn calc_checksum(buffer: &[u8], offset: usize, size: usize, include_size: bool) -> u32 {
        let mut checksum = if include_size { (size as u32) << 16 } else { 0 };

        let size_leftover = size % 4;
        let mut shift = 0;

        for i in offset..(offset + size - size_leftover) {
            checksum = checksum.wrapping_add((buffer[i] as u32) << shift);
            shift += 8;
            if shift > 24 {
                shift = 0;
            }
        }

        shift = 24;
        for i in (offset + size - size_leftover)..(offset + size) {
            checksum = checksum.wrapping_add((buffer[i] as u32) << shift);
            shift -= 8;
        }

        checksum
    }

    pub fn encrypt(buffer: &mut [u8], offset: usize, size: usize) {
        Self::swap_nibbles(buffer, offset, size);
        Self::xor_with_key(buffer, offset, size, &Self::CRYPTO_KEY);
    }

    pub fn decrypt(buffer: &mut [u8], offset: usize, size: usize) {
        Self::xor_with_key(buffer, offset, size, &Self::CRYPTO_KEY);
        Self::swap_nibbles(buffer, offset, size);
    }

    fn swap_nibbles(buffer: &mut [u8], offset: usize, size: usize) {
        for i in offset..(offset + size) {
            buffer[i] = (buffer[i] >> 4) | (buffer[i] << 4);
        }
    }

    fn xor_with_key(buffer: &mut [u8], offset: usize, size: usize, key: &[u8]) {
        for i in offset..(offset + size - (size % 4)) {
            buffer[i] ^= key[i % key.len()];
        }
    }
}
