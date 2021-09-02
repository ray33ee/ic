use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{Metadata, OpenOptions};
use std::time::SystemTime;

use serde::{Serialize, Deserialize};
use std::io::{Write, Read};

//The metadata for a single sub file
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct FileAttributes {
    size: u64,
    created: SystemTime,
    modified: SystemTime,
}

impl From<&Metadata> for FileAttributes {
    fn from(metadata: &Metadata) -> Self {
        Self {
            size: metadata.len(),
            created: metadata.created().unwrap(),
            modified: metadata.modified().unwrap(),
        }
    }
}

//Struct containing the metadata of all sub files, along with a timestamp
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderSignature {
    pub base_path: PathBuf,

    pub time: SystemTime,

    pub total_size: u64,

    pub state_map: HashMap<PathBuf, FileAttributes>,

    // The number of bytes already copied for each file and the total size of the file
    pub copied_map: HashMap<PathBuf, (u64, u64)>,
}

impl FolderSignature {

    //Scan the folder at 'path' to create a new 'SubFilesState' object
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let time = SystemTime::now();

        let mut state_map = HashMap::new();
        let mut copied_map = HashMap::new();

        let mut total_size = 0;

        let base_path = PathBuf::from(path.as_ref());

        for entry in walkdir::WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {

            let relative_path = entry.path().strip_prefix(&path).unwrap();

            let metadata = entry.metadata().unwrap();

            total_size += metadata.len();

            copied_map.insert(PathBuf::from(relative_path), (0, metadata.len()));

            state_map.insert(PathBuf::from(relative_path),  FileAttributes::from(&metadata));
        }

        Self {
            base_path,
            time,
            state_map,
            copied_map,
            total_size,
        }
    }

    pub fn overall_size(&self) -> u64 {
        self.total_size
    }

    //Load an existing 'FolderSignature' object from a file
    pub fn load_ron<P: AsRef<Path>>(path: P) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path).unwrap();

        let mut ron_string = String::new();

        file.read_to_string(& mut ron_string).unwrap();

        ron::from_str(& ron_string).unwrap()
    }

    //Save the 'FolderSignature' to a file
    pub fn save_ron<P: AsRef<Path>>(&self, path: P) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path).unwrap();

        file.write_all(ron::to_string(&self).unwrap().as_bytes()).unwrap();
    }

}

//Two states are deemed the same if their state maps are equal. This means they have the same file structure, and
//those files have the same a) size, b) created time and c) modified time
impl PartialEq for FolderSignature {
    fn eq(&self, other: &Self) -> bool {
        self.state_map == other.state_map
    }
}