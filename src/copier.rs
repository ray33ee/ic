
use std::io::Result;
use std::io::{Read, Write};
use crate::messages::{GUICommand, Shared};
use std::thread::sleep;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use crate::blockcopy::{BlockCopy};
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

    pub fn paused(&self) -> bool {
        self.paused
    }

    //A buffered copy from reader to writer.
    //Similar to std::io::copy, but after every block, check for pause/play/stop, and report back the progress
    pub fn copy<R: Read, W: Write, B: BlockCopy>(
        & mut self,
        reader: &mut R,
        writer: &mut W,
        block_copier: & B,
        buffer: & mut [u8]) -> Result<(u64, bool)> {

        //If we are paused, sleep for a bit
        let (len, paused) = if self.paused {
            //Put the thread to sleep for 100ms
            sleep(Duration::new(0, 1000 * 1000 * 100));
            (0, true)
        } else {

            //Copy a block. If the copy returns an error, stop and return the error
            match block_copier.copy_block(reader, writer, buffer.borrow_mut()) {
                Ok(0) => return Ok((0, false)),
                Ok(len) => (len, false),
                Err(e) => return Err(e),
            }
        };

        //Acquire the lock and a reference to the shared data
        {
            let mut lock = self.shared.lock();
            let mut shared = lock.as_mut().unwrap().deref_mut();

            //Check the 'command' field to see if a pause/stop/play has been initiated
            if shared.gui_command.is_some() {
                match shared.gui_command.as_ref().unwrap() {
                    GUICommand::Pause => {
                        self.paused = true;
                    }
                    GUICommand::Resume => {
                        self.paused = false;
                    }
                    GUICommand::Stop => {}
                }

                shared.gui_command = None;
            }

            //Update the total number of bytes copied for all files
            shared.status.overall += len;

            shared.status.paused = self.paused;

            shared.status.progress += len;
        }

        Ok((len, paused))

    }
}
