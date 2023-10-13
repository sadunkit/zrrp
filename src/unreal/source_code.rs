use std::fs;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::path::Path;
use log::debug;

// Define the new data type.
pub struct FileInfo {
    pub count_config_vars: i32,
    pub line_numbers: Vec<usize>,
}

pub type FileStats = HashMap<String, FileInfo>;

pub fn count_uproperty_config(path: &str) -> FileStats {
    let mut counts: FileStats = HashMap::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension() == Some(Path::new("h").as_ref()) {
            if let Ok(contents) = fs::read_to_string(path) {
                let mut line_numbers = Vec::new();
                let lines: Vec<&str> = contents.split('\n').collect();
                for (i, line) in lines.iter().enumerate() {
                    let lower = line.to_lowercase();
                    if lower.contains("uproperty") && lower.contains("config") {
                        line_numbers.push(i + 1);
                        debug!("Found UPROPERTY(Config) at line {}", i + 1);
                    }
                }
                let matches = line_numbers.len() as i32;
                if matches > 0 {
                    let filename = entry.file_name().to_string_lossy().into_owned();
                    debug!("Added file {} with {} UPROPERTY(Config) instances", filename, matches);
                    counts.insert(filename, FileInfo { count_config_vars: matches, line_numbers });
                }
            }
        }
    }

    debug!("Finished counting UPROPERTY(Config) instances");

    counts
}