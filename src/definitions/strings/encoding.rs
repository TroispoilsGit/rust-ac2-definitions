pub enum EncodingType {
    Utf8,
    Ascii,
}

pub struct Encoding {
    pub encoding: EncodingType,
    pub max_bytes_per_char: usize,
}

impl Encoding {
    pub fn new(encoding: EncodingType) -> Self {
        let max_bytes_per_char = match encoding {
            EncodingType::Utf8 => 4,
            EncodingType::Ascii => 1,
        };

        Encoding {
            encoding,
            max_bytes_per_char,
        }
    }

    pub fn max_bytes_per_char(&self) -> usize {
        self.max_bytes_per_char
    }

    pub(crate) fn decode(&self, buffer: &[u8]) -> String {
        match self.encoding {
            EncodingType::Utf8 => String::from_utf8_lossy(buffer).to_string(),
            EncodingType::Ascii => String::from_utf8_lossy(buffer).to_string(),
        }
    }
}
