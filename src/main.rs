mod blockcopy;
mod messages;
mod nullstream;
mod foldersignature;
mod ic;
mod options;
mod gui;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use ic::IC;

use gui::GUI;
use crate::foldersignature::FolderSignature;
use crate::options::Options;

fn main() {

    let (g_send, g_recv) = channel();
    let (c_send, c_recv) = channel();


    //let fs = FolderSignature::new("E:\\Software Projects\\IntelliJ\\ic\\source");
    let fs = FolderSignature::load_ron("E:\\Software Projects\\IntelliJ\\ic\\saved.ron");

    let overall_size = fs.overall_size();

    let mut gui = GUI::new(g_send, c_recv, fs.overall_size());

    let mut ic = IC::new(
        "E:\\Software Projects\\IntelliJ\\ic\\destination",
        fs,
    Options,
    g_recv,
            c_send);


    thread::spawn(move || {
        ic.resume().unwrap();


        //ic.signature().save_ron("E:\\Software Projects\\IntelliJ\\ic\\saved.ron");
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
