use std:: {thread, time, path::Path, env};
use winrt_notification:: {Duration, Sound, Toast};


fn main() {

    loop {
        let now = time::SystemTime::now();
        let time_now = now.duration_since(time::UNIX_EPOCH).expect("fuck time somehow went backwards");

        println!(
            "[{}] Time to take a 20 second eye rest. Look at least 20 feet away for 20 seconds.", 
            time_now.as_secs()
        );

        let assets_path_string = String::from(env::current_dir().unwrap_or_default().to_string_lossy() + "\\src\\assets");

        let zero_two_image_path = assets_path_string.to_owned() + "\\zero_two_eyes.gif";
        let thumbs_up_image_path = assets_path_string.to_owned() + "\\anime_thumbs_up.gif";

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("Time to take an eye break!")
            .text1("Look at least 20 feet away for 20 seconds.")
            .sound(Some(Sound::SMS))
            .hero(Path::new(&zero_two_image_path), "Zero Two blinking eyes")
            .duration(Duration::Long)
            .show()
            .expect("unable to toast for some weird reason");

        thread::sleep(time::Duration::from_millis(25 * 1000));

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("You can continue now :)")
            .text1("20 seconds up you may continue.")
            .sound(Some(Sound::Reminder))
            .hero(Path::new(&thumbs_up_image_path), "Zero Two blinking eyes")
            .duration(Duration::Short)
            .show()
            .expect("unable to toast for some weird reason");


        thread::sleep(time::Duration::from_millis((20 * 60) * 1000));
    }

}
