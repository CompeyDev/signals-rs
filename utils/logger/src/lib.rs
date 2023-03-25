use chrono;

#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Info,
    Warning,
    Error
}

pub fn log(scope: Scope, msg: &str) {
    let now = chrono::Local::now().format("%d/%m/%Y %H:%M:%S:%MS");

    match scope {
        Scope::Info => println!("[{} :: info] -> {}", now, msg),
        Scope::Warning => println!("[{} :: warn] -> {}", now, msg),
        Scope::Error => println!("[{} :: error] -> {}", now, msg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        log(Scope::Info, "this is an info log");
        log(Scope::Warning, "this is an warn log");
        log(Scope::Error, "this is an error log");
    }
}