use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Print,
    // Add other commands here
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Print => println!("Hello, World!"),
        // Handle other commands here
    }
}
