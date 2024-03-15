use notify_rust::Notification;

fn main() {
    println!("Hello, world!");
    let _ = Notification::new()
        .appname("My super application")
        .summary("Your New Title") // Change this line
        .body("This is a notification from Rust!")
        .icon(r"C:\Users\leonc\Documents\Fork\AutoGitSync\github-icon-clipart-7.png")
        .show();
}
