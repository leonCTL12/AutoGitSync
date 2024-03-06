use copy_dir::copy_dir;

pub fn copy_directory(directory_path: &str) -> Result<String, String> {
    let destination = format!("{}_temp_clone", directory_path);
    match copy_dir(directory_path, &destination) {
        Ok(_) => {
            println!("{} has been copied to {}", directory_path, destination);
            Ok(destination)
        }
        Err(e) => Err(format!(
            "Failed to copy {} to {}: {}",
            directory_path, destination, e,
        )),
    }
}
