use std::io::{Read, Cursor};
use unformatted::ReadUnformattedExt;
use byteorder::ReadBytesExt;
use ::error::*;
use ::{Endian, Frame, DcdHeader};

pub struct DcdReader<R> {
    inner: R,
    index: usize,
    // fixed_atoms: Vec<(f32, f32, f32)>,
    pub header: DcdHeader,
}

impl<R: Read> DcdReader<R> {
    pub fn new(mut reader: R) -> Result<Self> {
        let header = DcdHeader::load(&mut reader)?;
        // let fixed_atoms = vec![(0.0, 0.0, 0.0); header.num_fixed_atoms];
        Ok(DcdReader {
            inner: reader,
            index: 0,
            // fixed_atoms: fixed_atoms,
            header: header,
        })
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

        let step = self.header.step_interval * self.index as i32
                   + self.header.initial_step;
        self.index += 1;

        Ok(Frame::new(step, self.header.delta * step as f32, positions))
    }

    pub fn frames(self) -> DcdFrames<R> {
        DcdFrames {
            reader: self
        }
    }
}

pub struct DcdFrames<R> {
    reader: DcdReader<R>,
}

impl<R: Read> Iterator for DcdFrames<R> {
    type Item = Result<Frame>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.index < self.reader.header.num_frames {
            Some(self.reader.read_frame())
        } else {
            None
        }
    }
}
