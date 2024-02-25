use std::path::PathBuf;
const APP_NAME: &str = "AutoGitSync";
const CONFIG_FILE_NAME: &str = "config.json";

#[cfg(target_os = "windows")]
fn get_config_path_prefix() -> String {
    use std::env;

    let username = match env::var("USERNAME") {
        Ok(username) => username,
        Err(_) => {
            panic!("Failed to get username")
        }
    };

    format!("C:\\Users\\{}\\AppData\\Roaming\\", username)
}

#[cfg(target_os = "macos")]
fn get_config_path_prefix() -> String {
    expand_home_path("~/.config")
}

#[cfg(target_os = "linux")]
fn get_config_path_prefix() -> String {
    //TODO: test linux
    expand_home_path("~/.config")
}

fn expand_home_path(path: &str) -> String {
    if path.starts_with('~') {
        let home = match dirs::home_dir() {
            Some(home) => home,
            None => {
                panic!("Failed to get home directory");
            }
        };

        let mut path = path.to_string();
        path.remove(0);
        path.insert_str(0, home.to_str().unwrap());
        path
    } else {
        path.to_string()
    }
}

pub fn get_config_path() -> Result<String, String> {
    let config_path_prefix = &get_config_path_prefix();
    let mut path = PathBuf::from(config_path_prefix);
    path.push(APP_NAME);
    path.push(CONFIG_FILE_NAME);

    match path.to_str() {
        Some(p) => Ok(p.to_string()),
        None => Err("Fail to form a config path".to_string()),
    }
}
