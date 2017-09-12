use std::io::{Read, Cursor};
use unformatted::ReadUnformattedExt;
use byteorder::ReadBytesExt;
use ::error::*;
use ::{Endian, Frame, DcdHeader};

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

        let mut bufx = Cursor::new(self.inner.read_unformatted::<Endian>()?);
        let mut bufy = Cursor::new(self.inner.read_unformatted::<Endian>()?);
        let mut bufz = Cursor::new(self.inner.read_unformatted::<Endian>()?);

        let mut positions = Vec::new();
        for _ in 0..num_atoms {
            positions.push((bufx.read_f32::<Endian>()?,
                            bufy.read_f32::<Endian>()?,
                            bufz.read_f32::<Endian>()?));
        }

        let step = self.next_step;
        self.next_step += self.header.step_interval;

        Ok(Frame::new(step, self.header.delta * step as f32, positions))
    }
}

