pub struct Logger;

impl Logger {
    pub fn log_message(msg: &str) {
        println!("{msg}");
    }
}

pub fn main() {
    Logger::log_message("Hello, World!");
}
