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
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Watch { folder } => {
            println!("{} is being watched...", folder);
        }
    }
}
