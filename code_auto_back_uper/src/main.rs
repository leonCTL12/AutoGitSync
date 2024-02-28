mod backup_executor;
mod config_manager;
mod cross_platform_constant;
mod data_structures;
mod file_change_watcher;
mod repository_instance;
mod utilities;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Watches a Folder")]
    Add {
        #[structopt(help = "The folder to backup when it is updated")]
        folder: String,
    },
    #[structopt(about = "List the watched folders")]
    List,
    #[structopt(about = "Remove a watched folder")]
    Remove {
        #[structopt(help = "The folder to remove from the watch list")]
        folder: String,
    },
    #[structopt(about = "Start to periodically backup the watched folders")]
    Run,
    #[structopt(about = "Store the Personal Access Token")]
    Auth {
        #[structopt(help = "The personal access token")]
        token: String,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Add { folder } => {
            config_manager::store_watched_folder(&folder);
        }
        Command::List => {
            config_manager::list_watched_folder();
        }
        Command::Remove { folder } => {
            config_manager::remove_watched_folder(&folder);
        }
        Command::Run => {
            println!("Start to periodically backup the watched folders");
            backup_executor::start();
        }
        Command::Auth { token } => {
            //TODO Implement this
        }
    }
}
