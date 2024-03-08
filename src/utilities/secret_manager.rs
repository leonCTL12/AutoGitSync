use keyring::Entry;

const SERVICE_NAME: &str = "AUTO_GIT_SYNC";
const SSH_KEY_PATH: &str = "SSH_KEY_PATH";
const PERSONAL_ACCESS_TOKEN: &str = "PERSONAL_ACCESS_TOKEN";

pub fn set_ssh_key_path(path: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, SSH_KEY_PATH)?;
    entry.set_password(path)
}

pub fn set_personal_access_token(token: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, PERSONAL_ACCESS_TOKEN)?;
    entry.set_password(token)
}

pub fn get_ssh_key_path() -> Result<String, keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, SSH_KEY_PATH)?;
    entry.get_password()
}

pub fn get_personal_access_token() -> Result<String, keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, PERSONAL_ACCESS_TOKEN)?;
    entry.get_password()
}

pub fn delete_ssh_key_path() -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, SSH_KEY_PATH)?;
    if let Err(e) = entry.get_password() {
        println!("ssh private key path is not found");
        return Err(e);
    }

    entry.delete_password()
}

pub fn delete_personal_access_token() -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, PERSONAL_ACCESS_TOKEN)?;
    if let Err(e) = entry.get_password() {
        println!("Personal Access Token is not found");
        return Err(e);
    }
    entry.delete_password()
}
