//Handles the indicatif progress bars, processing the key stroked
//Must display the progress of the current file copy, and the overall progress of all files to copy
//Must also detect and pass on key events
//Each progress bar must display time elapsed, eta, bytes copied over total bytes, and transfer rate

use std::sync::mpsc::{Sender, Receiver};
use crate::messages::{CopierMessage, GUIMessage};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::time::Duration;
use crossterm::event::{read, poll, Event, KeyCode};

pub struct GUI {
    //shared: Arc<Mutex<Shared>>,
    current_file: ProgressBar,
    overall: ProgressBar,
    //multi: MultiProgress,
    stopped: bool,

    sender: Sender<GUIMessage>,
    receiver: Receiver<CopierMessage>,
}

impl GUI {

    // pass in the shared memory and create the progress bars
    pub fn new(sender: Sender<GUIMessage>, receiver: Receiver<CopierMessage>, overall_size: u64) -> Self {
        let multi = MultiProgress::new();

        let overall = multi.add(ProgressBar::new(overall_size));
        let current_file = multi.add(ProgressBar::new(1));

        current_file.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}]  {msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
            .progress_chars("#>-"));

        overall.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Overall Progress: [{wide_bar:.cyan/blue}] {msg} {bytes}/{total_bytes} {binary_bytes_per_sec} ({eta})")
            .with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
            .progress_chars("#>-"));

        overall.set_message("Copying...");

        current_file.tick();
        overall.tick();


        Self {
            current_file,
            overall,
            //multi,
            stopped: false,
            sender,
            receiver,
        }
    }

    //Look at the shared memory and update the progress bar
    //Look at the keyboard events and update the shared memory
    pub fn update(& mut self) -> u64 {

        //Send a message to the copier. The copier should respond with a CopierMessage::Progress object
        self.sender.send(GUIMessage::Request).unwrap();

        //Check to see if a keyboard button has been pressed
        if let Ok(true) = poll(Duration::from_secs(0)) {

            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('p') => {
                            self.overall.set_message("Paused.");
                            self.sender.send(GUIMessage::Pause).unwrap();
                        },
                        KeyCode::Char('s') => {
                            self.overall.set_message("Copying...");
                            self.current_file.reset_eta();
                            self.overall.reset_eta();
                            self.sender.send(GUIMessage::Resume).unwrap();
                        },
                        KeyCode::Char('q') => {
                            self.overall.set_message("Stopped.");
                            self.stopped = true;
                            self.stop();
                            self.sender.send(GUIMessage::Stop).unwrap();
                        },

                        _ => {}
                    }
                }
                _ => {}
            }
        }

        //Keep waiting for a CopierMessage::Progress message
        loop {
            match self.receiver.recv() {
                Ok(copier_message) => {
                    match copier_message {
                        CopierMessage::Progress(file, overall) => {
                            self.current_file.set_position(file);
                            self.overall.set_position(overall);
                            break;
                        }
                        CopierMessage::StartFile { file_size, file_path } => {
                            self.current_file.set_length(file_size);
                            self.current_file.reset();
                            self.current_file.set_message(file_path.clone());
                        }
                        CopierMessage::Finished => {
                            self.current_file.set_position(self.current_file.length());
                            self.overall.set_position(self.overall.length());
                            self.finish();
                        }
                    }
                }
                Err(_) => {}
            }
        }
        0

    }

    //Call this function when all the copies have finished
    pub fn finish(&self) {

        self.current_file.finish();
        self.overall.finish_with_message("done");

    }

    //Call this function when the copies have been forcibly stopped
    pub fn stop(&self) {

        self.current_file.finish();
        self.overall.finish_with_message("STOPPED");
    }

}

