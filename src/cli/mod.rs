use clap::{Command};

pub fn create_cli() -> Command {
    Command::new("zrrp")
        .about("A developers best friend")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clean").about("Cleans up the unreal project under the current directory"),
        )
        .subcommand(
            Command::new("clean-ddc").about("Cleans up the DerivedDataCache folder under the current directory"),
        )
        .subcommand(
            Command::new("pua").about("Counts UPROPERTY(Config) instances in the given path")
                .arg(clap::Arg::new("folder"))
                .allow_missing_positional(true),
        )
        .subcommand(
            Command::new("nuke").about("Nukes the current directory for unwanted files")
                .arg(clap::Arg::new("files")
                    .long("files")
                    .short('f')
                    .action(clap::ArgAction::Append)
                )
        )
}