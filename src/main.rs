use single_instance::SingleInstance;
use std::thread;
use std::time::Duration;

fn main() {
    let instance = SingleInstance::new("my_app_name").unwrap();
    if instance.is_single() {
        println!("Running application...");
        thread::sleep(Duration::from_secs(10));
        // Your application's code goes here.
        panic!("Test Crash");
        println!("Application finished.");
    } else {
        println!("Another instance is already running.");
        return;
    }
}
