use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use tui::{Frame, Terminal};

use crate::core::scheduler::state::{Log, PALETTE_STATE};
use crate::utils::log::events::{Event, Events};
use crate::utils::log::LogType;

pub fn draw() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let event = Events::new();
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.clear().unwrap();

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .constraints([
                        // progress 8 rows
                        Constraint::Length(8),
                        // log rows
                        Constraint::Min(0),
                        // footer
                        Constraint::Length(3),
                    ])
                    .split(f.size());

                draw_progress(f, chunks[0]);
                draw_log(f, chunks[1]);
                draw_info(f, chunks[2]);
            })
            .unwrap();

        match event.next() {
            Event::Input(key) => {
                if let Key::Char(c) = key {
                    PALETTE_STATE.write().unwrap().on_key(c);
                }
            }
            Event::Tick => {}
        }

        if PALETTE_STATE.read().unwrap().should_exit {
            break;
        }
    }
}

fn draw_progress<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Progress");
    frame.render_widget(block, area);
}

fn create_log(l: &Log) -> Spans {
    let style = match l.log_type {
        LogType::Info => Style::default().fg(Color::Cyan),
        LogType::Success => Style::default().fg(Color::Green),
        LogType::Error => Style::default().fg(Color::Red),
        LogType::Warning => Style::default().fg(Color::Yellow),
    };
    Spans::from(vec![
        Span::styled(l.label.to_string(), style),
        Span::raw(l.content.to_string()),
    ])
}

fn draw_log<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let ps = PALETTE_STATE.read().unwrap();
    let logs: Vec<ListItem> = ps
        .logs
        .items
        .iter()
        .map(|l| ListItem::new(Spans::from(create_log(l))))
        .collect();
    let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("Log"));
    frame.render_stateful_widget(logs, area, &mut ListState::default());
}

fn draw_info<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Info");

    let text = vec![Spans::from(vec![
        Span::from("press "),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::from(" to exit palette"),
    ])];

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
