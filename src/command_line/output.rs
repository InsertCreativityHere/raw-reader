
use crate::disk_info::DiskInfo;

/// TODO
pub fn print_disk_selection_introduction() {
    // Print the opening instructions about selecting a location to read from.
    println!("Select one of the devices below by entering its corresponding number");
    println!("or manually enter the absolute path of the file/device to read from.");
    println!();
}

/// TODO
pub fn print_disk_info(disk_info: &DiskInfo) {
    // Iterate through each column of disk info, and find the length of the longest string in that column.
    let column_widths = disk_info.iter().map(|disk_info_column| {
        disk_info_column.iter().map(|s| s.len()).max().unwrap_or(0)
    }).collect::<Vec<_>>();

    // Print column headers that are spaced to match the column widths.
    println!(
        "        {name:^n$}    {path:^p$}    {space:^s$}    {fs:^f$}    {media:^m$}",
        n = column_widths[0], name  = "NAME",
        p = column_widths[1], path  = " MOUNT POINT",
        s = column_widths[2], space = " USED / TOTAL",
        f = column_widths[3], fs    = "FS",
        m = column_widths[4], media = "MEDIA TYPE",
    );

    // Iterate through each disk and print it's information in nicely formatted columns.
    for i in 0..disk_info[0].len() {
        println!(
            "    [{i}] {name:<n$}    {path:<p$}    {space:<s$}    {fs:<f$}    {media:<m$}",
            n = column_widths[0], name  = disk_info[0][i],
            p = column_widths[1], path  = disk_info[1][i],
            s = column_widths[2], space = disk_info[2][i],
            f = column_widths[3], fs    = disk_info[3][i],
            m = column_widths[4], media = disk_info[4][i],
        )
    }
}

pub fn print_disk_selection_complete() {
    // TODO print something here!
    println!();
}
