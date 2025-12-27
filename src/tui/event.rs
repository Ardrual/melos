use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::Receiver<AppEvent>,
    _tx: mpsc::Sender<AppEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap_or(false) {
                    if let Ok(evt) = event::read() {
                        let app_event = match evt {
                            Event::Key(key) => Some(AppEvent::Key(key)),
                            Event::Resize(w, h) => Some(AppEvent::Resize(w, h)),
                            _ => None,
                        };
                        if let Some(e) = app_event {
                            if event_tx.send(e).is_err() {
                                break;
                            }
                        }
                    }
                } else {
                    if event_tx.send(AppEvent::Tick).is_err() {
                        break;
                    }
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<AppEvent> {
        Ok(self.rx.recv()?)
    }
}

pub fn handle_key(key: KeyEvent, state: &mut super::state::AppState) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            state.running = false;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.running = false;
        }
        KeyCode::Tab => {
            state.view_mode = match state.view_mode {
                super::state::ViewMode::Score => super::state::ViewMode::Tracks,
                super::state::ViewMode::Tracks => super::state::ViewMode::Help,
                super::state::ViewMode::Help => super::state::ViewMode::Score,
            };
        }
        KeyCode::Char('h') | KeyCode::Char('?') => {
            state.view_mode = super::state::ViewMode::Help;
        }
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            let track_num = c.to_digit(10).unwrap() as usize;
            if track_num <= state.track_count() {
                state.selected_track = track_num - 1;
            }
        }
        _ => {}
    }
}
