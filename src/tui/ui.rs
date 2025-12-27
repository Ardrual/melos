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
    // Vertical tracker: columns = tracks, rows = beats (time flows top to bottom)
    render_tracker_vertical(frame, area, state);
}

fn render_tracker_vertical(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " Tracker: Beat {}-{} / {} ",
            state.scroll_beat + 1,
            (state.scroll_beat + state.beats_visible).min(state.total_beats),
            state.total_beats
        ))
        .border_style(Style::default().fg(Color::White));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if inner_area.width < 10 || inner_area.height < 2 {
        return;
    }

    // Calculate layout: beat column on left, then track columns
    let beat_col_width: u16 = 5; // "  1 |"
    let track_col_width: u16 = 14; // Enough for "[C4,E4,G4] " etc
    let available_width = inner_area.width.saturating_sub(beat_col_width);
    let visible_tracks = (available_width / track_col_width).max(1) as usize;
    let visible_tracks = visible_tracks.min(state.ir.tracks.len());

    // Calculate visible beats (rows)
    let header_rows: u16 = 1; // Track names header
    let beats_visible = (inner_area.height.saturating_sub(header_rows)) as u32;
    state.beats_visible = beats_visible;

    let ppq = state.ir.ppq;
    let (ts_num, _) = state.time_signature();

    let mut lines: Vec<Line> = Vec::new();

    // Header row: track names
    let mut header_spans: Vec<Span> = Vec::new();
    header_spans.push(Span::styled(
        format!("{:>4} ", "Beat"),
        Style::default().fg(Color::DarkGray),
    ));

    for (i, track) in state.ir.tracks.iter().take(visible_tracks).enumerate() {
        let style = if i == state.selected_track {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        let name = if track.name.len() > (track_col_width as usize - 2) {
            format!("{:.11}.. ", &track.name)
        } else {
            format!("{:width$} ", track.name, width = track_col_width as usize - 1)
        };
        header_spans.push(Span::styled(name, style));
    }
    lines.push(Line::from(header_spans));

    // Beat rows
    for row in 0..beats_visible {
        let beat_num = state.scroll_beat + row;
        if beat_num >= state.total_beats {
            break;
        }

        let beat_start_tick = beat_num * ppq;
        let beat_end_tick = beat_start_tick + ppq;
        let is_measure_start = beat_num % ts_num == 0;

        let mut row_spans: Vec<Span> = Vec::new();

        // Beat number column
        let beat_style = if is_measure_start {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        row_spans.push(Span::styled(
            format!("{:>4} ", beat_num + 1),
            beat_style,
        ));

        // Track columns
        for (track_idx, track) in state.ir.tracks.iter().take(visible_tracks).enumerate() {
            let is_selected = track_idx == state.selected_track;

            // Find notes that start or are active during this beat
            let notes_in_beat: Vec<_> = track
                .events
                .iter()
                .filter_map(|e| match &e.kind {
                    IrEventKind::Note { pitch, duration, .. } => {
                        let note_end = e.time + duration;
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
                format!("{:·<width$}", "", width = track_col_width as usize - 1)
            } else if notes_in_beat.len() == 1 {
                let (pitch, start, _) = notes_in_beat[0];
                let note_name = pitch_to_name(pitch);
                if start >= beat_start_tick && start < beat_end_tick {
                    format!("{:<width$}", note_name, width = track_col_width as usize - 1)
                } else {
                    format!("{:─<width$}", "│", width = track_col_width as usize - 1)
                }
            } else {
                // Chord - show all notes
                let mut chord_notes: Vec<_> = notes_in_beat.iter()
                    .filter(|(_, start, _)| *start >= beat_start_tick && *start < beat_end_tick)
                    .map(|(pitch, _, _)| pitch_to_name(*pitch))
                    .collect();

                if chord_notes.is_empty() {
                    format!("{:─<width$}", "│", width = track_col_width as usize - 1)
                } else {
                    chord_notes.sort();
                    let chord_str = format!("[{}]", chord_notes.join(","));
                    format!("{:<width$}", chord_str, width = track_col_width as usize - 1)
                }
            };

            let style = if is_selected {
                if !notes_in_beat.is_empty() {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Yellow)
                }
            } else if !notes_in_beat.is_empty() {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            row_spans.push(Span::styled(format!("{} ", cell_content), style));
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
        Line::from("  ↑/↓        Scroll timeline by 1 beat"),
        Line::from("  PgUp/PgDn  Scroll timeline by page"),
        Line::from("  [/]        Scroll timeline by 1 measure"),
        Line::from("  Home/End   Jump to start/end"),
        Line::from("  ←/→        Select previous/next track"),
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
            " [{}] Track {}: {} | ↑↓:Scroll ←→:Track []:Measure | q:Quit Tab:View h:Help",
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
