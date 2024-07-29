use notify_rust::Notification;

pub fn main() -> Result<(), notify_rust::error::Error> {
    Notification::new()
        .summary("Test Summary")
        .body("This is a dummy error. Testing notification")
        .show()?;
    Ok(())
}
//fn main() {
//println!("Hello, world!");
//}
//
