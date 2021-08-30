
//Enum encapsulating messages from the main thread to the copier
pub enum Command {
    // Pause the copy
    Pause,

    //Resume or start the copy
    Play,

    //Stop the copying and close the copy thread
    Stop,
}

//Enum encapsulating messages from the copier thread to the main
#[derive(Clone)]
pub struct Status {
    //Number of bytes read from reader and written to writer so far
    pub progress: u64,

    //True if the 'paused' field is set
    pub paused: bool,
}

pub struct Shared {
    pub command: Option<Command>,
    pub status: Status,
}

impl Status {
    pub fn new(progress: u64, paused: bool) -> Self {
        Self {
            progress,
            paused,
        }
    }
}