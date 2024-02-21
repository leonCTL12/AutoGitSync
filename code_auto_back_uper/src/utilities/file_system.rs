use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub fn is_git_repository(path: &str) -> bool {
    let git_path = Path::new(path).join(".git");
    git_path.exists()
}

pub fn is_path_exist(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn read_file_to_string(path: &str) -> Result<String, String> {
    let mut file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            println!("file is read successfully!");
            Ok(s)
        }
        Err(e) => {
            panic!("Failed to read the config file : {}", e)
        }
    }
}

pub fn write_string_to_file(path: &str, content: String) -> Result<(), String> {
    let mut file = match OpenOptions::new().write(true).create(true).open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_file_recursively(path: &str) -> Result<(), String> {
    let path = Path::new(path);
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
