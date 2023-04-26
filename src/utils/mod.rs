use std::fs;
use std::path::Path;
use std::error::Error;

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
