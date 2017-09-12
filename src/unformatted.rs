use std::io::{Read, Result, Error, ErrorKind};
use byteorder::{ReadBytesExt, ByteOrder};

pub trait ReadUnformattedExt : Read {
    fn read_unformatted<T: ByteOrder>(&mut self) -> Result<Vec<u8>>;
}

impl<R: Read + ?Sized> ReadUnformattedExt for R {
    fn read_unformatted<T: ByteOrder>(&mut self) -> Result<Vec<u8>> {
        let bytes = self.read_i32::<T>()?;

        let mut buf = vec![0u8; bytes as usize];
        self.read(&mut buf)?;

        if self.read_i32::<T>()? == bytes {
            Ok(buf)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Consistency check failed."))
        }
    }
}
