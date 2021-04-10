use colored::*;
use std::thread;

use crate::core::scheduler::state::PALETTE_STATE;

pub enum LogType {
    Info,
    Success,
    Error,
    Warning,
}

fn get_log_type(log_type: LogType) -> String {
    format!(
        "[Palette {}]",
        match log_type {
            LogType::Info => "Info",
            LogType::Success => "Success",
            LogType::Error => "Error",
            LogType::Warning => "Warning",
        }
    )
}

fn get_thread_info() -> String {
    let info = match thread::current().name() {
        None => format!("{:?}", thread::current().id()),
        Some(name) => format!("{}:{:?}", name, thread::current().id()),
    };

    let str = format!("[{}]", info);

    if PALETTE_STATE.read().unwrap().gui {
        str
    } else {
        str.bright_blue().to_string()
    }
}

fn colored(msg: String, log_type: LogType) -> String {
    match log_type {
        LogType::Info => msg.cyan().to_string(),
        LogType::Success => msg.green().to_string(),
        LogType::Error => msg.red().to_string(),
        LogType::Warning => msg.yellow().to_string(),
    }
}

fn auto_switch(label: String, msg: &str, log_type: LogType) {
    if PALETTE_STATE.read().unwrap().gui {
        PALETTE_STATE
            .write()
            .unwrap()
            .put_log(label, msg.to_string(), log_type);
    } else {
        println!("{} {}", colored(label, log_type), msg);
    }
}

pub fn info(msg: &str) {
    auto_switch(
        format!("{}{} ", get_thread_info(), get_log_type(LogType::Info)),
        msg,
        LogType::Info,
    );
}

pub fn error(msg: &str) {
    auto_switch(
        format!("{}{} ", get_thread_info(), get_log_type(LogType::Error)),
        msg,
        LogType::Error,
    );
}

pub fn success(msg: &str) {
    auto_switch(
        format!("{}{} ", get_thread_info(), get_log_type(LogType::Success)),
        msg,
        LogType::Success,
    );
}

pub fn warning(msg: &str) {
    auto_switch(
        format!("{}{} ", get_thread_info(), get_log_type(LogType::Warning)),
        msg,
        LogType::Warning,
    );
}
