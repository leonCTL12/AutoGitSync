use keyring::{Entry, Result};

const SERVICE_NAME: &str = "AUTO_GIT_SYNC";
const SSH_KEY_PATH: &str = "SSH_KEY_PATH";
const PERSONAL_ACCESS_TOKEN: &str = "PERSONAL_ACCESS_TOKEN";

pub fn set_ssh_key_path(path: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, SSH_KEY_PATH)?;
    entry.set_password(path)
}

pub fn set_personal_access_token(token: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, PERSONAL_ACCESS_TOKEN)?;
    entry.set_password(token)
}

pub fn get_ssh_key_path() -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, SSH_KEY_PATH)?;
    entry.get_password()
}

pub fn get_personal_access_token() -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, PERSONAL_ACCESS_TOKEN)?;
    entry.get_password()
}
