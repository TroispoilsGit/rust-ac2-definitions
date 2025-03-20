use std::io;

use serde::{Deserialize, Serialize};

use crate::{
    reader::binary_reader::BinaryReader,
    types::{quaternion::Quaternion, vector3::Vector3},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub pos: Vector3,
    pub rot: Quaternion,
}

impl Frame {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let pos = data.read_vector3()?;
        let rot = data.read_quaternion()?;

        Ok(Self { pos, rot })
    }
}
