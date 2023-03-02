
use super::handle::DiskSelectionInputHandler;
use std::fs::File;
use std::io::{self, Error, ErrorKind};

/// TODO
pub fn get_user_disk_selection(disk_paths: &[String]) -> File {
    let mut input_handler = DiskSelectionInputHandler::new();
    loop {
        // If the user's selection was valid, return it, otherwise print why it was invalid.
        match get_user_disk_selection_impl(&mut input_handler, disk_paths) {
            Ok(file) => return file,
            Err(err) => eprintln!("error: {err}"),
        }
    }
}

/// TODO
fn get_user_disk_selection_impl(input_handler: &mut DiskSelectionInputHandler, disk_paths: &[String]) -> io::Result<File> {
    // Attempt to read a line from `stdin` into the provided string buffer.
    let mut selection = input_handler.prompt("\n> ");

    // If the user typed an integer, replace it with the corresponding disk's path.
    if let Ok(index) = selection.trim().parse::<usize>() {
        // Ensure the provided integer corresponds to a disk, otherwise return an error.
        let disk_path = disk_paths.get(index).ok_or_else(|| {
            let message = format!(
                "'{index}' does not correspond to a disk. Enter a number between 0 and {} (inclusive).",
                disk_paths.len() - 1,
            );
            Error::new(ErrorKind::NotFound, message)
        })?;

        // Replace the integer they typed with that disk's path.
        selection = disk_path.clone();
    }

    // Obtain a handle to the file/device at the specified path.
    // Returns an error if no file/device exists at that path or if it's unreadable.
    File::open(selection.trim())
}
