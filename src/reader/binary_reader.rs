use std::io::{self, Cursor, Read, Seek};

use crate::types::data_id::DataId;

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

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0; 1];
        self.cursor.read_exact(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.cursor.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0; 8];
        self.cursor.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    pub fn read_dataid(&mut self) -> io::Result<DataId> {
        Ok(DataId::new(self.read_u32()?))
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

    pub fn read_list<T, F>(
        &mut self,
        mut element_reader: F,
        size_of_size: u32,
    ) -> Result<Vec<T>, io::Error>
    where
        F: FnMut(&mut Self) -> Result<T, io::Error>,
    {
        let mut list = Vec::new();
        self.read_list_into(&mut list, &mut element_reader, size_of_size)?;
        Ok(list)
    }

    fn read_list_into<T, F>(
        &mut self,
        list: &mut Vec<T>,
        element_reader: &mut F,
        size_of_size: u32,
    ) -> Result<(), io::Error>
    where
        F: FnMut(&mut Self) -> Result<T, io::Error>,
    {
        let num_elements = match size_of_size {
            1 => self.read_u8()? as u32,
            2 => self.read_u16()? as u32,
            4 => self.read_u32()?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid size_of_size",
                ));
            }
        };

        list.reserve(num_elements as usize);
        for _ in 0..num_elements {
            list.push(element_reader(self)?);
        }
        Ok(())
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
