use std::io::{Write, Cursor};
use byteorder::WriteBytesExt;
use ::error::*;
use ::unformatted::WriteUnformattedExt;
use ::{Endian, DcdHeader, Position};

pub struct DcdWriter<W> {
    writer: W,
    // index: usize,
    // header: DcdHeader,
}

impl<W: Write> DcdWriter<W> {
    pub fn new(mut writer: W, header: DcdHeader) -> Result<Self> {
        write_header(&mut writer, &header)?;
        Ok(DcdWriter {
            writer: writer,
            // index: 0,
            // header: header,
        })
    }

    pub fn write_frame(&mut self, positions: &[Position]) -> Result<()> {
        let mut bufx = Cursor::new(Vec::new());
        let mut bufy = Cursor::new(Vec::new());
        let mut bufz = Cursor::new(Vec::new());

        // assert(positions.len() == self.header.num_atoms);

        for &(x, y, z) in positions {
            bufx.write_f32::<Endian>(x)?;
            bufy.write_f32::<Endian>(y)?;
            bufz.write_f32::<Endian>(z)?;
        }

        self.writer.write_unformatted::<Endian>(bufx.get_ref())?;
        self.writer.write_unformatted::<Endian>(bufy.get_ref())?;
        self.writer.write_unformatted::<Endian>(bufz.get_ref())?;

        Ok(())
    }
}

fn write_header<W: Write + ?Sized>(writer: &mut W, header: &DcdHeader) -> Result<()> {
    {
        let mut buff = Cursor::new(Vec::new());

        buff.write(b"CORD")?;
        buff.write_i32::<Endian>(header.num_frames as i32)?;
        buff.write_i32::<Endian>(header.initial_step)?;
        buff.write_i32::<Endian>(header.step_interval)?;

        buff.write(&[0u8; 20])?;

        buff.write_i32::<Endian>(header.num_fixed_atoms as i32)?;
        buff.write_f32::<Endian>(header.delta)?;

        buff.write(&[0u8; 36])?;

        buff.write_i32::<Endian>(header.version)?;

        buff.flush()?;

        writer.write_unformatted::<Endian>(buff.get_ref().as_ref())?;
    }

    {
        let mut buff = Cursor::new(Vec::new());
        let num_titles = header.title.len() / 80;
        buff.write_i32::<Endian>(num_titles as i32)?;
        for i in 0..num_titles {
            buff.write(&header.title.as_bytes()[i*80..(i+1)*80])?;
        }
        buff.flush()?;
        writer.write_unformatted::<Endian>(buff.get_ref().as_ref())?;
    }

    {
        let mut buff = Cursor::new(Vec::new());
        buff.write_i32::<Endian>(header.num_atoms as i32)?;
        buff.flush()?;
        writer.write_unformatted::<Endian>(buff.get_ref().as_ref())?;
    }

    Ok(())
}
