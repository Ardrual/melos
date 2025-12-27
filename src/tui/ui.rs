use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::ir::IrEventKind;
use super::state::{AppState, ViewMode};

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    render_header(frame, chunks[0], state);
    render_main(frame, chunks[1], state);
    render_status_bar(frame, chunks[2], state);
}

fn render_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let title = state.title().unwrap_or("Untitled");
    let tempo = state.tempo();
    let (ts_num, ts_den) = state.time_signature();
    let key = state
        .key_signature()
        .map(|(r, s)| format!("{} {}", r, s))
        .unwrap_or_else(|| "C Major".to_string());

    let header_text = format!(
        "{} | Tempo: {} BPM | Time: {}/{} | Key: {} | Tracks: {}",
        title,
        tempo,
        ts_num,
        ts_den,
        key,
        state.track_count()
    );

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Melos ")
                .border_style(Style::default().fg(Color::Cyan)),
        );

    frame.render_widget(header, area);
}

fn render_main(frame: &mut Frame, area: Rect, state: &mut AppState) {
    match state.view_mode {
        ViewMode::Score => render_score_view(frame, area, state),
        ViewMode::Tracks => render_tracks_view(frame, area, state),
        ViewMode::Help => render_help_view(frame, area),
    }
}

fn render_score_view(frame: &mut Frame, area: Rect, state: &mut AppState) {
    // Layout: track labels on left, tracker grid on right
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(16), // Track labels
            Constraint::Min(0),     // Tracker grid
        ])
        .split(area);

    render_track_labels(frame, chunks[0], state);
    render_tracker_grid(frame, chunks[1], state);
}

fn render_track_labels(frame: &mut Frame, area: Rect, state: &AppState) {
    let inner = Block::default()
        .borders(Borders::ALL)
        .title(" Tracks ")
        .border_style(Style::default().fg(Color::DarkGray));

    let inner_area = inner.inner(area);
    frame.render_widget(inner, area);

    // Skip header row, render track names
    if inner_area.height < 2 {
        return;
    }

    let tracks: Vec<Line> = state
        .ir
        .tracks
        .iter()
        .enumerate()
        .take((inner_area.height - 1) as usize) // -1 for beat header
        .map(|(i, track)| {
            let style = if i == state.selected_track {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let name = if track.name.len() > 12 {
                format!("{}...", &track.name[..9])
            } else {
                track.name.clone()
            };
            Line::from(Span::styled(name, style))
        })
        .collect();

    // Add empty first line (for beat header alignment)
    let mut lines = vec![Line::from("")];
    lines.extend(tracks);

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner_area);
}

fn render_tracker_grid(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " Beat {}-{} / {} ",
            state.scroll_beat + 1,
            (state.scroll_beat + state.beats_visible).min(state.total_beats),
            state.total_beats
        ))
        .border_style(Style::default().fg(Color::White));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if inner_area.width < 4 || inner_area.height < 2 {
        return;
    }

    // Calculate how many beats fit (each beat = 4 chars wide)
    let beat_width: u16 = 4;
    let beats_visible = (inner_area.width / beat_width) as u32;
    state.beats_visible = beats_visible;

    let ppq = state.ir.ppq;
    let (ts_num, _) = state.time_signature();

    // Build the grid content
    let mut lines: Vec<Line> = Vec::new();

    // Header row: beat numbers
    let mut header_spans: Vec<Span> = Vec::new();
    for b in 0..beats_visible {
        let beat_num = state.scroll_beat + b + 1;
        if beat_num > state.total_beats {
            break;
        }
        // Highlight measure boundaries
        let is_measure_start = (beat_num - 1) % ts_num == 0;
        let style = if is_measure_start {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        header_spans.push(Span::styled(format!("{:>3} ", beat_num), style));
    }
    lines.push(Line::from(header_spans));

    // Track rows
    for (track_idx, track) in state.ir.tracks.iter().enumerate() {
        if lines.len() >= inner_area.height as usize {
            break;
        }

        let is_selected = track_idx == state.selected_track;
        let mut row_spans: Vec<Span> = Vec::new();

        for b in 0..beats_visible {
            let beat_num = state.scroll_beat + b;
            if beat_num >= state.total_beats {
                break;
            }

            let beat_start_tick = beat_num * ppq;
            let beat_end_tick = beat_start_tick + ppq;

            // Find notes that start or are active during this beat
            let notes_in_beat: Vec<_> = track
                .events
                .iter()
                .filter_map(|e| match &e.kind {
                    IrEventKind::Note { pitch, duration, .. } => {
                        let note_end = e.time + duration;
                        // Note overlaps with this beat
                        if e.time < beat_end_tick && note_end > beat_start_tick {
                            Some((*pitch, e.time, *duration))
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect();

            let cell_content = if notes_in_beat.is_empty() {
                "  · ".to_string()
            } else if notes_in_beat.len() == 1 {
                let (pitch, start, _) = notes_in_beat[0];
                let note_name = pitch_to_name(pitch);
                // Show note start vs sustain
                if start >= beat_start_tick && start < beat_end_tick {
                    format!("{:>3} ", note_name)
                } else {
                    format!(" ── ") // Sustain
                }
            } else {
                // Chord or multiple notes
                format!("{:>3} ", notes_in_beat.len())
            };

            let is_measure_start = beat_num % ts_num == 0;
            let style = if is_selected {
                if !notes_in_beat.is_empty() {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else if is_measure_start {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::DarkGray)
                }
            } else if !notes_in_beat.is_empty() {
                Style::default().fg(Color::Green)
            } else if is_measure_start {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            row_spans.push(Span::styled(cell_content, style));
        }

        lines.push(Line::from(row_spans));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner_area);
}

fn pitch_to_name(pitch: u8) -> String {
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octave = (pitch / 12) as i8 - 1;
    let note = (pitch % 12) as usize;
    format!("{}{}", note_names[note], octave)
}

fn render_tracks_view(frame: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .ir
        .tracks
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let style = if i == state.selected_track {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let event_count = track.events.len();
            let note_count = track
                .events
                .iter()
                .filter(|e| matches!(e.kind, crate::ir::IrEventKind::Note { .. }))
                .count();

            ListItem::new(format!(
                " {}. {} (ch {}) - {} events, {} notes",
                i + 1,
                track.name,
                track.channel,
                event_count,
                note_count
            ))
            .style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Tracks ")
            .border_style(Style::default().fg(Color::White)),
    );

    frame.render_widget(list, area);
}

fn render_help_view(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("  Navigation", Style::default().fg(Color::Yellow))),
        Line::from("  ←/→        Scroll timeline by 1 beat"),
        Line::from("  [/]        Scroll timeline by 1 measure"),
        Line::from("  Home/End   Jump to start/end"),
        Line::from("  ↑/↓        Select previous/next track"),
        Line::from("  1-9        Select track by number"),
        Line::from(""),
        Line::from(Span::styled("  Views", Style::default().fg(Color::Yellow))),
        Line::from("  Tab        Switch view (Score/Tracks/Help)"),
        Line::from("  h, ?       Show this help"),
        Line::from("  q, Esc     Quit"),
        Line::from(""),
        Line::from(Span::styled(
            "  Coming Soon (Phase 3+)",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(Span::styled(
            "  Space      Play/Pause",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  m          Mute track",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  o          Solo track",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .border_style(Style::default().fg(Color::White)),
    );

    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    let mode_str = match state.view_mode {
        ViewMode::Score => "SCORE",
        ViewMode::Tracks => "TRACKS",
        ViewMode::Help => "HELP",
    };

    let track_name = state.ir.tracks.get(state.selected_track)
        .map(|t| t.name.as_str())
        .unwrap_or("None");

    let status = match state.view_mode {
        ViewMode::Score => format!(
            " [{}] Track {}: {} | ←→:Scroll []:Measure ↑↓:Track | q:Quit Tab:View h:Help",
            mode_str,
            state.selected_track + 1,
            track_name
        ),
        _ => format!(
            " [{}] Track {}: {} | q:Quit Tab:Switch View h:Help",
            mode_str,
            state.selected_track + 1,
            track_name
        ),
    };

    let paragraph = Paragraph::new(status)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}
