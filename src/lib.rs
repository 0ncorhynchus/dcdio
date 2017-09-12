extern crate byteorder;
mod unformatted;
mod error;
mod read;
mod write;

pub use ::read::{DcdReader, DcdFrames};
pub use ::write::DcdWriter;

type Endian = byteorder::NativeEndian;
type Position = (f32, f32, f32);

pub struct Frame {
    pub step: i32,
    pub time: f32,
    pub positions: Vec<Position>,
}

impl Frame {
    pub fn new(step: i32, time: f32, positions: Vec<Position>) -> Self {
        Frame {
            step: step,
            time: time,
            positions: positions,
        }
    }
}

pub struct DcdHeader {
    pub num_frames: usize,
    pub initial_step: i32,
    pub step_interval: i32,
    pub num_fixed_atoms: usize,
    pub delta: f32,
    pub version: i32,
    pub num_atoms: usize,
    pub title: String
}

