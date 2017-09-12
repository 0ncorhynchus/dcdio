extern crate byteorder;
mod unformatted;
mod error;
mod read;

use std::io::{Read, Cursor, SeekFrom};
use std::io::prelude::*;
use byteorder::*;
use unformatted::ReadUnformattedExt;
use error::*;

pub use read::{DcdReader, DcdFrames};

type Endian = NativeEndian;
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

impl DcdHeader {
    pub fn load<R: Read + ?Sized> (reader: &mut R) -> Result<Self> {
        let mut buf = Cursor::new(reader.read_unformatted::<Endian>()?);
        buf.seek(SeekFrom::Current(4))?;

        let num_frames    = buf.read_i32::<Endian>()? as usize;
        let initial_step  = buf.read_i32::<Endian>()?;
        let step_interval = buf.read_i32::<Endian>()?;

        buf.seek(SeekFrom::Current(20))?;

        let num_fixed_atoms = buf.read_i32::<Endian>()? as usize;
        if num_fixed_atoms != 0 {
            return Err(Error::NotSupported(
                    "Fixed atoms are not supported.".to_string()));
        }

        let delta = buf.read_f32::<Endian>()?;

        buf.seek(SeekFrom::Current(36))?;

        let version = buf.read_i32::<Endian>()?;

        buf = Cursor::new(reader.read_unformatted::<Endian>()?);
        let num_titles = buf.read_i32::<Endian>()? as usize;
        let mut lines = vec![0u8; num_titles * 80];
        buf.read(&mut lines)?;
        let title = String::from_utf8(lines)?;

        buf = Cursor::new(reader.read_unformatted::<Endian>()?);
        let num_atoms = buf.read_i32::<Endian>()? as usize;

        Ok(DcdHeader {
            num_frames:     num_frames,
            initial_step:   initial_step,
            step_interval:  step_interval,
            num_fixed_atoms: num_fixed_atoms,
            delta:           delta,
            version:         version,
            num_atoms:       num_atoms,
            title:           title,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
