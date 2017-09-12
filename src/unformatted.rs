use std::io::{Read, Write, Result, Error, ErrorKind};
use byteorder::{ReadBytesExt, WriteBytesExt, ByteOrder};

pub trait ReadUnformattedExt : Read {
    fn read_unformatted<T: ByteOrder>(&mut self) -> Result<Vec<u8>>;
}

impl<R: Read + ?Sized> ReadUnformattedExt for R {
    fn read_unformatted<T: ByteOrder>(&mut self) -> Result<Vec<u8>> {
        let bytes = self.read_i32::<T>()?;

        let mut buf = vec![0u8; bytes as usize];
        self.read_exact(&mut buf)?;

        if self.read_i32::<T>()? == bytes {
            Ok(buf)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Consistency check failed."))
        }
    }
}

pub trait WriteUnformattedExt : Write {
    fn write_unformatted<T: ByteOrder>(&mut self, buf: &[u8]) -> Result<()>;
}

impl<W: Write + ?Sized> WriteUnformattedExt for W {
    fn write_unformatted<T: ByteOrder>(&mut self, buf: &[u8]) -> Result<()> {
        let size = buf.len() as i32;
        self.write_i32::<T>(size)?;
        self.write_all(buf)?;
        self.write_i32::<T>(size)?;
        self.flush()
    }
}
