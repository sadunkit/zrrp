mod unreal;
mod utils;
mod perforce;
mod app_logger;
mod cli;

fn main() {
    use log::error;

    let matches = cli::create_cli().get_matches();

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
                    let _file_stats = unreal::count_uproperty_config(final_path.as_str());

                    // TODO: Record the fileStats to a file or show it through a nice ui
                    // working with an editor could prove useful (i.e. Rider for Unreal)
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
        _ => {}
    }
}
