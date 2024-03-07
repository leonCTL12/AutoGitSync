use keyring::{Entry, Result};

fn main() -> Result<()> {
    println!("Hello, world!");

    let entry = Entry::new("my_service", "my_name")?;
    entry.set_password("my_personal_access_token")?;
    let password = entry.get_password()?;
    println!("My password is '{}'", password);
    entry.delete_password()?;
    Ok(())
}
