use std::path::PathBuf;

const APP_NAME: &str = "GitAutoSync";
const CONFIG_FILE_NAME: &str = "config.json";

//TODO: how to figure out username????
#[cfg(target_os = "windows")]
const CONFIG_PATH_PREFIX: &str = "C:\\Users\\YourUsername\\AppData\\Roaming\\";

#[cfg(target_os = "macos")]
const CONFIG_PATH_PREFIX: &str = "~/Library/Preferences/";

#[cfg(target_os = "linux")]
const CONFIG_PATH_PREFIX: &str = "~/.config/";

pub fn get_config_path() -> Result<String, String> {
    let mut path = PathBuf::from(CONFIG_PATH_PREFIX);
    path.push(APP_NAME);
    path.push(CONFIG_FILE_NAME);

    match path.to_str() {
        Some(p) => Ok(p.to_string()),
        None => Err("Fail to form a config path".to_string()),
    }
}
