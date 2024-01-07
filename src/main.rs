use std:: {thread, time, env::var, fs};
use notify_rust::Notification;

const APP_DATA_DIR: &str = "/eye-break";

fn main() {

    let take_break = create_notification(
        "Time to take an eye break!",
        "Look at least 20 feet away for 20 seconds.",
        find_image("take_break".into())
    );

    let break_done = create_notification(
        "You can continue now :)",
        "20 seconds up you may continue.",
        find_image("break_done".into())
    );

    loop {
        let now = time::SystemTime::now();
        let time_now = now.duration_since(time::UNIX_EPOCH).expect("fuck time somehow went backwards");

        println!(
            "[{}] Time to take a 20 second eye rest. Look at least 20 feet away for 20 seconds.", 
            time_now.as_secs()
        );

        take_break.show().expect("unable to toast for some weird reason");

        thread::sleep(time::Duration::from_millis(25 * 1000)); // Sleep for 25 seconds.

        break_done.show().expect("unable to toast for some weird reason");

        thread::sleep(time::Duration::from_millis((20 * 60) * 1000)); // Sleep for 20 minutes.
    }

}

fn create_notification(title: &str, message: &str, image_path: Option<String>) -> Notification{
    let mut notification = Notification::new();
    notification.summary(title);
    notification.body(message);
    notification.icon(&get_path(Some("/icon.ico".into())));

    if !image_path.is_none() {
        notification.image_path(&image_path.unwrap());
    }

    notification
}

fn get_path(target_file: Option<String>) -> String {
    let target_file = target_file.unwrap_or("".into()).to_owned();

    if cfg!(target_os = "windows") {
        var("AppData").expect("Failed to find Windows AppData environment variable!") + &(APP_DATA_DIR.to_owned() + &target_file).replace("/", r"\")
    } else { // If you want MacOS support, contribute please.
        var("HOME").expect("Failed to find HOME environment variable! Are you on Linux?") + "/.config" + &APP_DATA_DIR + &target_file
    }
}

fn find_image(image_name: String) -> Option<String> {
    let app_data_files = fs::read_dir(get_path(None)).expect(
        "Failed to read appdata directory!"
    );

    for file in app_data_files {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_string();

        if file_name.contains(&image_name) {
            return Some(file.path().to_str().unwrap().to_string());
        }
    }

    None
}