extern crate byteorder;
mod unformatted;
mod error;

use std::io::{Read, Cursor, SeekFrom};
use std::io::prelude::*;
use byteorder::*;
use unformatted::ReadUnformattedExt;
use error::*;

pub struct Frame {
    pub step: i32,
    pub time: f32,
    pub positions: Vec<(f32, f32, f32)>,
}

impl Frame {
    pub fn new(step: i32, time: f32, positions: Vec<(f32, f32, f32)>) -> Self {
        Frame {
            step: step,
            time: time,
            positions: positions,
        }
    }
}

struct DcdHeader {
    num_frames: usize,
    initial_step: i32,
    step_interval: i32,
    // num_fixed_atoms: usize,
    delta: f32,
    version: i32,
    num_atoms: usize,
    title: String
}

impl DcdHeader {
    pub fn load<R: Read + ?Sized> (reader: &mut R) -> Result<Self> {
        let mut buf = Cursor::new(reader.read_unformatted()?);
        buf.seek(SeekFrom::Current(4))?;

        let num_frames    = buf.read_i32::<LittleEndian>()? as usize;
        let initial_step  = buf.read_i32::<LittleEndian>()?;
        let step_interval = buf.read_i32::<LittleEndian>()?;

        buf.seek(SeekFrom::Current(20))?;

        let num_fixed_atoms = buf.read_i32::<LittleEndian>()? as usize;
        if num_fixed_atoms != 0 {
            return Err(Error::NotSupported(
                    "Fixed atoms are not supported.".to_string()));
        }

        let delta = buf.read_f32::<LittleEndian>()?;

        buf.seek(SeekFrom::Current(36))?;

        let version = buf.read_i32::<LittleEndian>()?;

        buf = Cursor::new(reader.read_unformatted()?);
        let num_atoms = buf.read_i32::<LittleEndian>()? as usize;

        buf = Cursor::new(reader.read_unformatted()?);
        let num_titles = buf.read_i32::<LittleEndian>()? as usize;
        let mut lines = vec![0u8; num_titles * 80];
        buf.read(&mut lines)?;
        let title = String::from_utf8(lines)?;

        Ok(DcdHeader {
            num_frames:     num_frames,
            initial_step:   initial_step,
            step_interval:  step_interval,
            // num_fixed_atoms: num_fixed_atoms,
            delta:           delta,
            version:         version,
            num_atoms:       num_atoms,
            title:           title,
        })
    }
}

pub struct DcdReader<R> {
    inner: R,
    next_step: i32,
    header: DcdHeader,
    // fixed_atoms: Vec<(f32, f32, f32)>,
}

impl<R: Read> DcdReader<R> {
    pub fn new(mut reader: R) -> Result<Self> {
        let header = DcdHeader::load(&mut reader)?;
        // let fixed_atoms = vec![(0.0, 0.0, 0.0); header.num_fixed_atoms];
        Ok(DcdReader {
            inner: reader,
            next_step: header.initial_step,
            header: header,
            // fixed_atoms: fixed_atoms,
        })
    }

    pub fn num_frames(&self) -> usize {
        self.header.num_frames
    }

    pub fn version(&self) -> i32 {
        self.header.version
    }

    pub fn title(&self) -> String {
        self.header.title.clone()
    }

    pub fn read_frame(&mut self) -> Result<Frame> {
        let num_atoms = self.header.num_atoms;

        let mut bufx = Cursor::new(self.inner.read_unformatted()?);
        let mut bufy = Cursor::new(self.inner.read_unformatted()?);
        let mut bufz = Cursor::new(self.inner.read_unformatted()?);

        let positions = vec![
            (bufx.read_f32::<LittleEndian>()?,
             bufy.read_f32::<LittleEndian>()?,
             bufz.read_f32::<LittleEndian>()?)
            ; num_atoms
        ];

        let step = self.next_step;
        self.next_step += self.header.step_interval;

        Ok(Frame::new(step, self.header.delta * step as f32, positions))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
