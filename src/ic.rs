use crate::foldersignature::FolderSignature;
use std::path::{Path, PathBuf};
use crate::options::Options;
use std::sync::{Arc, Mutex};
use crate::messages::{Shared, ManagerCommand};
use std::fs::OpenOptions;
use crate::copier::Copier;
use std::io::{Seek, SeekFrom};
use crate::blockcopy::{BufferBlockCopy, BlockCopy};
use std::io::Result;
use std::ops::DerefMut;

//Take a 'FolderSignature' instance and start/resume the copy. Handle the:
// - Command line arguments
// - Keep track of the number of bytes copied
pub struct IC {
    destination_base_path: PathBuf,
    signature: FolderSignature,
    options: Options,
    shared: Arc<Mutex<Shared>>,
}

impl IC {

    pub fn new<P: AsRef<Path>>(
        destination_base_path: P,
        signature: FolderSignature,
        options: Options,
        shared: Arc<Mutex<Shared>>) -> Self
        where PathBuf: From<P> {

        Self {
            destination_base_path: PathBuf::from(destination_base_path),
            signature,
            options,
            shared,
        }
    }

    pub fn resume(& mut self) -> Result<()> {
        //Make sure the directory structure of the destination path is correct

        let mut copier = Copier::new(Arc::clone(&self.shared));

        let mut buffer = vec![0u8; BufferBlockCopy.bytes_per_block() as usize];

        for (path, (bytes_copied, file_size)) in self.signature.copied_map.iter_mut() {

            let destination_file_path = self.destination_base_path.clone().join(path);
            let source_file_path = self.signature.base_path.clone().join(path);

            //Send the StartFile command
            {
                let mut lock = self.shared.lock();
                let mut shared = lock.as_mut().unwrap().deref_mut();

                shared.manager_command = Some(ManagerCommand::StartFile { file_size: *file_size , file_path: String::from(path.to_str().unwrap()) });

                shared.status.progress = 0;
            }

            let mut destination_fh = OpenOptions::new()
                .create(true)
                .append(true)
                .open(destination_file_path)?;

            let mut source_fh = OpenOptions::new()
                .read(true)
                .open(source_file_path)?;

            source_fh.seek(SeekFrom::Start(*bytes_copied))?;

            loop {
                let (len, paused) = copier.copy(& mut source_fh, & mut destination_fh, &BufferBlockCopy, & mut buffer)?;

                if len == 0 && !paused {
                    break;
                }

                *bytes_copied += len;
            }



        }

        Ok(())

    }
    
}