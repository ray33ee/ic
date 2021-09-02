mod blockcopy;
mod copier;
mod messages;
mod nullstream;
mod foldersignature;
mod ic;
mod options;
mod gui;

use messages::{Shared};
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use ic::IC;

use gui::GUI;
use crate::foldersignature::FolderSignature;
use crate::options::Options;

fn main() {

    let shared = Shared {
        gui_command: None,
        status: Default::default(),
        manager_command: None
    };

    let shared = Arc::from(Mutex::from(shared));

    let fs = FolderSignature::new("E:\\Software Projects\\IntelliJ\\ic\\source");

    let overall_size = fs.overall_size();

    let gui = GUI::new(Arc::clone(&shared), overall_size);

    let mut ic = IC::new(
        "E:\\Software Projects\\IntelliJ\\ic\\destination",
        fs,
    Options,
    Arc::clone(&shared));


    thread::spawn(move || {
        ic.resume().unwrap();
    });

    loop {

        let overall_progress = gui.update();

        if overall_progress == overall_size {
            gui.finish();
            break;
        }

        thread::sleep(Duration::from_millis(100));

    }


}
