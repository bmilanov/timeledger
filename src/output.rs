use std::fmt::Arguments;

pub trait Output {
    fn write(&self, args: Arguments);
}

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

pub trait Log {
    fn log(&self, level: LogLevel, args: Arguments);
}

pub trait Out: Output + Log {}
