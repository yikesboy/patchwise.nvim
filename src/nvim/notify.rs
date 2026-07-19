use nvim_oxi::api::{self, types::LogLevel};

pub fn info(message: &str) {
    send(message, LogLevel::Info);
}

pub fn error(message: &str) {
    send(message, LogLevel::Error);
}

fn send(message: &str, level: LogLevel) {
    let _ = api::notify(message, level, &Default::default());
}
