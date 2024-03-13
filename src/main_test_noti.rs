use notify_rust::Notification;

//TODO: Test this on windows
fn main() {
    println!("Hello, world!");
    let _ = Notification::new()
        .summary("Hello")
        .body("This is a notification from Rust!")
        .show();
}
