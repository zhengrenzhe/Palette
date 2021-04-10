use colored::*;
use std::thread;

enum LogType {
    Info,
    Success,
    Error,
    Warning,
}

fn wrap(log_type: LogType) -> String {
    let type_str = match log_type {
        LogType::Info => "Info",
        LogType::Success => "Success",
        LogType::Error => "Error",
        LogType::Warning => "Warning",
    };

    let c = format!("[Palette {}]", type_str);

    match log_type {
        LogType::Info => c.cyan().to_string(),
        LogType::Success => c.green().to_string(),
        LogType::Error => c.red().to_string(),
        LogType::Warning => c.yellow().to_string(),
    }
}

fn get_thread_info() -> String {
    let info = match thread::current().name() {
        None => format!("{:?}", thread::current().id()),
        Some(name) => format!("{}:{:?}", name, thread::current().id()),
    };
    format!("[{}]", info).bright_blue().to_string()
}

pub fn info(msg: &str) {
    println!("{}{} {}", get_thread_info(), wrap(LogType::Info), msg)
}

pub fn error(msg: &str) {
    println!("{}{} {}", get_thread_info(), wrap(LogType::Error), msg)
}

pub fn success(msg: &str) {
    println!("{}{} {}", get_thread_info(), wrap(LogType::Success), msg)
}

pub fn warning(msg: &str) {
    println!("{}{} {}", get_thread_info(), wrap(LogType::Warning), msg)
}
