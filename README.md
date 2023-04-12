<div align="left"><img src="https://raw.githubusercontent.com/CompeyDev/signals-rs/main/assets/signals-rs_banner_light.png#gh-light-mode-only" width="50%" ></div>
<div align="left"><img src="https://raw.githubusercontent.com/CompeyDev/signals-rs/main/assets/signals-rs_banner_dark.png#gh-dark-mode-only" width="50%" ></div>

A lua(u)-inspired implementation of signals/events in rust.

A signal is a global state switch which can be used as a gateway for conditional code execution. Signals can be activated by first "connecting" to them with a callback, then "firing" them to toggle their state.
```rs
let mut some_signal = Signal::new();

let (connection, connection_id) = some_signal.connect(&|| println!("This signal has been fired, continuing..."));

// We can now fire this connection to execute its registered callback. 
// Do note that in a more complicated scenario with multiple connections to a signal, a connection_id parameter must be passed.
connection.fire(None);

// This "disconnects" from a signal and removes the registered callback, as it is no longer required.
connection.disconnect(Some(connection_id)); 
    
// Signals can be destroyed or dropped too.
some_signal.destroy();
```

Signals are especially useful as lightweight events for global state sync-ups.

# Installation
In order to install signals-rs, first switch to a nightly channel (`rustup toolchain install nightly`) for your rust compiler and then you can add it as such to your dependencies section in `Cargo.toml`:

```toml
[dependencies]
signals_rs = { git = "https://github.com/CompeyDev/signals-rs.git", rev = "2a236fe" }
```

# Features
As of now, this crate consists 1 feature, namely `log`. This feature is opt-in and enables progress logging for various activities, classified into "errors", "warns", and "info" logs. Useful for debugging purposes.

It can be enabled as such:
```toml
[dependencies]
signals_rs = { git = "https://github.com/CompeyDev/signals-rs.git", rev = "2a236fe", features = ["log"] }
```
