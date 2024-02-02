use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "print" {
        println!("Hello, World!");
    } else {
        println!("Unknown command");
    }
}
