use signals_rs::{args, Signal};

fn main() {
    let mut signal = Signal::new();

    signal.connect(&|args| {
        let secret_msg = args.get::<&str>(0).unwrap().to_owned();
        let secret_num = args.get::<u32>(1).unwrap().to_owned();
        let info_msg = args.get::<&str>(2).unwrap().to_owned();
        
        println!(
            "this signal was invoked with the secret message: '{}'",
            secret_msg
        );

        println!(
            "this signal was also invoked with as secret number: '{}'",
            secret_num
        );

        println!(
            "{}",
            info_msg
        );
    });

    signal.fire(None, Some(args! {
        "Hello, signals!",
        69u32,
        "this is an example of a signal using the args! macro"
    }));

    signal.disconnect(None);
    signal.destroy();
}
