const app_name: &str = "code_auto_back_uper";

//TODO: how to figure out username????
#[cfg(target_os = "windows")]
const CONFIG_PATH_PREFIX: &str = "C:\\Users\\YourUsername\\AppData\\Roaming\\";

#[cfg(target_os = "macos")]
const CONFIG_PATH_PREFIX: &str = "~/Library/Preferences/";

#[cfg(target_os = "linux")]
const CONFIG_PATH_PREFIX: &str = "~/.config/";

fn get_config_path() -> String {
    format!("{}{}", CONFIG_PATH_PREFIX, APP_NAME)
}
