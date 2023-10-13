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

use clap::Parser;
use log::error;

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
        Some(("nuke", args)) => {
            let current_dir = std::env::current_dir().expect("Failed to get the current directory");

            match args.try_get_many::<String>("files").unwrap(){
                Some(files) => {
                    let file_names: Vec<&str> = files.map(|s| s.as_str()).collect::<Vec<_>>();
                    println!("Nuking {} for files {}", current_dir.display(), file_names.join(", "));
                    match utils::remove_unwanted_files(&current_dir, &file_names) {
                        Ok(_) => println!("Successfully removed files"),
                        Err(e) => {
                            error!("Error: {}", e);
                            std::process::exit(1);
                        }
                    };
                }
                None => {
                    eprintln!("Error: file extensions are required 'zrrp nuke -f ext -f ext2'", );
                    std::process::exit(1);
                }
            }
        }
        Some(("clean-ddc", _)) => {
            println!("Cleaning DDC");
            unreal::clean_ddc();
        }
        Some(("pua", args)) => {
            match std::env::current_dir() {
                Ok(dir) => if let Some(path) = dir.to_str() {

                    let mut final_path = path.to_string();

                    if let Ok(arg) = args.try_get_one::<String>("folder") {
                        if let Some(folder) = arg {
                           final_path.push_str(format!("/{}", folder).as_str());
                        }
                    }
                    println!("Counting UPROPERTY(Config) instances in {}", final_path);
                    unreal::count_uproperty_config(final_path.as_str());
                },
                Err(e) => error!("Error: {}", e)
            }
        }
        Some(("p4-info", _)) => {
            match perforce::run_p4_info() {
                Ok(output) => println!("Output:\n{}", output),
                Err(e) => error!("Error: {}", e),
            }
        }
        _ => unreachable!()
    }
}
