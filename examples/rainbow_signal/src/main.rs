use colored::Colorize;
use rand::prelude::*;
use signals_rs::Signal;
use std::fmt::Write;

fn main() {
    println!("Hello, signals!");

    // First, we instantiate a new signal.
    let rainbow_time = &mut Signal::new();

    // Then we provide a connection callback to our signal.
    let (conn, id) = rainbow_time.connect(&|| {
        let orig_msg = "ooo, rainbows!";
        let mut secret_msg: String = String::new();
        let mut rng = rand::thread_rng();

        for (_, char) in orig_msg.chars().enumerate() {
            let char_string = char.to_string();
            let choices = vec!["red", "blue", "purple", "yellow", "green", "magenta"];

            let mut choices_idx: Vec<usize> = (0..choices.len()).collect();

            choices_idx.shuffle(&mut rng);

            let chosen_color = choices[choices_idx[0]];
            let chosen_color_string = match chosen_color {
                "red" => char_string.red(),
                "blue" => char_string.blue(),
                "purple" => char_string.purple(),
                "yellow" => char_string.yellow(),
                "green" => char_string.green(),
                "magenta" => char_string.magenta(),
                &_ => panic!("well, this wasn't supposed to happen..."), // out of bounds str
            };

            write!(secret_msg, "{}", chosen_color_string).unwrap();
        }

        println!("psst... {}", secret_msg.bold());
    });

    // Now, we can fire this when we're ready and have our callback execute!
    conn.fire(None);

    // Post firing, we must clean up the signal that we no longer use.
    conn.disconnect(Some(id));
    rainbow_time.destroy();
}
