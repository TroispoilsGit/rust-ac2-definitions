pub enum EncodingType {
    Utf8,
    Ascii,
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
        }
    }
}
