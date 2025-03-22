use std::{
    collections::HashMap,
    io::{self, Cursor, Read, Seek, SeekFrom},
};

use num_traits::FromPrimitive;

use crate::{
    crypto::ac2_crypto::AC2Crypto,
    strings::encoding::{Encoding, EncodingType},
    types::{
        cell_id::CellId, data_id::DataId, local_cell_id::LocalCellId, matrix4x4::Matrix4x4,
        quaternion::Quaternion, rgba_color::RGBAColor, string_id::StringId, vector3::Vector3,
    },
};

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

    pub fn read_cellid(&mut self) -> io::Result<CellId> {
        Ok(CellId::new(self.read_u32()?))
    }

    pub fn read_local_cellid(&mut self) -> io::Result<LocalCellId> {
        Ok(LocalCellId::new(self.read_u16()?))
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0; 8];
        self.cursor.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    pub fn read_dataid(&mut self) -> io::Result<DataId> {
        Ok(DataId::new(self.read_u32()?))
    }
    pub fn read_stringid(&mut self) -> io::Result<StringId> {
        Ok(StringId::new(self.read_u32()?))
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

        //println!("[DEBUG] Count list: {}", num_elements);

        list.reserve(num_elements as usize);
        for _ in 0..num_elements {
            list.push(element_reader(self)?);
        }
        Ok(())
    }

    pub fn read_enum<T>(&mut self) -> io::Result<T>
    where
        T: FromPrimitive,
    {
        let value = self.read_u32()?;
        match T::from_u32(value) {
            Some(variant) => Ok(variant),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid enum value: {}", value),
            )),
        }
    }

    pub fn read_enum16<T>(&mut self) -> io::Result<T>
    where
        T: FromPrimitive,
    {
        let value = self.read_u16()?;
        match T::from_u16(value) {
            Some(variant) => Ok(variant),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid enum value: {}", value),
            )),
        }
    }

    pub fn read_enum_with_default<T>(&mut self, default: T) -> io::Result<T>
    where
        T: FromPrimitive,
    {
        let value = self.read_u32()?;
        Ok(T::from_u32(value).unwrap_or(default))
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

    pub fn read_vector3(&mut self) -> io::Result<Vector3> {
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let z = self.read_f32()?;
        Ok(Vector3::new(x, y, z))
    }

    pub fn read_f32(&mut self) -> io::Result<f32> {
        let mut buffer = [0; 4];
        self.cursor.read_exact(&mut buffer)?;
        Ok(f32::from_le_bytes(buffer))
    }

    pub fn read_f64(&mut self) -> io::Result<f64> {
        let mut buffer = [0; 8];
        self.cursor.read_exact(&mut buffer)?;
        Ok(f64::from_le_bytes(buffer))
    }

    pub fn read_quaternion(&mut self) -> io::Result<Quaternion> {
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let z = self.read_f32()?;
        let w = self.read_f32()?;
        Ok(Quaternion::new(x, y, z, w))
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        let mut buffer = [0; 4];
        self.cursor.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }

    pub fn read_color(&mut self) -> io::Result<RGBAColor> {
        let r = self.read_u8()?;
        let g = self.read_u8()?;
        let b = self.read_u8()?;
        let a = self.read_u8()?;
        Ok(RGBAColor::new(r, g, b, a))
    }

    pub fn read_string(&mut self, encoding: Encoding) -> io::Result<String> {
        let num_char = self.read_u16().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to read num_char: {}", e),
            )
        })?;

        if num_char == 0 {
            self.align(4);
            return Ok(String::new());
        }

        let length = encoding.max_bytes_per_char() * num_char as usize;
        let mut buffer = self.read_bytes(length).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to read bytes: {}", e),
            )
        })?;

        self.align(4);
        AC2Crypto::decrypt(&mut buffer, 0, length);

        Ok(encoding.decode(&buffer))
    }

    pub fn read_bytes(&mut self, length: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer).map_err(|e| {
            io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("Failed to read {} bytes: {}", length, e),
            )
        })?;
        Ok(buffer)
    }

    pub fn read_i64(&mut self) -> io::Result<i64> {
        let mut buffer = [0; 8];
        self.cursor.read_exact(&mut buffer)?;
        Ok(i64::from_le_bytes(buffer))
    }

    pub fn align(&mut self, bytes: u64) {
        let align_delta = self.cursor.position() % bytes;
        if align_delta != 0 {
            self.cursor
                .seek(SeekFrom::Current(bytes as i64 - align_delta as i64))
                .expect("Issue with align");
        }
    }

    pub fn read_string_map(&mut self) -> io::Result<HashMap<u32, String>> {
        let num_entries = self.read_u16().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to read num_entries: {}", e),
            )
        })?;
        let _table_size = self.read_u16().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to read table_size: {}", e),
            )
        })?;
        let mut map = HashMap::with_capacity(num_entries as usize);

        for _ in 0..num_entries {
            let key = self.read_u32().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to read key: {}", e),
                )
            })?;
            let value = self
                .read_string(Encoding::new(EncodingType::Ascii))
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to read string value: {}", e),
                    )
                })?;
            map.insert(key, value);
        }

        Ok(map)
    }

    pub fn read_matrix4x4(&mut self) -> io::Result<Matrix4x4> {
        Ok(Matrix4x4 {
            data: [
                [
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                ],
                [
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                ],
                [
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                ],
                [
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                    self.read_f32()?,
                ],
            ],
        })
    }

    pub fn read_dictionary<K, V>(
        &mut self,
        mut key_reader: impl FnMut(&mut Self) -> io::Result<K>,
        mut value_reader: impl FnMut(&mut Self) -> io::Result<V>,
    ) -> io::Result<HashMap<K, V>>
    where
        K: std::hash::Hash + Eq,
    {
        let count = self
            .read_u16()
            .expect("Erreur lors de la lecture de 'count'");
        let _ = self.read_u16().expect("table size issue");
        let mut dictionary = HashMap::with_capacity(count as usize);

        for _ in 0..count {
            let key = key_reader(self).expect("[read_dictionary] key_reader issue");
            let value = value_reader(self).expect("[read_dictionary] value_reader issue");
            dictionary.entry(key).insert_entry(value);
        }

        Ok(dictionary)
    }

    pub fn read_multi_dictionary<K, V>(
        &mut self,
        mut key_reader: impl FnMut(&mut Self) -> io::Result<K>,
        mut value_reader: impl FnMut(&mut Self) -> io::Result<V>,
    ) -> io::Result<HashMap<K, Vec<V>>>
    where
        K: std::hash::Hash + Eq,
    {
        let count = self
            .read_u16()
            .expect("Erreur lors de la lecture de 'count'");
        let _ = self.read_u16().expect("table size issue");
        let mut dictionary = HashMap::with_capacity(count as usize);

        for _ in 0..count {
            let key = key_reader(self).expect("[read_multi_dictionary] key_reader issue");
            let value = self
                .read_list(|d| value_reader(d), 4)
                .expect("[read_multi_dictionary] self.read_list(|d| value_reader(d), 4) issue");
            dictionary.entry(key).insert_entry(value);
        }

        Ok(dictionary)
    }
}
