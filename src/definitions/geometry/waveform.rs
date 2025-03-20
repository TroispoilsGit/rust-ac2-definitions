use std::io;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

#[derive(Debug, FromPrimitive, Serialize, Deserialize)]
pub enum WaveformType {
    INVALID = 0,   // WAVEFORM_INVALID
    NONE = 1,      // WAVEFORM_NONE
    SPEED = 2,     // WAVEFORM_SPEED
    NOISE = 3,     // WAVEFORM_NOISE
    SINE = 4,      // WAVEFORM_SINE
    SQUARE = 5,    // WAVEFORM_SQUARE
    BOUNCE = 6,    // WAVEFORM_BOUNCE
    PERLIN = 7,    // WAVEFORM_PERLIN
    FRACTAL = 8,   // WAVEFORM_FRACTAL
    FRAMELOOP = 9, // WAVEFORM_FRAMELOOP
    NUM = 10,      // NUM_WAVEFORMS
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Waveform {
    pub types: WaveformType,
    pub base_value: f32,
    pub base_vel: f32,
    pub amplitude: f32,
    pub amplitude_vel: f32,
    pub phase: f32,
    pub phase_vel: f32,
    pub frequency: f32,
    pub frequency_vel: f32,
    pub roughness: f32,
    pub roughness_vel: f32,
}

impl Waveform {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let types = data.read_enum::<WaveformType>()?;
        let base_value = data.read_f32()?;
        let base_vel = data.read_f32()?;
        let amplitude = data.read_f32()?;
        let amplitude_vel = data.read_f32()?;
        let phase = data.read_f32()?;
        let phase_vel = data.read_f32()?;
        let frequency = data.read_f32()?;
        let frequency_vel = data.read_f32()?;
        let roughness = data.read_f32()?;
        let roughness_vel = data.read_f32()?;

        Ok(Waveform {
            types,
            base_value,
            base_vel,
            amplitude,
            amplitude_vel,
            phase,
            phase_vel,
            frequency,
            frequency_vel,
            roughness,
            roughness_vel,
        })
    }
}
