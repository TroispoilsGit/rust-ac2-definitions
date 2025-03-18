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

    pub fn read_list<T>(&mut self) -> io::Result<Vec<T>>
    where
        T: TryFrom<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64>,
        <T as TryFrom<u8>>::Error: std::fmt::Debug,
        <T as TryFrom<u16>>::Error: std::fmt::Debug,
        <T as TryFrom<u32>>::Error: std::fmt::Debug,
        <T as TryFrom<u64>>::Error: std::fmt::Debug,
    {
        let size_list = self.read_u32()?;
        let mut list = Vec::with_capacity(size_list as usize);

        for _ in 0..size_list {
            let value: T = match std::any::type_name::<T>() {
                "u8" => self.read_u8()?.try_into().expect("Conversion failed"),
                "u16" => self.read_u16()?.try_into().expect("Conversion failed"),
                "u32" => self.read_u32()?.try_into().expect("Conversion failed"),
                "u64" => self.read_u64()?.try_into().expect("Conversion failed"),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unsupported type",
                    ));
                }
            };

            list.push(value);
        }

        Ok(list)
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
