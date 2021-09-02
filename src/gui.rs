//Handles the indicatif progress bars, processing the key stroked
//Must display the progress of the current file copy, and the overall progress of all files to copy
//Must also detect and pass on key events
//Each progress bar must display time elapsed, eta, bytes copied over total bytes, and transfer rate

use std::sync::{Arc, Mutex};
use crate::messages::{Shared, GUICommand, ManagerCommand};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::time::Duration;
use crossterm::event::{read, poll, Event, KeyCode};
use std::ops::DerefMut;

pub struct GUI {
    shared: Arc<Mutex<Shared>>,
    current_file: ProgressBar,
    overall: ProgressBar,
    //multi: MultiProgress,
}

impl GUI {

    // pass in the shared memory and create the progress bars
    pub fn new(shared: Arc<Mutex<Shared>>, overall_size: u64) -> Self {
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
            shared,
            current_file,
            overall,
            //multi,
        }
    }

    //Look at the shared memory and update the progress bar
    //Look at the keyboard events and update the shared memory
    pub fn update(&self) -> u64 {

        let gui_command = if let Ok(true) = poll(Duration::from_secs(0)) {

            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('p') => {
                            self.overall.set_message("Paused.");
                            Some(GUICommand::Pause)
                        },
                        KeyCode::Char('s') => {
                            self.overall.set_message("Copying...");
                            self.current_file.reset_eta();
                            self.overall.reset_eta();
                            Some(GUICommand::Resume)
                        },
                        KeyCode::Char('q') => {
                            self.overall.set_message("Stopped.");
                            Some(GUICommand::Stop)
                        },

                        _ => {None}
                    }
                }
                _ => {None}
            }
        } else {
            None
        };


        let mut lock = self.shared.lock();
        let mut shared = lock.as_mut().unwrap().deref_mut();

        //If a stop command has been sent, set the stop flag
        if let Some(ref c) = gui_command {
            if let GUICommand::Stop = c {
                shared.status.stopped = true;
            }
        }

        // Send the gui command off to the copier thread
        shared.gui_command = gui_command;

        // Process the manager command
        if let Some(ref m_command) = shared.manager_command {
            match m_command {
                ManagerCommand::StartFile { file_size, file_path } => {
                    self.current_file.set_length(*file_size);
                    self.current_file.reset();
                    self.current_file.set_message(file_path.clone());
                }
            }

            shared.manager_command = None;
        }



        self.current_file.set_position(shared.status.progress);
        self.overall.set_position(shared.status.overall);


        shared.status.overall

    }

    //Call this function when all the copies have finished
    pub fn finish(&self) {

        self.current_file.finish();
        self.overall.finish_with_message("done");

    }

}

