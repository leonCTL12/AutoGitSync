use crate::cross_platform_constant;
use crate::data_structures::config::Config;
use crate::utilities::file_system::{
    create_file_recursively, is_git_repository, is_path_exist, read_file_to_string,
    write_string_to_file,
};

pub fn remove_watched_folder(folder: &str) {
    let mut config = read_config();
    config.remove_watching_folder(folder);
    match write_config(config) {
        Ok(_) => println!("config is updated successfully!"),
        Err(e) => panic!("Failed to remove {}: {}", folder, e),
    }
}

pub fn add_watched_folder(folder: &str) {
    if !is_git_repository(folder) {
        println!("{} is not a git repository, it will be ignored", folder);
        return;
    }

    let mut config = read_config();
    config.insert_watching_folder(folder.to_string());
    match write_config(config) {
        Ok(_) => println!("{} is added to the watch list", folder),
        Err(e) => panic!("Failed to store {}: {}", folder, e),
    }
}

pub fn list_watched_folder() {
    let config = read_config();
    if config.watching_folders.is_empty() {
        println!("No folder is being watched");
        return;
    }

    println!("Watching {} Folders:", config.watching_folders.len());
    for folder in config.watching_folders {
        println!("{}", folder);
    }
}

//Read-only, by the executor
pub fn read_config() -> Config {
    //Step 1: Get Config Path for different os
    let config_path = match cross_platform_constant::get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => {
            panic!("Failed to get config path: {}", e)
        }
    };

    if !is_path_exist(&config_path) {
        return Config::new();
    }

    let s = match read_file_to_string(&config_path) {
        Ok(s) => s,
        Err(e) => {
            panic!("Failed to read the config file : {}", e)
        }
    };

    if s.is_empty() {
        return Config::new();
    }

    match serde_json::from_str(&s) {
        Ok(config) => config,
        Err(e) => {
            println!("Failed to parse the config file : {}", e);
            Config::new()
        }
    }
}

fn write_config(config: Config) -> Result<(), String> {
    //Step 1: Get Config Path for different os
    let config_path = match cross_platform_constant::get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => return Err(format!("Failed to get config path: {}", e)),
    };

    //Step 2: Check if the config path is exist, if not create one
    if !is_path_exist(&config_path) {
        match create_file_recursively(&config_path) {
            Ok(_) => println!("Created a new config file"),
            Err(e) => return Err(format!("Failed to create a new directory: {}", e)),
        };
    }

    //Step 3: Serialise config to json
    let config_json = match serde_json::to_string(&config) {
        Ok(config_json) => config_json,
        Err(e) => return Err(format!("Failed to serialise config to json: {}", e)),
    };

    //Step 4: Write the json to the config file
    write_string_to_file(&config_path, config_json)
}
