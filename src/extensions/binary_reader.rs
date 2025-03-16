use std::io::{self, Cursor, Read, Seek};

pub struct BinaryReader {
    cursor: Cursor<Vec<u8>>,
}

impl BinaryReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn read_all(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.cursor.read_to_end(&mut buffer).unwrap();
        buffer
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        self.cursor.read_exact(buffer)
    }

    pub fn seek(&mut self, pos: io::SeekFrom) -> Result<(), Box<dyn std::error::Error>> {
        self.cursor.seek(pos)?;
        Ok(())
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.cursor.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        self.cursor.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    pub fn read_bool(&mut self) -> io::Result<bool> {
        let buffer = self.read_u32()?;
        Ok(buffer != 0)
    }

    pub fn file_to_vec_u3_to_string(&mut self) -> String {
        self.seek(io::SeekFrom::Start(0)).expect("seek issue");
        // Return a String of file (uint hex per line)
        let mut result = String::new();
        while let Ok(value) = self.read_u32() {
            result.push_str(&format!("{:08x}\n", value));
        }
        result
    }
}
