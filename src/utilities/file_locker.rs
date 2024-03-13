use nix::fcntl::{flock, FlockArg};
use std::fs::File;
use std::os::unix::io::AsRawFd;

const LOCK_FILE: &str = "tmp/lockfile.lock";

//TODO: add tmp folder to cross_platform_constant for windows and linux
pub fn try_lock_file() {
    let f = File::create(LOCK_FILE).expect("Unable to create lock file");

    let ret = flock(f.as_raw_fd(), FlockArg::LockExclusiveNonblock);

    match ret {
        Ok(_) => {
            println!("Got lock, running program");
            // Your program here
        }
        Err(_) => {
            println!("Another instance is already running");
            std::process::exit(1);
        }
    }
}
