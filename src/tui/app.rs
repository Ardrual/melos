use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::Duration;

use crate::ast::Score;
use crate::ir::IrScore;

use super::event::{handle_key, AppEvent, EventHandler};
use super::state::AppState;
use super::ui;

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_handler: EventHandler,
    state: AppState,
}

impl App {
    pub fn new(score_path: PathBuf, ast: Score, ir: IrScore) -> Result<Self> {
        let terminal = setup_terminal()?;
        let event_handler = EventHandler::new(Duration::from_millis(100));
        let state = AppState::new(score_path, ast, ir);

        Ok(Self {
            terminal,
            event_handler,
            state,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while self.state.running {
            self.terminal.draw(|frame| {
                ui::render(frame, &self.state);
            })?;

            match self.event_handler.next()? {
                AppEvent::Key(key) => {
                    handle_key(key, &mut self.state);
                }
                AppEvent::Tick => {
                    // Future: update playback position
                }
                AppEvent::Resize(_, _) => {
                    // Terminal will redraw automatically
                }
            }
        }

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = restore_terminal(&mut self.terminal);
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn run_tui(score_path: PathBuf, ast: Score, ir: IrScore) -> Result<()> {
    let mut app = App::new(score_path, ast, ir)?;
    app.run()
}
