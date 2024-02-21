mod cross_platform_constant;
mod data_structures;

use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};
use structopt::StructOpt;

use data_structures::config::{self, Config};

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
    let mut config = read_config();
    config.insert_watching_folder(folder.to_string());
    match write_config(config) {
        Ok(_) => println!("{} is stored successfully!", folder),
        Err(e) => panic!("Failed to store {}: {}", folder, e),
    }
}

fn read_config() -> Config {
    //Step 1: Get Config Path for different os
    let config_path = match cross_platform_constant::get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => {
            panic!("Failed to get config path: {}", e)
        }
    };

    //Step 2: Check if the config path is exist, if not return a new config
    let config_path = Path::new(&config_path);
    if !config_path.exists() {
        return Config::new();
    }

    //Step 3: Read the config file
    let mut file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open the config file : {}", e)
        }
    };

    //Step 4: See if it contains any data
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            print!("config file is read successfully!")
        }
        Err(e) => {
            panic!("Failed to read the config file : {}", e)
        }
    }

    //Step 5: create new config if empty, else parse the data
    if s.is_empty() {
        return Config::new();
    }

    let config = match serde_json::from_str(&s) {
        Ok(config) => config,
        Err(e) => {
            println!("Failed to parse the config file : {}", e);
            return Config::new();
        }
    };
    config
}

fn write_config(config: Config) -> Result<(), String> {
    //Step 1: Get Config Path for different os
    let config_path = match cross_platform_constant::get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => return Err(format!("Failed to get config path: {}", e)),
    };

    //Step 2: Check if the config path is exist, if not create one
    let config_path = Path::new(&config_path);
    if !config_path.exists() {
        match create_file_recursively(config_path) {
            Ok(_) => println!("Created a new config file"),
            Err(e) => return Err(format!("Failed to create a new directory: {}", e)),
        };
    }

    //Step 3: Serialise config to json
    let config_json = match serde_json::to_string(&config) {
        Ok(config_json) => config_json,
        Err(e) => return Err(format!("Failed to serialise config to json: {}", e)),
    };

    //Step 4: write it back to the file
    let mut file = match OpenOptions::new().write(true).open(config_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open the config file : {}", e)),
    };
    match file.write_all(config_json.as_bytes()) {
        Ok(_) => println!("Config file is written successfully!"),
        Err(e) => return Err(format!("Failed to write the config file : {}", e)),
    };

    Ok(())
}

fn create_file_recursively(path: &Path) -> Result<(), String> {
    let parent = match path.parent() {
        Some(parent) => parent,
        None => return Ok(()),
    };

    if !parent.exists() {
        println!("Such directory does not exist: {:?}", parent);
        match fs::create_dir_all(parent) {
            Ok(_) => println!("Created a new directory: {:?}", parent),
            Err(e) => return Err(e.to_string()),
        }
    }

    match File::create(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
