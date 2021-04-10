use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use tui::widgets::ListState;

use crate::utils::log::LogType;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

pub struct Log {
    pub label: String,
    pub content: String,
    pub log_type: LogType,
}

impl Log {
    pub fn new(label: String, content: String, log_type: LogType) -> Log {
        Log {
            label,
            content,
            log_type,
        }
    }
}

pub struct State {
    pub should_exit: bool,
    pub gui: bool,
    pub logs: StatefulList<Log>,
}

impl State {
    pub fn new() -> State {
        State {
            should_exit: false,
            gui: false,
            logs: StatefulList::new(),
        }
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_exit = true
        };
    }

    pub fn set_gui(&mut self, gui: bool) {
        self.gui = gui
    }

    pub fn put_log(&mut self, label: String, content: String, log_type: LogType) {
        self.logs.items.push(Log::new(label, content, log_type));
    }
}

lazy_static! {
    // global app state
    pub static ref PALETTE_STATE: Arc<RwLock<State>> = Arc::new(RwLock::new(State::new()));
}
