use anymap::AnyMap as HashMap;
use logger::{log, Scope};
use rand::random;

/// # signals-rs
/// `signals-rs` is a Rust-inspired implementation similar to lua(u) signals/events.
///
/// A signal is a global state switch which can be used as a gateway for conditional code execution.
/// Signals can be activated by first "connecting" to them with a callback, then "firing" them to toggle their state.
///
/// ```
/// use signals_rs::Signal;
///
/// fn main() {
///     let mut some_signal = Signal::new();
///
///     let (_, connection_id) = some_signal.connect(&|| println!("This signal has been fired, continuing..."));
///     some_signal.disconnect(connection_id); // This "disconnects" from a signal and removes the registered callback, as it is no longer required.
///
///     some_signal.destroy(); // Signals can be destroyed or dropped too.
/// }
/// ```
///
/// Signals are especially useful as lightweight events for global state sync-ups.
pub struct Signal {
    pub connections: HashMap<String>,
    pub destroyed: bool,
}

impl Signal {
    /// `Signal::new` instantiates and returns a new Signal, which can then be made use of.
    ///
    /// ```
    ///  use signal_rs::Signal;
    ///
    ///  let mut signal = Signal::new();
    /// ```
    pub fn new() -> Signal {
        let mut connections = HashMap::new();

        connections.insert("last_accessed_connection".to_string(), "Unknown");

        #[cfg(feature = "log")]
        log(Scope::Info, "constructed and instantianted signal!");

        return Signal {
            connections,
            destroyed: false,
        };
    }

    /// `Signal.connect` registers a supplied callback closure/function to be executed on a signal mutation activated
    /// by `Signal.fire`. A signal can be connected to with multiple callbacks.
    ///
    /// Every callback returns a tuple with a `Signal` and Callback ID, which can be stored for future use with
    /// `Signal.fire` & `Signal.disconnect`.
    ///
    /// ```
    /// use signal_rs::Signal;
    ///
    /// let mut signal = Signal::new();
    ///
    /// let (_, first_callback_id) = signal.connect(&|| println!("received signal fire from callback #1!"));
    ///
    /// fn signal_callback() {
    ///     println!("received signal fire from callback #2!");
    /// }
    ///
    /// let (_, second_callback_id) = signal.connect(&signal_callback);
    ///
    /// println!("#1 -> {}", first_callback_id);
    /// println!("#2 -> {}", second_callback_id);
    /// ```
    pub fn connect(&mut self, callback: &'static dyn Fn()) -> (&mut Signal, String) {
        if !self.destroyed {
            let connection_id: String = format!("{:x}", random::<u32>());

            #[cfg(feature = "log")]
            log(Scope::Info, format!("generating connection {}", connection_id.clone()).as_str());

            let mut connection_meta: HashMap<&str> = HashMap::new();

            connection_meta.insert("callback", callback);
            connection_meta.insert("is_primary", false);

            #[cfg(feature = "log")]
            log(Scope::Info, format!("generating connection meta for {}", connection_id.clone()).as_str());

            self.connections
                .insert(connection_id.to_owned(), connection_meta);
            self.connections.insert(
                "last_accessed_connection".to_string(),
                connection_id.to_owned(),
            );

            #[cfg(feature = "log")]
            log(Scope::Info, format!("successfully created connection {}", connection_id.clone()).as_str());

            return (self, connection_id);
        } else {
            panic!("fatal: signal has been destroyed!")
        }
    }

    /// `Signal.disconnect` disconnects a registered callback from a signal. This prevents execution of a certain callback
    /// once it's been disconnected. An optional `connection_id` parameter may be provided which can be used to disconnect
    /// a specific connection instead of the last accessed connection. Providing a connection_id is highly recommended and
    /// minimizes the risk of an unregistered connection from being disconnected.
    ///
    /// ```
    /// use signal_rs::Signal;
    ///
    /// let mut signal = Signal::new();
    ///
    /// let (_, callback_id) = signal.connect(&|| println!("received signal fire from callback"));
    ///
    /// signal.disconnect(Some(callback_id));
    /// ```
    pub fn disconnect(&mut self, connection_id: Option<String>) {
        if !self.destroyed {
            let target_id = match connection_id {
                Some(conn_id) => conn_id,
                None => {
                    let conn_id = self
                        .connections
                        .get::<String>("last_accessed_connection".to_string())
                        .unwrap()
                        .to_owned();

                    if conn_id == "Unknown".to_string() {
                        panic!("no last known connection, please manually supply one")
                    } else {
                        conn_id
                    }
                }
            };

            

            #[cfg(feature = "log")]
            let logger_id = target_id.clone();

            #[cfg(feature = "log")]
            log(Scope::Info, format!("calculated target {}", logger_id).as_str());

            *self
                .connections
                .get_mut::<String>("last_accessed_connection".to_string())
                .unwrap() = "Unknown".to_string();
            self.connections
                .remove::<HashMap<&str>>(target_id)
                .expect("non existent connection id supplied");

                #[cfg(feature = "log")]
                log(Scope::Info, format!("successfully disconnected from {}", logger_id).as_str());
        } else {
            panic!("fatal: signal has been destroyed!")
        }
    }

    /// `Signal.fire` fires a callback from a connection registered beforehand. An optional `connection_id` may be provided.
    /// In such a case where no `connection_id` is provided, it will default to the previously accessed connection. It is
    /// recommended practice to provide a `connection_id` as this minimizes the risk of an unregistered connection from being
    /// fired.
    ///
    /// ```
    /// use signal_rs::Signal;
    ///
    /// let mut signal = Signal::new();
    ///
    /// let (_, callback_id) = signal.connect(&|| println!("received signal fire from callback"));
    ///
    /// signal.fire(Some(callback_id));
    /// signal.disconnect(Some(callback_id));
    /// ```

    pub fn fire(&mut self, connection_id: Option<String>) {
        let conn_id = match connection_id {
            Some(target_id) => target_id,
            None => self
                .connections
                .get::<String>("last_accessed_connection".to_string())
                .unwrap()
                .to_owned(),
        };

        #[cfg(feature = "log")]
        let logger_id = conn_id.clone();

        #[cfg(feature = "log")]
        log(Scope::Info, format!("calculated connection target {}", logger_id).as_str());

        let conn_meta = self
            .connections
            .get::<HashMap<&str>>(conn_id)
            .expect("non existent connection id supplied");

            #[cfg(feature = "log")]
            log(Scope::Info, format!("generating connection {:#?}", logger_id).as_str());

        conn_meta.get::<&'static dyn Fn()>("callback").unwrap()();
    }

    /// `Signal.destroy` destroys the signal and all registered callbacks are rendered dysfunctional. It is good practice
    /// to destroy a signal once it no longer serves its purpose. Destroying a signal is equivalent to dropping it as the
    /// `Drop` trait has been implemented for `Signal`.
    ///
    /// ```
    /// use signal_rs::Signal;
    ///
    /// let mut signal = Signal::new();
    ///
    /// signal.destroy()
    /// ```
    pub fn destroy(&mut self) {
        drop(self);
    }
}

impl Drop for Signal {
    fn drop(&mut self) {
        #[cfg(feature = "log")]
        log(Scope::Info, format!("dropping self: Signal").as_str());

        self.destroyed = true;

        self.connections = HashMap::new();

        #[cfg(feature = "log")]
        log(Scope::Info, format!("sucessfully ran drop cleanup").as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_works() {
        let mut signal = Signal::new();

        signal.connect(&|| println!("received signal fire!"));

        assert_eq!(
            signal
                .connections
                .get::<String>("last_accessed_connection".to_string())
                .unwrap()
                .to_owned()
                != "Unknown".to_string(),
            true
        );
    }

    #[test]
    fn disconnection_works() {
        let mut signal = Signal::new();

        signal.connect(&|| println!("received signal fire!"));

        signal.disconnect(None);
    }

    #[test]
    fn destruction_works() {
        let mut signal = Signal::new();

        signal.connect(&|| println!("received signal fire!"));

        signal.destroy();

        assert_eq!(signal.destroyed, true);
    }

    #[test]
    fn fire_works() {
        let mut signal = Signal::new();

        let (_, conn_id) = signal.connect(&|| println!("received signal fire!"));

        signal.fire(Some(conn_id));
    }
}
