use signals_rs::{Signal, Arguments};



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

    signal_args.insert(0, "Hello, signals!");

    signal.fire(None, Some(signal_args));

    signal.disconnect(None);
    signal.destroy();
}
