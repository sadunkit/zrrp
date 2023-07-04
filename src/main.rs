mod unreal;
mod utils;
mod perforce;
mod app_logger;

use clap::{Command, ValueEnum};

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

fn main() {
    let matches = cli().get_matches();

    log::set_logger(&app_logger::AppLogger).unwrap();

    log::set_max_level(log::LevelFilter::Debug);

    match matches.subcommand() {
        Some(("clean", _)) => {
            println!("Cleaning up");
            unreal::clean();
        }
        Some(("clean-ddc", _)) => {
            println!("Cleaning DDC");
            unreal::clean_ddc();
        }
        Some(("pua", _)) => {
            unreal::count_uproperty_config("TODO");
        }
        Some(("p4-info", _)) => {
            match perforce::run_p4_info() {
                Ok(output) => println!("Output:\n{}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        _ => unreachable!()
    }
}
