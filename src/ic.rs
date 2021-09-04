use crate::foldersignature::FolderSignature;
use std::path::{Path, PathBuf};
use crate::options::Options;
use std::sync::mpsc::{Receiver, Sender};
use crate::messages::{GUIMessage, CopierMessage};
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use crate::blockcopy::{BufferBlockCopy, BlockCopy};
use std::io::Result;
use std::thread::sleep;
use std::time::{Duration};
use std::borrow::BorrowMut;

//Take a 'FolderSignature' instance and start/resume the copy. Handle the:
// - Command line arguments
// - Keep track of the number of bytes copied
pub struct IC {
    destination_base_path: PathBuf,
    signature: FolderSignature,
    options: Options,

    receiver: Receiver<GUIMessage>,
    sender: Sender<CopierMessage>,

    file_copied: u64,
    overall_copied: u64,


    paused: bool,
}

impl IC {

    pub fn new<P: AsRef<Path>>(
        destination_base_path: P,
        signature: FolderSignature,
        options: Options,
        receiver: Receiver<GUIMessage>,
        sender: Sender<CopierMessage>,) -> Self
        where PathBuf: From<P> {

        Self {
            destination_base_path: PathBuf::from(destination_base_path),
            signature,
            options,
            paused: false,
            receiver,
            sender,

            file_copied: 0,
            overall_copied: 0
        }
    }

    pub fn resume(& mut self) -> Result<()> {
        //Make sure the directory structure of the destination path is correct


        let mut buffer = vec![0u8; BufferBlockCopy.bytes_per_block() as usize];

        let overall_offset = self.signature.overall_offset();

        for (path, (bytes_copied, file_size)) in self.signature.copied_map.iter_mut() {

            let destination_file_path = self.destination_base_path.clone().join(path);
            let source_file_path = self.signature.base_path.clone().join(path);

            let file_offset = *bytes_copied;

            self.sender.send(CopierMessage::StartFile { file_size: *file_size , file_path: String::from(path.to_str().unwrap()) }).unwrap();

            let mut destination_fh = OpenOptions::new()
                .create(true)
                .write(true)
                .open(destination_file_path)?;

            let mut source_fh = OpenOptions::new()
                .read(true)
                .open(source_file_path)?;

            source_fh.seek(SeekFrom::Start(*bytes_copied))?;
            destination_fh.seek(SeekFrom::Start(*bytes_copied))?;

            loop {

                //If we are paused, sleep for a bit
                let len = if self.paused {
                    //Put the thread to sleep for 100ms
                    sleep(Duration::from_millis(100));
                    0
                } else {

                    //Copy a block. If the copy returns an error, stop and return the error
                    match BufferBlockCopy.copy_block(& mut source_fh, & mut destination_fh, buffer.borrow_mut()) {
                        Ok(0) => {
                            if !self.paused {
                                break;
                            }
                            0
                        },
                        Ok(len) => len,
                        Err(e) => return Err(e),
                    }
                };

                self.overall_copied += len;

                self.file_copied += len;

                *bytes_copied += len;

                match self.receiver.try_recv() {
                    Ok(gui_message) => {
                        match gui_message {
                            GUIMessage::Pause => {self.paused = true;}
                            GUIMessage::Resume => {self.paused = false;}
                            GUIMessage::Stop => {return Ok(());}
                            GUIMessage::Request => {
                                self.sender.send(CopierMessage::Progress(self.file_copied + file_offset, self.overall_copied + overall_offset)).unwrap();
                            }
                        }
                    }
                    Err(_) => {}
                }


            }

        }

        self.sender.send(CopierMessage::Finished).unwrap();

        Ok(())

    }
    
}