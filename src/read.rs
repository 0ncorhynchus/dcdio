use std::io::{Read, Cursor, Seek, SeekFrom};
use byteorder::ReadBytesExt;
use ::error::*;
use ::unformatted::ReadUnformattedExt;
use ::{Endian, Frame, DcdHeader};

pub struct DcdReader<R> {
    inner: R,
    index: usize,
    pub header: DcdHeader,
}

impl<R: Read> DcdReader<R> {
    pub fn new(mut reader: R) -> Result<Self> {
        let header = read_header(&mut reader)?;
        Ok(DcdReader {
            inner: reader,
            index: 0,
            header: header,
        })
    }

    pub fn read_frame(&mut self) -> Result<Frame> {
        let mut bufx = Cursor::new(self.inner.read_unformatted::<Endian>()?);
        let mut bufy = Cursor::new(self.inner.read_unformatted::<Endian>()?);
        let mut bufz = Cursor::new(self.inner.read_unformatted::<Endian>()?);

        let mut positions = Vec::new();
        for _ in 0..self.header.num_atoms {
            positions.push((bufx.read_f32::<Endian>()?,
                            bufy.read_f32::<Endian>()?,
                            bufz.read_f32::<Endian>()?));
        }

        let step = self.header.step_interval * self.index as i32
                   + self.header.initial_step;
        let time = self.header.delta * step as f32;
        self.index += 1;

        Ok(Frame::new(step, time, positions))
    }

    pub fn frames(self) -> DcdFrames<R> {
        DcdFrames {
            reader: self
        }
    }
}

fn read_header<R: Read + ?Sized>(reader: &mut R) -> Result<DcdHeader> {
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

    let title = {
        let mut buf = Cursor::new(reader.read_unformatted::<Endian>()?);
        let num_titles = buf.read_i32::<Endian>()? as usize;
        let mut lines = vec![0u8; num_titles * 80];
        buf.read(&mut lines)?;
        String::from_utf8(lines)?
    };

    let num_atoms = {
        let mut buf = Cursor::new(reader.read_unformatted::<Endian>()?);
        buf.read_i32::<Endian>()? as usize
    };

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
