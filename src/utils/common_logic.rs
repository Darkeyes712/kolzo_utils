use std::error::Error;
use std::fs;

pub fn get_all_items_from_directory(path: &str) -> Result<Vec<fs::Metadata>, Box<dyn Error>> {
    let mut metadata_entries = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
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
