# IC

This file aims to outline the implementation of a directory copying tool that shows the copy progress, and also allows pausing and resuming of copies.

# Pre copy

Before copying we perform a few checks, making sure the destination is available, and any other checks specified by the options.

Then we create a map from every file in the source directory to a struct containing

 - File size
 - Number of bytes copied
 - Date created
 - Date modified

NOTE: Do we need to separate the 'Number of bytes copied' field from the rest, so that the others can be easily compared with the 'Resume' map (which doesn't contain this field)

We use this map later on to keep track of which files have been copied, and if any files have been modified since the copy was initiated.

# Copying

We copy the data in 8k blocks. After each 8k block, we update the 'Number of bytes copied' field in the file map, and we check to see
if the user has requested a pause.

# Saving

If the program is terminated via a proper exit or sigterm, we save the map (and any other data needed) to the disk.

Store these maps in a single repository in the installation directory, one file for each copy. When the copy is finished, remove the file. Store a shortcut to the file at (not inside) the source directory.

Add an option to continuously save the progress, so if the copy is closed incorrectly (via sigkill or premature power down) the copy can be safely resumed.

# Resume

When a resume is initiated, we load the map for the copy and copy all files that have not yet been copied (copied files will have their
'Number of bytes copied' field equal to the 'File size' field).

If a copy is resumed some time after it was initiated, it is possible that the source files have changed since the copy. We supply an
option that when set, will warn the user if a change has happened. To detect this, we obtain a map (similar to the pre copy stage) and compare the
attributes to the pre copy hash. If tha attributes have changed, warn the user.

To resume copies, start ic with the `-l` argument to list all unfinished copies. Then start ic with the `-r` argument followed by the ID of the copy to resume 

# Threads

The main thread handles the TUI, and a worker thread is spawned to handle the copying. The threads send each other data, the worker thread lets the main thread
know when an 8k block is finished, and the main thread lets the worker thread know when a pause has been initiated.


