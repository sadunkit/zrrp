use std::env;
use std::process;

use crate::utils::remove_unwanted_directories;

pub fn clean() {
    let current_dir = env::current_dir().expect("Failed to get the current directory");
    let unwanted_dirs = ["Saved", "Intermediate", "Binaries", "DerivedDataCache"];

    if let Err(e) = remove_unwanted_directories(&current_dir, &unwanted_dirs) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

pub fn clean_ddc() {
    let current_dir = env::current_dir().expect("Failed to get the current directory");
    let unwanted_dirs = ["DerivedDataCache"];

    if let Err(e) = remove_unwanted_directories(&current_dir, &unwanted_dirs) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
