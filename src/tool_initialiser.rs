use std::io::{self, Write};

use crate::utilities::secret_manager;

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

    let token = user_input_handler("Please enter your GitHub Personal Access Token:");
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
    match secret_manager::set_ssh_key_path(&ssh_key_path) {
        Ok(_) => println!("SSH private key path is stored successfully!"),
        Err(e) => {
            println!("Failed to store the ssh private key path: {} \nAbort init, please run init command to retry", e);
            return;
        }
    };

    println!("Init is done! You can now use the add command to add the git repositories you want to watch.");
}

fn user_input_handler(message: &str) -> String {
    println!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_matches('\n').to_string();
    println!("Your input: {}", input);
    input
}
