use std::io::{Read, Result, Error, ErrorKind};
use byteorder::{ReadBytesExt, LittleEndian};

pub trait ReadUnformattedExt : Read {
    fn read_unformatted(&mut self) -> Result<Vec<u8>>;
}

impl<R: Read + ?Sized> ReadUnformattedExt for R {
    fn read_unformatted(&mut self) -> Result<Vec<u8>> {
        let bytes = self.read_i32::<LittleEndian>()?;

        let mut buf = vec![0u8; bytes as usize];
        self.read(&mut buf)?;

        if self.read_i32::<LittleEndian>()? == bytes {
            Ok(buf)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Consistency check failed."))
        }
    }
}
