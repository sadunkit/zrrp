use std::env;
use std::process;
use log::error;

use crate::utils::*;

fn clean_internal(unwanted_dirs: &[&str]) {
    let current_dir = env::current_dir().expect("Failed to get the current directory");

    if !directory_has_file(&current_dir, "uproject") {
        if !ask_yes_no_question("This folder doesn't have .uproject file Continue? ", 4) {
            println!("Exiting...");
            process::exit(0);
        }
    }

    match remove_unwanted_directories(&current_dir, &unwanted_dirs) {
        Ok(_) => println!("Successfully removed directories"),
        Err(e) => {
            error!("Error: {}", e);
            process::exit(1);
        }
    }
}

pub fn clean() {
    let unwanted_dirs = ["Saved", "Intermediate", "Binaries", "DerivedDataCache"];

    clean_internal( &unwanted_dirs)
}

pub fn clean_ddc() {
    let unwanted_dirs = ["DerivedDataCache"];

    clean_internal(&unwanted_dirs)
}
