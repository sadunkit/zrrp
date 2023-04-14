use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn clean() {
    let current_dir = env::current_dir().expect("Failed to get the current directory");

    if let Err(e) = remove_unwanted_directories(&current_dir) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn remove_unwanted_directories<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let unwanted_dirs = ["Saved", "Intermediate", "Binaries"];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let dir_name = path.file_name().and_then(|n| n.to_str());

        if entry.file_type()?.is_dir() {
            if dir_name.map(|n| unwanted_dirs.contains(&n)).unwrap_or(false) {
                println!("Removing {}", path.display());
                fs::remove_dir_all(&path)?;
            } else {
                remove_unwanted_directories(&path)?;
            }
        }
    }

    Ok(())
}