
use std::io::{Read, Write, Result};

pub struct NullStream;

impl Read for NullStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for byte in buf.iter_mut() {
            *byte = 0;
        }

        Ok(buf.len())
    }
}

impl Write for NullStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
