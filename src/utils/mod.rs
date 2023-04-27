use std::io::Write;
use std::{fs, io};
use std::path::Path;
use std::error::Error;

pub fn has_uproject_file<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("uproject") {
                    return true;
                }
            }
        }
    }
    false
}

pub fn remove_unwanted_directories(path: &Path, unwanted_dirs: &[&str]) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let dir_name = path.file_name().and_then(|n| n.to_str());

        if entry.file_type()?.is_dir() {
            if dir_name.map(|n| unwanted_dirs.contains(&n)).unwrap_or(false) {
                println!("Removing {}", path.display());
                fs::remove_dir_all(&path)?;
            } else {
                remove_unwanted_directories(&path, unwanted_dirs)?;
            }
        }
    }

    Ok(())
}

pub fn ask_yes_no_question(prompt: &str, max_retries: usize) -> bool {
    println!("{} (y/n)", prompt);
    io::stdout().flush().unwrap();

    for _ in 0..max_retries {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                print!("Invalid input. Please enter 'y' or 'n': ");
                io::stdout().flush().unwrap();
            }
        }
    }

    return false;
}