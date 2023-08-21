use chrono::prelude::*;
use std::error::Error;
use std::fs;
use std::time::SystemTime;

pub fn get_all_items_from_directory() -> Result<Vec<fs::Metadata>, Box<dyn Error>> {
    let mut metadata_entries = Vec::new();

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry
                    .metadata()
                    .expect("Failed to acquire the metadata of the entry");
                metadata_entries.push(metadata);
            }
        }
        Ok(metadata_entries)
    } else {
        eprintln!("Error reading the directory");
        Err("Error reading directory".into())
    }
}

pub fn print_metadata_entries() {
    match get_all_items_from_directory() {
        Ok(metadata_entries) => {
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

                let permissions = metadata.permissions();
                println!("5. Permissions: {:?}", permissions);

                println!("------------------");
            }
        }
        Err(err) => eprintln!("Error: {}", err),
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

//TODO: Decide on how to format the data, whether it should be in 1 function or seperate for each entry.
//TODO: Move the ls relevant code to ls.rs and keep here only the calling of acquisition of the metadata.
//TODO: Figure out how to make the path logic. We need to be able to select which lath to ls, not only the current path.

// pub fn test_metadata(metadata: &Vec<fs::Metadata>) {
//     println!("{:?}", metadata);

//     let first_index = metadata.get(0).unwrap();
//     println!("{:?}", first_index.is_dir());
//     println!("{:?}", metadata.get(1));
// }
