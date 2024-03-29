use std::io::{self, Write};

use crate::{
    config_manager,
    utilities::{file_system, secret_manager},
};

pub fn init() {
    println!(
        r"
         ___                                    _    ___           _   
        (  _`\                               _ ( )_ (  _`\        (_ ) 
        | ( (_)   _     ___ ___    ___ ___  (_)| ,_)| |_) )   _ _  | | 
        | |  _  /'_`\ /' _ ` _ `\/' _ ` _ `\| || |  | ,__/' /'_` ) | | 
        | (_( )( (_) )| ( ) ( ) || ( ) ( ) || || |_ | |    ( (_| | | | 
        (____/'`\___/'(_) (_) (_)(_) (_) (_)(_)`\__)(_)    `\__,_)(___)"
    );
    println!(
        "\nWelcome to CommitPal!
Never forget to commit again.
This tool will help you create backup branches and push your code to GitHub automatically.
Please follow the prompts to set up your environment.\n
    "
    );

    config_manager::reset();

    let token = user_input_handler("Please enter your GitHub Personal Access Token:");
    if token.is_empty() {
        println!(
            "Personal access token cannot be empty.\nAbort init, please run init command to retry"
        );
        return;
    }
    match secret_manager::set_personal_access_token(&token) {
        Ok(_) => println!("Personal Access Token is stored successfully!"),
        Err(e) => {
            println!("Failed to store the personal access token: {} \nAbort init, please run init command to retry", e);
            return;
        }
    };

    let ssh_key_path = user_input_handler("Please enter your ssh private key path:")
        .trim()
        .trim_matches('\'')
        .trim_matches('\"')
        .to_string();
    if ssh_key_path.is_empty() {
        println!("Ssh key path can't be empty.\nAbort init, please run init command to retry");
        return;
    }
    if !file_system::is_path_exist(&ssh_key_path) {
        println!("The ssh private key path does not exist.\nAbort init, please run init command to retry");
        return;
    }

    match secret_manager::set_ssh_key_path(&ssh_key_path) {
        Ok(_) => println!("SSH private key path is stored successfully!"),
        Err(e) => {
            println!("Failed to store the ssh private key path: {} \nAbort init, please run init command to retry", e);
            return;
        }
    };

    let user_input_backup_frequency = user_input_handler(
        "Please enter the frequency of the backup in minutes: (default is 30, use press enter to use the default value)",
    );

    if !user_input_backup_frequency.is_empty() {
        let backup_frequency: u64 = match user_input_backup_frequency.parse() {
            Ok(frequency) => frequency,
            Err(_) => {
                println!("Invalid input, use the default value 30");
                30
            }
        };

        config_manager::set_backup_frequency(backup_frequency);
    }

    println!("Init is done! You can now use the add command to add the git repositories you want to watch.");
}

fn user_input_handler(message: &str) -> String {
    println!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_matches('\n').to_string()
}
