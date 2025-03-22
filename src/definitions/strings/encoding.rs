pub enum EncodingType {
    Utf8,
    Ascii,
    Unicode,
}

pub struct Encoding {
    pub encoding: EncodingType,
}

impl Encoding {
    pub fn new(encoding: EncodingType) -> Self {
        Encoding { encoding }
    }

    pub fn max_bytes_per_char(&self) -> usize {
        match self.encoding {
            EncodingType::Utf8 => 4,
            EncodingType::Ascii => 1,
            EncodingType::Unicode => 2,
        }
    }

    pub fn decode(&self, buffer: &[u8]) -> String {
        match self.encoding {
            EncodingType::Utf8 => String::from_utf8_lossy(buffer).to_string(),
            EncodingType::Ascii => {
                if buffer.iter().all(|&b| b.is_ascii()) {
                    buffer.iter().map(|&b| b as char).collect()
                } else {
                    String::new()
                }
            }
            EncodingType::Unicode => String::from_utf16_lossy(
                buffer
                    .chunks(2)
                    .map(|chunk| {
                        if chunk.len() < 2 {
                            (chunk[0] as u16) | 0
                        } else {
                            (chunk[0] as u16) | ((chunk[1] as u16) << 8)
                        }
                    })
                    .collect::<Vec<u16>>()
                    .as_slice(),
            ),
        }
    }
}
