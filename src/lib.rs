extern crate byteorder;

use std::io::Read;
use byteorder::*;

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

pub struct DCDReader<R: Read> {
    inner: R,
    is_initialized: bool,
    num_frames: usize,
    initial_step: i32,
    step_interval: i32,
    num_fixed_atoms: usize,
    num_atoms: usize,
    delta: f32,
}

impl<R: Read> DCDReader<R> {
    pub fn new(reader: R) -> Self {
        DCDReader {
            inner: reader,
            is_initialized: false,
            num_frames: 0,
            initial_step: 0,
            step_interval: 0,
            num_fixed_atoms: 0,
            num_atoms: 0,
            delta: 0.0
        }
    }

    pub fn num_frames(&self) -> usize {
        self.num_frames
    }

    fn read_i32(&mut self) -> std::io::Result<i32> {
        self.inner.read_i32::<LittleEndian>()
    }

    fn read_f32(&mut self) -> std::io::Result<f32> {
        self.inner.read_f32::<LittleEndian>()
    }

    fn skip_bytes(&mut self, bytes: usize) -> std::io::Result<usize> {
        let mut buf = vec![0u8; bytes];
        self.inner.read(&mut buf)
    }

    pub fn read_header(&mut self) -> std::io::Result<()> {
        if self.is_initialized {
            return Ok(());
        }

        {
            let block_size = self.read_i32()?;
            self.skip_bytes(4)?;

            self.num_frames = self.read_i32()? as usize;
            self.initial_step = self.read_i32()?;
            self.step_interval = self.read_i32()?;

            self.skip_bytes(20)?;

            self.num_fixed_atoms = self.read_i32()? as usize;
            self.delta = self.read_f32()?;
            let _ = self.read_i32()?; // crystal

            self.skip_bytes(32)?;
            let _ = self.read_i32()?; // version

            // TODO
            assert_eq!(block_size, self.read_i32()?);
        }

        {
            let block_size = self.read_i32()?;
            self.num_atoms = self.read_i32()? as usize;
            // TODO
            assert_eq!(block_size, self.read_i32()?);
        }

        self.is_initialized = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
