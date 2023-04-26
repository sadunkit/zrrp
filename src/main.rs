mod unreal;
mod utils;

use clap::Command;
use unreal::*;

fn cli() -> Command {
    Command::new("zrrp")
        .about("A developers best friend")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clean").about("Cleans up unreal projects under the current directory"),
        )
        .subcommand(
            Command::new("clean-ddc").about("Cleans up the DerivedDataCache folder inside unreal engine"),
        )
}

fn run_p4_info() -> Result<String, String> {
    use std::str;
    use std::process::Command;

    let output = Command::new("p4")
        .args(&[
            "info",
            ])
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

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("clean", _)) => {
            println!("Cleaning up");
            clean();
        }
        Some(("clean-ddc", _)) => {
            println!("Cleaning DDC");
            clean_ddc();
        }
        Some(("p4-info", _)) => {
            match run_p4_info() {
                Ok(output) => println!("Output:\n{}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        _ => unreachable!()
    }
}
