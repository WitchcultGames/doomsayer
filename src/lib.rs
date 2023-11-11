pub enum LogLevel {
    Info,
    Warning,
    Error,
    Fatal,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $message:expr) => {
        println!("[{}] [{}:{}:{}] - {}", stringify!($level), file!(), line!(), column!(), $message);
    }
}
