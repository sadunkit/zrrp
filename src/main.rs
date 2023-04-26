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
        _ => unreachable!()
    }
}
