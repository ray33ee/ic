mod blockcopy;
mod copier;
mod messages;
mod nullstream;

use nullstream::NullStream;
use messages::{Shared, Status};
use std::sync::{Mutex, Arc};
use std::thread;
use copier::Copier;
use blockcopy::BufferBlockCopy;
use std::time::Duration;
use std::ops::DerefMut;
use crate::messages::Command;
use crossterm::event::{read, poll, Event, KeyCode};

fn main() {

    //Shared data between the copier and the main thread
    let shared = Shared {
        command: None,
        status: Status::new(0, false),
    };

    let main_shared = Arc::new(Mutex::new(shared));

    let mut copier = copier::Copier::new(Arc::clone(&main_shared));

    thread::spawn(move || copier.copy(& mut NullStream, & mut NullStream, &BufferBlockCopy));

    loop {

        let command = if let Ok(true) = poll(Duration::from_secs(0)) {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('p') => {Some(Command::Pause)},
                        KeyCode::Char('s') => {Some(Command::Play)},
                        KeyCode::Char('q') => {Some(Command::Stop)},

                        _ => {None}
                    }
                }
                _ => {None}
            }
        } else {
            None
        };

        let status = {
            let mut lock = main_shared.lock();
            let mut shared = lock.as_mut().unwrap().deref_mut();

            shared.command = command;

            shared.status.clone()

        };

        println!("progress: {} {} {}", status.progress, status.paused, Arc::strong_count(&main_shared));

        thread::sleep(Duration::from_millis(1000));
    }

}
