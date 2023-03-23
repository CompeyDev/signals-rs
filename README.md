<div align="left"><img src="https://raw.githubusercontent.com/CompeyDev/signals-rs/main/assets/signals-rs_banner_light.png#gh-light-mode-only" width="50%" ></div>
<div align="left"><img src="https://raw.githubusercontent.com/CompeyDev/signals-rs/main/assets/signals-rs_banner_dark.png#gh-dark-mode-only" width="50%" ></div>

A lua(u)-inspired implementation of signals/events in rust.

A signal is a global state switch which can be used as a gateway for conditional code execution. Signals can be activated by first "connecting" to them with a callback, then "firing" them to toggle their state.
```rs
use signals_rs::Signal;

fn main() {
    let mut some_signal = Signal::new();

    let (_, connection_id) = some_signal.connect(&|| println!("This signal has been fired, continuing..."));
    
    // This "disconnects" from a signal and removes the registered callback, as it is no longer required.
    some_signal.disconnect(connection_id); 
    
    // Signals can be destroyed or dropped too.
    some_signal.destroy();
}
```

Signals are especially useful as lightweight events for global state sync-ups.
