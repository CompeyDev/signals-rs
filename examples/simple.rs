use signals_rs::Signal;

fn main() {
    let mut some_signal = Signal::new();

    let (connection, connection_id) =
        some_signal.connect(&|_| println!("This signal has been fired, continuing..."));

    // We can now fire this connection to execute its registered callback.
    // Do note that in a more complicated scenario with multiple connections
    // to a signal, a connection_id parameter must be passed.
    connection.fire(None, None);

    // This "disconnects" from a signal and removes the registered callback, as it is no longer required.
    connection.disconnect(Some(connection_id));

    // Signals can be destroyed or dropped too.
    some_signal.destroy();
}
