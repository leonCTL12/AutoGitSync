use std::io::{self, Write};

use crate::{
    config_manager,
    utilities::{file_system, secret_manager},
};

const SKIP_KEY: &str = "SKIP";

pub fn init() {
    let mut auth_count = 0;
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

    let pat_result = set_personal_access_token();
    if pat_result.is_err() {
        return;
    } else if pat_result.unwrap() {
        auth_count += 1;
    }

    let ssh_result = set_ssh_path();
    if ssh_result.is_err() {
        return;
    } else if ssh_result.unwrap() {
        auth_count += 1;
    }

    let user_input_backup_frequency = user_input_handler(
        "Please enter the frequency of the backup in minutes: (default is 30, use press enter to use the default value)",
    );

    if !user_input_backup_frequency.is_empty() {
        let backup_frequency: u64 = match user_input_backup_frequency.parse() {
            Ok(frequency) => frequency,
            Err(_) => {
                println!("Default value will be used");
                30
            }
        };

        config_manager::set_backup_frequency(backup_frequency);
    }

    if auth_count < 1 {
        println!("No authentication information is provided. Please provide at least one authentication information.\nAbort init, please run init command to retry");
        return;
    }

    config_manager::set_inited();

    println!("Init is done! You can now use the add command to add the git repositories you want to watch.");
}

fn set_personal_access_token() -> Result<bool, ()> {
    let token = user_input_handler("Please enter your GitHub Personal Access Token:");

    match token {
        token if token == SKIP_KEY => {
            println!("Skipped setting up personal access token.");
            Ok(false)
        }
        token if token.is_empty() => {
            println!("Personal Access Token cannot be empty.\nAbort init, please run init command to retry");
            Err(())
        }
        _ => match secret_manager::set_personal_access_token(&token) {
            Ok(_) => {
                println!("Personal Access Token is stored successfully!");
                Ok(true)
            }
            Err(e) => {
                println!("Failed to store the personal access token: {} \nAbort init, please run init command to retry", e);
                Err(())
            }
        },
    }
}

fn set_ssh_path() -> Result<bool, ()> {
    let ssh_key_path = user_input_handler("Please enter your ssh private key path:")
        .trim()
        .trim_matches('\'')
        .trim_matches('\"')
        .to_string();

    match ssh_key_path {
        ssh_key_path if ssh_key_path == SKIP_KEY => {
            println!("Skipped setting up ssh private key path.");
            Ok(false)
        }
        ssh_key_path if !file_system::is_path_exist(&ssh_key_path) => {
            println!("The ssh private key path does not exist.\nAbort init, please run init command to retry");
            Err(())
        }
        _ => match secret_manager::set_ssh_key_path(&ssh_key_path) {
            Ok(_) => {
                println!("SSH private key path is stored successfully!");
                Ok(true)
            }
            Err(e) => {
                println!("Failed to store the ssh private key path: {} \nAbort init, please run init command to retry", e);
                Err(())
            }
        },
    }
}

fn user_input_handler(message: &str) -> String {
    println!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_matches('\n').to_string()
}
