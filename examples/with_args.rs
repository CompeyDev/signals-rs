use signals_rs::{Arguments, Signal};

fn main() {
    let mut signal = Signal::new();

    signal.connect(&|args| {
        let secret_msg = args.get::<&str>(0).unwrap().to_owned();

        println!(
            "this signal was invoked with the secret message: '{}'",
            secret_msg
        );
    });

    let mut signal_args = Arguments::new();

    signal_args.push("Hello, signals!");

    signal.fire(None, Some(signal_args));

    signal.disconnect(None);
    signal.destroy();
}
