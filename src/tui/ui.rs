use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::state::{AppState, ViewMode};

pub fn render(frame: &mut Frame, state: &AppState) {
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

fn render_main(frame: &mut Frame, area: Rect, state: &AppState) {
    match state.view_mode {
        ViewMode::Score => render_score_view(frame, area, state),
        ViewMode::Tracks => render_tracks_view(frame, area, state),
        ViewMode::Help => render_help_view(frame, area),
    }
}

fn render_score_view(frame: &mut Frame, area: Rect, state: &AppState) {
    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Score View - Tracker display coming in Phase 2",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(format!("  File: {}", state.score_path.display())),
        Line::from(format!("  Parts: {}", state.ast.parts.len())),
        Line::from(""),
    ];

    // Add part summaries
    let mut lines = content;
    for (i, part) in state.ast.parts.iter().enumerate() {
        let measure_count = part
            .content
            .iter()
            .filter(|b| matches!(b, crate::ast::MeasureBlock::Measure(_)))
            .count();
        lines.push(Line::from(format!(
            "  {}. {} ({}) - {} measures",
            i + 1,
            part.name,
            part.instrument,
            measure_count
        )));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Score ")
            .border_style(Style::default().fg(Color::White)),
    );

    frame.render_widget(paragraph, area);
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
        Line::from("  q, Esc     Quit"),
        Line::from("  Tab        Switch view (Score/Tracks/Help)"),
        Line::from("  h, ?       Show this help"),
        Line::from("  1-9        Select track"),
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
            "  s          Stop",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  [, ]       Seek backward/forward",
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

    let status = format!(
        " [{}] Track: {} | q:Quit Tab:Switch View h:Help",
        mode_str,
        state.selected_track + 1
    );

    let paragraph = Paragraph::new(status)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}
