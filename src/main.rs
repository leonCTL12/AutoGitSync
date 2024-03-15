mod backup_executor;
mod config_manager;
mod cross_platform_constant;
mod data_structures;
mod file_change_watcher;
mod gitignore_wrapper;
mod repository_instance;
mod temp_clone_repo;
mod tool_initialiser;
mod utilities;
use single_instance::SingleInstance;
use structopt::StructOpt;
use utilities::{file_system, secret_manager};

const APP_NAME: &str = "GIT_AUTO_BACKUP";
#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Initialize the tools")]
    Init,
    #[structopt(about = "Watches a Folder")]
    Add {
        #[structopt(help = "The folder to backup when it is updated")]
        folder: String,
    },
    #[structopt(about = "Watch all the git repo in this folder (not recursive)")]
    AddWorkspace {
        #[structopt(help = "The root folder to watch all the git repos in it")]
        folder: String,
    },
    #[structopt(about = "List the watched folders")]
    List,
    #[structopt(about = "Remove a watched folder")]
    Remove {
        #[structopt(help = "The folder to remove from the watch list")]
        folder: String,
    },
    #[structopt(about = "Remove all the watched folders")]
    Clean,
    #[structopt(about = "Start to periodically backup the watched folders")]
    Run,
    #[structopt(about = "Store the ssh private key path")]
    SetSSH {
        #[structopt(help = "The ssh private key path")]
        ssh_key_path: String,
    },
    #[structopt(about = "Store the Personal Access Token")]
    SetPAT {
        #[structopt(help = "The personal access token")]
        token: String,
    },
    #[structopt(about = "Delete the stored ssh private key path")]
    DeleteSSH,
    #[structopt(about = "Delete the stored Personal Access Token")]
    DeletePAT,
    #[structopt(about = "Set the frequency of the backup")]
    SetBackupFreq {
        #[structopt(help = "The frequency of the backup in minutes")]
        frequency: u64,
    },
    #[structopt(
        about = "How long should the file considered as changed after the last change (in minutes)"
    )]
    SetChangeBuffer {
        #[structopt(help = "The buffer time in minutes")]
        buffer_time: u64,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Init => {
            tool_initialiser::init();
        }
        Command::Add { folder } => {
            config_manager::add_watched_folder(&folder);
        }
        Command::AddWorkspace { folder } => {
            let repos = match file_system::get_sub_folders(&folder) {
                Ok(repos) => repos,
                Err(e) => {
                    println!("Failed to get sub folders: {}", e);
                    return;
                }
            };
            if repos.is_empty() {
                println!("No git repository is found in {}", folder);
                return;
            }

            for repo in repos {
                config_manager::add_watched_folder(&repo);
            }
        }
        Command::List => {
            config_manager::list_watched_folder();
        }
        Command::Remove { folder } => {
            config_manager::remove_watched_folder(&folder);
        }
        Command::Clean => {
            config_manager::clean_watched_folder();
        }
        Command::Run => {
            let instance = SingleInstance::new(APP_NAME).unwrap();
            if !instance.is_single() {
                println!("Another instance is already running.");
                return;
            }
            println!("Start to periodically backup the watched folders");
            backup_executor::BackupExecutor::new().start();
        }
        Command::SetSSH { ssh_key_path } => match secret_manager::set_ssh_key_path(&ssh_key_path) {
            Ok(_) => println!("SSH private key path is stored successfully!"),
            Err(e) => println!("Failed to store the ssh private key path: {}", e),
        },
        Command::SetPAT { token } => match secret_manager::set_personal_access_token(&token) {
            Ok(_) => println!("Personal Access Token is stored successfully!"),
            Err(e) => println!("Failed to store the personal access token: {}", e),
        },
        Command::DeleteSSH => match secret_manager::delete_ssh_key_path() {
            Ok(_) => println!("SSH private key path is deleted successfully!"),
            Err(e) => println!("Failed to delete the ssh private key path: {}", e),
        },
        Command::DeletePAT => match secret_manager::delete_personal_access_token() {
            Ok(_) => println!("Personal Access Token is deleted successfully!"),
            Err(e) => println!("Failed to delete the personal access token: {}", e),
        },
        Command::SetBackupFreq { frequency } => {
            config_manager::set_backup_frequency(frequency);
        }
        Command::SetChangeBuffer { buffer_time } => {
            config_manager::set_change_buffer_time(buffer_time);
        }
    }
}
