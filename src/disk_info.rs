
use sysinfo::{System, SystemExt, DiskExt};

// TODO Check the comments/logic here to see if "disk" really means volume or device on windows!

/// A list of metric unit suffixes to describe quantities of bytes.
/// Each unit in the vector, is 1024 times larger than the unit before it.
pub const UNIT_SUFFIXES: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

/// Represents a table of strings that describe the available disks.
/// The table has 1 row per disk, and 5 columns that store the following information:
/// - name: The human readable name of the disk.
/// - mount point: The path where the root of the disk is mounted to.
/// - space summary: Describes the total and used space in the following format: "<used> / <total>".
/// - file system: For common filesystems, this stores the human readable name of it: "NTFS".
///                For unknown filesystems, we stringify the raw bytes of the TODO
/// - media type: What kind of hardware the disk is using: SSD vs HDD, and whether it's internal or external.
pub type DiskInfo = [Vec<String>; 5];

/// Returns a table of strings describing all the disks that are currently available for scanning.
pub fn get_disk_info() -> DiskInfo {
    // Load any storage devices that are currently connected to the system.
    let mut system_info = System::new();
    system_info.refresh_disks_list();
    let disks = system_info.disks();

    // Allocate an array for storing the disk information in,
    // and iterate through the discovered disks to populate it.
    let mut disk_info = DiskInfo::default();
    for disk in disks {
        // Get the name and mount point of the disk as strings.
        let disk_name = disk.name().to_string_lossy();
        let disk_path = disk.mount_point().to_string_lossy();

        // Determine what units to measure the space of the disk with by finding the largest
        // power of 1024 that divides the total space. If an error occurs, we default to `0`.
        // We also ensure that we don't exceed the number of unit suffixes hardcoded in this program.
        let unit_order = std::cmp::min(
            UNIT_SUFFIXES.len() as u32,
            disk.total_space().checked_ilog(1024u64).unwrap_or(0),
        );
        let unit_factor = 1024u64.pow(unit_order);
        let unit_suffix = UNIT_SUFFIXES[unit_order as usize];
        // Compute the used and total space on the disk in the selected units.
        let used_space = ceil_divide!(disk.total_space() - disk.available_space(), unit_factor);
        let total_space = ceil_divide!(disk.total_space(), unit_factor);
        // Create a string that summarizes the disk space.
        let disk_space = format!("{used_space} {unit_suffix} / {total_space} {unit_suffix}");

        // Get the file system that the disk is formatted with (if any). If the description
        // bytes are valid utf8, format them as a string, otherwise display the raw bytes.
        let file_system = match std::str::from_utf8(disk.file_system()) {
            Ok(s) => s.to_owned(),
            Err(_) => format!("{:?}", disk.file_system()),
        };

        // Summarize the type of media and whether it's removable or not.
        let media_type = format!(
            "{:?} ({})",
            disk.type_(),
            if disk.is_removable() { "external" } else { "internal" },
        );

        // Store the strings in the `disk_info` array for further formatting.
        disk_info[0].push(disk_name.into_owned());
        disk_info[1].push(disk_path.into_owned());
        disk_info[2].push(disk_space);
        disk_info[3].push(file_system);
        disk_info[4].push(media_type);
    }
    disk_info
}
