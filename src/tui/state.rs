use std::path::PathBuf;
use crate::ast::Score;
use crate::ir::IrScore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Score,
    Tracks,
    Help,
}

pub struct AppState {
    pub score_path: PathBuf,
    pub ast: Score,
    pub ir: IrScore,
    pub view_mode: ViewMode,
    pub selected_track: usize,
    pub running: bool,
}

impl AppState {
    pub fn new(score_path: PathBuf, ast: Score, ir: IrScore) -> Self {
        Self {
            score_path,
            ast,
            ir,
            view_mode: ViewMode::Score,
            selected_track: 0,
            running: true,
        }
    }

    pub fn title(&self) -> Option<&str> {
        for header in &self.ast.headers {
            if let crate::ast::Header::Title(t) = header {
                return Some(t);
            }
        }
        None
    }

    pub fn tempo(&self) -> u32 {
        for header in &self.ast.headers {
            if let crate::ast::Header::Tempo(t) = header {
                return *t;
            }
        }
        120 // default
    }

    pub fn time_signature(&self) -> (u32, u32) {
        for header in &self.ast.headers {
            if let crate::ast::Header::TimeSignature(num, den) = header {
                return (*num, *den);
            }
        }
        (4, 4) // default
    }

    pub fn key_signature(&self) -> Option<(&str, &str)> {
        for header in &self.ast.headers {
            if let crate::ast::Header::KeySignature(root, scale) = header {
                return Some((root, scale));
            }
        }
        None
    }

    pub fn track_count(&self) -> usize {
        self.ir.tracks.len()
    }

    pub fn track_names(&self) -> Vec<&str> {
        self.ir.tracks.iter().map(|t| t.name.as_str()).collect()
    }
}
