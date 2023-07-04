pub fn run_p4_info() -> Result<String, String> {
    use std::str;
    use std::process::Command;

    let output = Command::new("p4")
        .args(&["info",])
        .output()
        .map_err(|e| format!("Failed to run p4 command: {}", e))?;

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout)
            .map_err(|_| "Failed to convert output to UTF-8")?;
        Ok(output_str.to_owned())
    } else {
        let error_str = str::from_utf8(&output.stderr)
            .map_err(|_| "Failed to convert error output to UTF-8")?;
        Err(format!("p4 command failed: {}", error_str))
    }
}
