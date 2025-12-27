use std::path::PathBuf;
use crate::ast::Score;
use crate::ir::{IrScore, IrEventKind};

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
    // Tracker state
    pub scroll_beat: u32,      // Current scroll position in beats
    pub total_beats: u32,      // Total score duration in beats
    pub beats_visible: u32,    // How many beats fit on screen (set during render)
}

impl AppState {
    pub fn new(score_path: PathBuf, ast: Score, ir: IrScore) -> Self {
        let total_beats = Self::calculate_total_beats(&ir);
        Self {
            score_path,
            ast,
            ir,
            view_mode: ViewMode::Score,
            selected_track: 0,
            running: true,
            scroll_beat: 0,
            total_beats,
            beats_visible: 16, // Default, updated during render
        }
    }

    fn calculate_total_beats(ir: &IrScore) -> u32 {
        let mut max_tick: u32 = 0;
        for track in &ir.tracks {
            for event in &track.events {
                let end_tick = match &event.kind {
                    IrEventKind::Note { duration, .. } => event.time + duration,
                    _ => event.time,
                };
                max_tick = max_tick.max(end_tick);
            }
        }
        // Convert ticks to beats (PPQ = 480 ticks per quarter note)
        (max_tick / ir.ppq) + 1
    }

    pub fn scroll_left(&mut self, amount: u32) {
        self.scroll_beat = self.scroll_beat.saturating_sub(amount);
    }

    pub fn scroll_right(&mut self, amount: u32) {
        let max_scroll = self.total_beats.saturating_sub(self.beats_visible);
        self.scroll_beat = (self.scroll_beat + amount).min(max_scroll);
    }

    pub fn scroll_home(&mut self) {
        self.scroll_beat = 0;
    }

    pub fn scroll_end(&mut self) {
        self.scroll_beat = self.total_beats.saturating_sub(self.beats_visible);
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
