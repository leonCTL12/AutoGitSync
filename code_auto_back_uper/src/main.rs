mod backup_executor;
mod config_manager;
mod cross_platform_constant;
mod data_structures;
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
    Watch {
        #[structopt(help = "The folder to backup when it is updated")]
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
        Command::Watch { folder } => {
            config_manager::store_watched_folder(&folder);
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
