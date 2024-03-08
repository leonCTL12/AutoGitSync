use copy_dir::copy_dir;

use super::file_system::is_path_exist;

pub fn copy_directory(directory_path: &str) -> Result<String, String> {
    let destination = format!("{}_temp_clone", directory_path);

    if is_path_exist(&destination) {
        match std::fs::remove_dir_all(&destination) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "Failed to remove the existing directory {}: {}",
                    destination, e,
                ));
            }
        }
    }
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
