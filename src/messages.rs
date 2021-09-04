
//Enum encapsulating messages from the GUI to the copier
pub enum GUIMessage {
    // Pause the copy
    Pause,

    //Resume or start the copy
    Resume,

    //Stop the copying and close the copy thread
    Stop,

    //Request the progress of the copy
    Request,
}

//Enum encapsulating messages from the copier thread to the main
#[derive(Clone)]
pub enum CopierMessage {
    //Number of bytes copied for the current file and for all files combined
    Progress(u64, u64),

    //Inform the GUI that the copy has finished
    Finished,

    //Inform the GUI that the manager has started to copy a new file
    StartFile{file_size: u64, file_path: String},
}