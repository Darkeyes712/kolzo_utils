use chrono::prelude::*;
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::time::SystemTime;

pub fn print_metadata_entries(metadata_entries: Vec<fs::Metadata>) {
    for (index, metadata) in metadata_entries.iter().enumerate() {
        println!("Entry at index {}", index);

        let item_name = format!("{}", index);
        println!("1. Name of File: {}", item_name);

        let is_directory = metadata.is_dir();
        println!("2. Is Directory: {}", is_directory);

        let file_size = metadata.len();
        println!("3. File Size: {} bytes", file_size);

        let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let modified_formatted_time = format_system_time(modified_time);
        println!("4. Modification Time: {}", modified_formatted_time);

        let permissions = format_permissions_windows(metadata);
        println!("5. Permissions: {:?}", permissions);

        println!("------------------");
    }
}

/// Formats a given `SystemTime` value as a human-readable string in the format "YYYY-MM-DD HH:MM:SS".
///
/// This function converts the provided `SystemTime` value into a `DateTime<Local>` object
/// and then formats it using the specified format string. The format string includes
/// the year, month, day, hour, minute, and second components of the time.
///
/// # Arguments
///
/// * `time` - A `SystemTime` value representing the time to be formatted.
///
/// # Returns
///
/// A `String` containing the formatted representation of the provided `SystemTime`.
///
/// # Examples
///
/// ```rust
/// use std::time::SystemTime;
///
/// let time = SystemTime::now();
/// let formatted_time = format_system_time(time);
/// println!("Formatted Time: {}", formatted_time);
/// ```
fn format_system_time(time: SystemTime) -> String {
    let dt: DateTime<Local> = time.into();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Formats the Windows-specific file attributes from metadata into a human-readable string.
///
/// This function takes a reference to a `fs::Metadata` object and extracts the Windows-specific
/// file attributes using the `file_attributes()` method. It then checks the attributes and
/// constructs a string indicating whether the file is read-only, hidden, or a system file.
///
/// # Arguments
///
/// * `metadata` - A reference to a `fs::Metadata` object representing the file's metadata.
///
/// # Returns
///
/// A `String` containing a human-readable description of the file attributes.
///
/// # Examples
///
/// ```rust
/// use std::fs;
///
/// let metadata = fs::metadata("path/to/file").expect("Failed to get metadata");
/// let attributes_str = format_permissions_windows(&metadata);
/// println!("File Attributes: {}", attributes_str);
/// ```
fn format_permissions_windows(metadata: &fs::Metadata) -> String {
    let mode = metadata.file_attributes(); // Use Windows-specific method
    let read_only = mode & 0x1 != 0;
    let hidden = mode & 0x2 != 0;
    let system = mode & 0x4 != 0;

    format!(
        "Read-Only: {}, Hidden: {}, System: {}",
        read_only, hidden, system
    )
}

//TODO: Figure out how to make the path logic. We need to be able to select which lath to ls, not only the current path.
//TODO:
//TODO:
