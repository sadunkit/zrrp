mod unreal;

use clap::{ Command };
use unreal::clean;

fn cli() -> Command {
    Command::new("zrrp")
        .about("A developers best friend")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clean").about("Cleans up unreal projects under the current directory"),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("clean", _)) => {
            println!("Cleaning up");
            clean();
        }
        _ => unreachable!(),
    }
}
