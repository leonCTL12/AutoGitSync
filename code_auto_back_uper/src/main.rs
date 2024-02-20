mod cross_platform_constant;

use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Watches a Folder")]
    Watch {
        #[structopt(help = "The folder to backup when it is updated")]
        folder: String,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Watch { folder } => {
            println!("{} is being watched...", folder);
            store_watched_folder(&folder);
        }
    }
}

fn store_watched_folder(folder: &str) {
    let config_path = match cross_platform_constant::get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("Config path: {}", config_path);

    let config_path = Path::new(&config_path);

    if !config_path.exists() {
        match create_file_recursively(&config_path) {
            Ok(_) => println!("Created a new config file"),
            Err(e) => {
                println!("Failed to create a new directory: {}", e);
                return;
            }
        };
    }
}

fn create_file_recursively(path: &Path) -> Result<(), String> {
    //TODO: Fix this function
    let parent = path.parent().unwrap();

    if !parent.exists() {
        create_file_recursively(parent);
    }

    match File::create(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
