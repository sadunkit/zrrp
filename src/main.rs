mod unreal;
mod utils;

use clap::{Command, ValueEnum};
use unreal::*;
use log::{self, Metadata, Level, Record};

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
        .subcommand(
            Command::new("pua").about("Counts UPROPERTY(Config) instances in the given path"),
        )
}

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run swiftly
    Unreal,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    Zrrp,
}

fn run_p4_info() -> Result<String, String> {
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

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

fn main() {
    let matches = cli().get_matches();

    log::set_logger(&MyLogger).unwrap();

    log::set_max_level(log::LevelFilter::Debug);

    match matches.subcommand() {
        Some(("clean", _)) => {
            println!("Cleaning up");
            clean();
        }
        Some(("clean-ddc", _)) => {
            println!("Cleaning DDC");
            clean_ddc();
        }
        Some(("pua", _)) => {
            count_uproperty_config("TODO");
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
