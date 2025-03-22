use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromPrimitive)]
pub enum TextureType {
    Undef = 0,
    Undefined = 1, // TEXTURETYPE_UNDEFINED
    TwoD = 2,      // TEXTURETYPE_2D
    ThreeD = 3,    // TEXTURETYPE_3D
    Cube = 4,      // TEXTURETYPE_CUBE
}
