use std:: {thread, time, env};
use notify_rust::Notification;

fn main() {

    loop {
        let now = time::SystemTime::now();
        let time_now = now.duration_since(time::UNIX_EPOCH).expect("fuck time somehow went backwards");

        println!(
            "[{}] Time to take a 20 second eye rest. Look at least 20 feet away for 20 seconds.", 
            time_now.as_secs()
        );

        let assets_path_string = String::from(env::current_dir().unwrap_or_default().to_string_lossy() + "/src/assets");

        let icon_path = assets_path_string.to_owned() + "/icon.ico";
        let zero_two_image_path = assets_path_string.to_owned() + "/zero_two_eyes.gif";
        let thumbs_up_image_path = assets_path_string.to_owned() + "/anime_thumbs_up.gif";

        let mut take_break_noti = Notification::new();
        take_break_noti.summary("Time to take an eye break!");
        take_break_noti.body("Look at least 20 feet away for 20 seconds.");
        take_break_noti.icon(&icon_path);

        let mut continue_noti = Notification::new();
        continue_noti.summary("You can continue now :)");
        continue_noti.body("20 seconds up you may continue.");
        continue_noti.icon(&icon_path);

        if cfg!(windows) {
            take_break_noti.image_path(&zero_two_image_path);
            continue_noti.image_path(&thumbs_up_image_path);
        }

        take_break_noti.show().expect("unable to toast for some weird reason");

        thread::sleep(time::Duration::from_millis(25 * 1000)); // Sleep for 25 seconds.

        continue_noti.show().expect("unable to toast for some weird reason");

        thread::sleep(time::Duration::from_millis((20 * 60) * 1000)); // Sleep for 20 minutes.
    }

}
