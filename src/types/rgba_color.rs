#[derive(Debug)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBAColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBAColor { r, g, b, a }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    pub fn from_hex(hex: &str) -> Result<Self, std::num::ParseIntError> {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        let a = u8::from_str_radix(&hex[6..8], 16)?;
        Ok(RGBAColor { r, g, b, a })
    }
}
