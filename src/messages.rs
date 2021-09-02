
//Enum encapsulating messages from the GUI to the copier
pub enum GUICommand {
    // Pause the copy
    Pause,

    //Resume or start the copy
    Resume,

    //Stop the copying and close the copy thread
    Stop,
}

//Enum encapsulating messages from the GUI (such as file change)
pub enum ManagerCommand {
    //Inform the GUI that the manager has started to copy a new file
    StartFile{file_size: u64, file_path: String},
}

//Enum encapsulating messages from the copier thread to the main
#[derive(Clone)]
pub struct Status {
    //Number of bytes copied for the current file
    pub progress: u64,

    //True if the 'paused' field is set
    pub paused: bool,

    //True if a stop command has been received
    pub stopped: bool,

    //Overall progress of all the copies
    pub overall: u64,
}

pub struct Shared {
    pub gui_command: Option<GUICommand>,
    pub status: Status,
    pub manager_command: Option<ManagerCommand>,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            progress: 0,
            paused: false,
            overall: 0,
            stopped: false,
        }
    }
}