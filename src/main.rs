
#[macro_use]
mod math_util;

mod command;
mod command_line;
mod data;
mod disk_info;
mod pattern;

fn main() {
    command_line::output::print_disk_selection_introduction();
    let disk_info = disk_info::get_disk_info();
    command_line::output::print_disk_info(&disk_info);
    let file = command_line::input::get_user_disk_selection(&disk_info[1]);
    command_line::output::print_disk_selection_complete();

    let mut input_handler = command_line::handle::CommandInputHandler::new();
    loop {
        match input_handler.prompt("\n> ").parse::<command::Command>() {
            Ok(command) => process_command(command),
            Err(err) => eprintln!("error: {err}"),
        }
    }
}

fn process_command(command: command::Command) {
    match command {
        
    }
}

struct Test<'a, const N: usize> {
    staging_buffer: data::aligned_buffer::AlignedBuffer<N>,
    worker_buffer: data::aligned_buffer::AlignedBuffer<N>,

    worker_channels: std::sync::mpsc::Sender<&'a data::aligned_buffer::AlignedBuffer<N>>,
    completed_workers: std::sync::atomic::AtomicUsize,
}

impl<'a, const N: usize> Test<'a, N> {

}





// so it has 2 buffers, it reads into one, while workers access the other buffer.
// Before it can do a buffer swap it needs to make sure that everyone is done with everything.

/*
buffer1 = empty
buffer2 = empty

read-data-into-buffer1-during-initialization
buffer1 = data0
buffer2 = empty

swap-buffers-during-initiliazation
buffer1 = empty
buffer2 = data0

signal-to-workers-that-data-is-ready
buffer1 = empty
buffer2 = data0

read-data-into-buffer1
buffer1 = data1
buffer2 = data0

wait-for-workers-to-check-in
buffer1 = data1
buffer2 = data0

swap-the-buffers
buffer1 = OLD
buffer2 = data1


staging_buffer: AlignedBuffer<N>
worker_buffer: AlignedBuffer<N>

worker_channels: 


 */


// WORK ON COMMAND AND PATTERN!


// TODO the command line gives us escaped strings but we don't use escapes!!


// Open a device
// Seek in the device
// Print data from the device
// Search for data on the device





// Select from one of the devices below or manually enter the path of the file/device to read from.



// print X
// find nonzero
// find bytes [0, 5, 3, ...]      (find these bytes)
// find string "regex" (find a match to this regex)
// exit
// help

// config 





// If a sector has data, and a section isn't open:
//     write the address of the sector in 8 bytes




// sector map
// 8 sectors can be stored in a single byte, the bits of that byte correspond to the sector.
//
// 4 bytes can store 32 sectors
// 5 bytes can store 40 sectors
// 6 bytes can store 48 sectors
// 8 bytes can store 64 sectors





// address compression
// Addresses are stored as sector addresses, so the exact byte location they point to depends on
// the configured sector size.
// These addresses use a compressed format, with the first 2 bits specifying the number of bytes
// the address is encoded on (including those 2 bits). These are described below:


// FOR A SECTOR SIZE OF 1
// 2 bit header & 4 bytes       lets us store addresses up to (1 GB)
// 2 bit header & 5 bytes       lets us store addresses up to (256 GB)
// 2 bit header & 6 bytes       lets us store addresses up to (64 TB)
// 2 bit header & 8 bytes       lets us store addresses up to (4096 PB)


// FOR A SECTOR SIZE OF 1K
// 2 bit header & 4 bytes       lets us store addresses up to (1 TB)
// 2 bit header & 5 bytes       lets us store addresses up to (256 TB)
// 2 bit header & 6 bytes       lets us store addresses up to (64 PB)
// 2 bit header & 8 bytes       lets us store addresses up to (4096 EB)


// FOR A SECTOR SIZE OF 4K
// 2 bit header & 4 bytes       lets us store addresses up to (4 TB)
// 2 bit header & 5 bytes       lets us store addresses up to (1 PB)
// 2 bit header & 6 bytes       lets us store addresses up to (256 PB)
// 2 bit header & 8 bytes       lets us store addresses up to (16384 EB)


// FOR A SECTOR SIZE OF 16K
// 2 bit header & 4 bytes       lets us store addresses up to (16 TB)
// 2 bit header & 5 bytes       lets us store addresses up to (4 PB)
// 2 bit header & 6 bytes       lets us store addresses up to (1 EB)
// 2 bit header & 8 bytes       lets us store addresses up to (65536 EB)


//the largest address we can store for X bytes with a 2bit header (2^((8*X)-2))





// Segment lengths are always stored on 3 bytes.
// If we hit the max value for this, we have to open a new segment, even if we theoretically didn't need to.
// 3 bytes lets us store (2^24)*(8) =    134,217,728 sectors per segment        512GB for 4k sectors





// sector maps are always of the form:
// [address][segment_length][segment_map]
// Where `address` is the compressed sector address that this segment starts at.
// `segment_length` is the number of bytes stored in this segment (or -1 if this segment goes to EOF)
// `segment_map` is a massive bit vector storing whether the sectors were empty or not.





// Length of 


// Things we want to accomplish:
// - keep the number of addresses small (this makes it easier to search through)
// - keep the file itself small too (makes it easier to store)