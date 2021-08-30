use std::io::{Read, Write};
use std::io::Result;

pub trait BlockCopy {
    // Number of bytes to copy per block
    fn bytes_per_block(&self) -> u64;

    //Function that copies a block from one reader to a writer (using buffer), and returns the number of bytes copied
    fn copy_block<R: Read, W: Write>(&self, reader: &mut R, writer: &mut W, buffer: & mut [u8]) -> Result<u64>;
}

pub struct BufferBlockCopy;

impl BlockCopy for BufferBlockCopy {
    fn bytes_per_block(&self) -> u64 {
        8 * 1024
    }

    fn copy_block<R: Read, W: Write>(&self, reader: &mut R, writer: &mut W, buffer: & mut [u8]) -> Result<u64> {

        //Copy from reader into buffer
        let len = reader.read(buffer)?;

        //Copy from buffer into reader
        writer.write_all(buffer)?;

        Ok(len as u64)
    }
}