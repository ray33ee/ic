
use std::io::Result;
use std::io::{Read, Write};
use crate::messages::{Command, Status, Shared};
use std::thread::sleep;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use crate::blockcopy::{BufferBlockCopy, BlockCopy};
use std::time::Duration;
use std::ops::DerefMut;

pub struct Copier {
    shared: Arc<Mutex<Shared>>,
    paused: bool,
}

impl Copier {
    pub fn new(shared: Arc<Mutex<Shared>>) -> Self {
        Self {
            shared,
            paused: false,
        }
    }

    //A buffered copy from reader to writer.
    //Similar to std::io::copy, but after every block, check for pause/play/stop, and report back the progress
    pub fn copy<R: Read, W: Write, B: BlockCopy>(& mut self, reader: &mut R, writer: &mut W, block_copier: & B) -> Result<u64> {
        let mut buffer = vec![0u8; block_copier.bytes_per_block() as usize];

        let mut written = 0;

        loop {

            //If we are paused, sleep for a bit
            if self.paused {
                //Put the thread to sleep for 100ms
                sleep(Duration::new(0, 1000 * 1000 * 100));
            } else {

                //Copy a block. If the copy returns an error, stop and return the error
                let len = match block_copier.copy_block(reader, writer, buffer.borrow_mut()) {
                    Ok(0) => return Ok(written),
                    Ok(len) => len,
                    Err(e) => return Err(e),
                };

                written += len;
            }

            //Acquire the lock and a reference to the shared data
            let mut lock = self.shared.lock();
            let mut shared = lock.as_mut().unwrap().deref_mut();

            //Check the 'command' field to see if a pause/stop/play has been initiated
            if shared.command.is_some() {

                match shared.command.as_ref().unwrap() {
                    Command::Pause => {
                        self.paused = true;
                    }
                    Command::Play => {
                        self.paused = false;
                    }
                    Command::Stop => {
                        return Ok(written);
                    }
                }

                shared.command = None;

            }

            //Report back the number of bytes written so far
            shared.status = Status::new(written, self.paused);

        }
    }
}
