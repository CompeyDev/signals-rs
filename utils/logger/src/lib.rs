use chrono;
use colored::Colorize;

#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Info,
    Warning,
    Error
}

pub fn log(scope: Scope, msg: &str) {
    let now = chrono::Local::now().format("%d/%m/%Y %H:%M:%S");

    match scope {
        Scope::Info => println!("[{} :: {}] -> {}", now, "info".green().bold(), msg),
        Scope::Warning => println!("[{} :: {}] -> {}", now, "warn".yellow().bold(), msg),
        Scope::Error => println!("[{} :: {}] -> {}", now, "error".red().bold(), msg),
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