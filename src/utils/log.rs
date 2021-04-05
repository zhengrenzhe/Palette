use colored::*;

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

pub fn info(msg: &str) {
    println!("{} {}", wrap(LogType::Info), msg)
}

pub fn error(msg: &str) {
    println!("{} {}", wrap(LogType::Error), msg)
}

pub fn success(msg: &str) {
    println!("{} {}", wrap(LogType::Success), msg)
}

pub fn warning(msg: &str) {
    println!("{} {}", wrap(LogType::Warning), msg)
}
