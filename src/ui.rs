use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, List, ListItem, ListState, Paragraph, Row, Table, TableState, Wrap},
};
use crate::app::{App, AppState, ConfigSection, ResultsButton};
use crate::config::{Difficulty, TimeLimit, SEQ_LEN_OPTIONS};
use crate::history;
use crate::sequences::SequenceKind;

pub fn ui(f: &mut Frame, app: &App) {
    match app.state {
        AppState::Configuration => draw_configuration(f, app),
        AppState::Testing       => draw_testing(f, app),
        AppState::Results       => draw_results(f, app),
        AppState::History       => draw_history(f, app),
    }
}

// ─── helpers ─────────────────────────────────────────────────────────────────

fn section_style(active: bool) -> Style {
    if active { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::DarkGray) }
}

fn button_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn render_button(label: &str, focused: bool) -> Span<'static> {
    let text = format!("  {}  ", label);
    Span::styled(text, button_style(focused))
}

// ─── Config ──────────────────────────────────────────────────────────────────

fn draw_configuration(f: &mut Frame, app: &App) {
    let area = f.area();
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),  // title
            Constraint::Min(10),    // body
            Constraint::Length(2),  // help
        ])
        .split(area);

    let title = Paragraph::new(Line::from(vec![
        Span::styled("sequences-rs", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("  ─  Sequence Trainer"),
    ])).alignment(Alignment::Center);
    f.render_widget(title, outer[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(outer[1]);

    // ── left: sequence types ──
    let types_items: Vec<ListItem> = SequenceKind::ALL.iter().map(|kind| {
        let checked = if app.config.enabled.contains(kind) { "✓" } else { " " };
        ListItem::new(format!(" [{}] {}", checked, kind.label()))
    }).collect();

    let types_list = List::new(types_items)
        .block(Block::default()
            .title("  Sequence Types")
            .borders(Borders::ALL)
            .border_style(section_style(app.config_section == ConfigSection::Types)))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    if app.config_section == ConfigSection::Types {
        list_state.select(Some(app.cursor));
    }
    f.render_stateful_widget(types_list, body[0], &mut list_state);

    // ── right column ──
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // difficulty (3 options)
            Constraint::Length(6),  // sequence length (4 options)
            Constraint::Length(8),  // time limit (6 options)
            Constraint::Length(4),  // presets
            Constraint::Min(3),     // start button
        ])
        .split(body[1]);

    // difficulty
    let diff_items: Vec<ListItem> = Difficulty::ALL.iter().map(|d| {
        let sel = app.config.difficulty == *d;
        let style = if sel { Style::default().fg(Color::Green) } else { Style::default() };
        ListItem::new(format!("  {} {}", if sel { "●" } else { "○" }, d.label())).style(style)
    }).collect();
    let diff_active = app.config_section == ConfigSection::Difficulty;
    let diff_list = List::new(diff_items)
        .block(Block::default().title("  Difficulty").borders(Borders::ALL).border_style(section_style(diff_active)))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));
    let mut diff_state = ListState::default();
    if diff_active {
        diff_state.select(Some(Difficulty::ALL.iter().position(|d| *d == app.config.difficulty).unwrap_or(0)));
    }
    f.render_stateful_widget(diff_list, right[0], &mut diff_state);

    // sequence length
    let len_items: Vec<ListItem> = SEQ_LEN_OPTIONS.iter().map(|&n| {
        let sel = app.config.seq_len == n;
        let style = if sel { Style::default().fg(Color::Green) } else { Style::default() };
        ListItem::new(format!("  {} {} terms", if sel { "●" } else { "○" }, n)).style(style)
    }).collect();
    let len_active = app.config_section == ConfigSection::Length;
    let len_list = List::new(len_items)
        .block(Block::default().title("  Sequence Length").borders(Borders::ALL).border_style(section_style(len_active)))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));
    let mut len_state = ListState::default();
    if len_active {
        len_state.select(Some(SEQ_LEN_OPTIONS.iter().position(|&n| n == app.config.seq_len).unwrap_or(0)));
    }
    f.render_stateful_widget(len_list, right[1], &mut len_state);

    // time limit
    let time_items: Vec<ListItem> = TimeLimit::ALL.iter().map(|t| {
        let sel = app.config.time_limit == *t;
        let style = if sel { Style::default().fg(Color::Green) } else { Style::default() };
        ListItem::new(format!("  {} {}", if sel { "●" } else { "○" }, t.label())).style(style)
    }).collect();
    let time_active = app.config_section == ConfigSection::TimeLimit;
    let time_list = List::new(time_items)
        .block(Block::default().title("  Time Limit").borders(Borders::ALL).border_style(section_style(time_active)))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));
    let mut time_state = ListState::default();
    if time_active {
        time_state.select(Some(TimeLimit::ALL.iter().position(|t| *t == app.config.time_limit).unwrap_or(0)));
    }
    f.render_stateful_widget(time_list, right[2], &mut time_state);

    // presets (static, key shortcuts)
    let preset_text = Paragraph::new(Line::from(vec![
        Span::styled(" [O]", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::raw(" Optiver   "),
        Span::styled("[F]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Flow Traders"),
    ]))
    .block(Block::default().title("  Presets").borders(Borders::ALL))
    .wrap(Wrap { trim: false });
    f.render_widget(preset_text, right[3]);

    // start button
    let start_focused = app.config_section == ConfigSection::Start;
    let start_style = if start_focused {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let start_label = if app.config.enabled.is_empty() {
        " ✗  No types selected "
    } else {
        "  ▶  START  "
    };
    let start_btn = Paragraph::new(start_label)
        .block(Block::default().borders(Borders::ALL).border_style(section_style(start_focused)))
        .style(start_style)
        .alignment(Alignment::Center);
    f.render_widget(start_btn, right[4]);

    // help bar
    let help = Paragraph::new("  ↑↓ navigate   Space toggle   Tab next section   Enter select/start   O/F presets   Q quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, outer[2]);
}

// ─── Testing ─────────────────────────────────────────────────────────────────

fn draw_testing(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(area);

    // score bar
    let score_color = if app.score > 0 { Color::Green } else { Color::White };
    let score = Paragraph::new(format!(
        "  Score: {}   Correct: {}/{}   Elapsed: {}s",
        app.score, app.correct, app.total, app.elapsed_secs()
    )).style(Style::default().fg(score_color).add_modifier(Modifier::BOLD));
    f.render_widget(score, chunks[0]);

    // sequence
    let seq_text = if let Some(ref q) = app.current {
        let mut parts: Vec<Span> = q.visible_terms.iter().map(|t| {
            Span::styled(format!("{}  ", t), Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        }).collect();
        parts.push(Span::styled("__", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        vec![Line::from(parts)]
    } else {
        vec![Line::from("Loading...")]
    };
    let seq_para = Paragraph::new(seq_text)
        .block(Block::default().borders(Borders::ALL).title("  What comes next?"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(seq_para, chunks[1]);

    // input
    let input_para = Paragraph::new(format!("  > {}_", app.input))
        .block(Block::default().borders(Borders::ALL).title("  Answer"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(input_para, chunks[2]);

    // gauge
    let ratio = app.remaining_ratio();
    let gauge_color = if ratio > 0.5 { Color::Green } else if ratio > 0.2 { Color::Yellow } else { Color::Red };
    let remaining = app.config.time_limit.seconds().saturating_sub(app.elapsed_secs());
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(ratio)
        .label(format!("{}s remaining", remaining));
    f.render_widget(gauge, chunks[3]);

    let help = Paragraph::new("  Enter submit   Esc skip   Q end session")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[4]);
}

// ─── Results ─────────────────────────────────────────────────────────────────

fn draw_results(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(4),  // summary
            Constraint::Min(8),     // breakdown
            Constraint::Length(3),  // buttons
        ])
        .split(area);

    // summary
    let accuracy = if app.total > 0 { app.correct as f64 / app.total as f64 * 100.0 } else { 0.0 };
    let header = Paragraph::new(vec![
        Line::from(Span::styled("Results", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(
            format!("Score: {}   Correct: {}/{}   Accuracy: {:.0}%   Time: {}s",
                app.score, app.correct, app.total, accuracy, app.session_duration),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
    ])
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // breakdown by type
    let items: Vec<ListItem> = app.per_kind.iter()
        .filter(|(_, _, t)| *t > 0)
        .map(|(kind, c, t)| {
            let pct = *c as f64 / *t as f64 * 100.0;
            let color = if pct >= 80.0 { Color::Green } else if pct >= 50.0 { Color::Yellow } else { Color::Red };
            ListItem::new(Line::from(vec![
                Span::raw(format!("  {:<16}", kind.label())),
                Span::styled(format!("{}/{}", c, t), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::raw(format!("  ({:.0}%)", pct)),
            ]))
        })
        .collect();
    let breakdown = List::new(items)
        .block(Block::default().title("  Breakdown by type").borders(Borders::ALL));
    f.render_widget(breakdown, chunks[1]);

    // tabbable buttons
    let btn_line = Line::from(vec![
        render_button("Restart", app.results_button == ResultsButton::Restart),
        Span::raw("   "),
        render_button("History", app.results_button == ResultsButton::History),
        Span::raw("   "),
        render_button("Quit",    app.results_button == ResultsButton::Quit),
    ]);
    let btns = Paragraph::new(btn_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(btns, chunks[2]);
}

// ─── History ─────────────────────────────────────────────────────────────────

fn draw_history(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(area);

    let title = Paragraph::new(Span::styled(
        "Score History",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )).alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    if app.history.is_empty() {
        let empty = Paragraph::new("  No sessions recorded yet.")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, chunks[1]);
    } else {
        let yellow_bold = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        let header_row = Row::new([
            Cell::from("When").style(yellow_bold),
            Cell::from("Score").style(yellow_bold),
            Cell::from("Correct").style(yellow_bold),
            Cell::from("Accuracy").style(yellow_bold),
            Cell::from("Difficulty").style(yellow_bold),
            Cell::from("Time").style(yellow_bold),
        ]);

        let data_rows: Vec<Row> = app.history.iter().map(|e| {
            let acc = if e.total > 0 { e.correct as f64 / e.total as f64 * 100.0 } else { 0.0 };
            let acc_color = if acc >= 80.0 { Color::Green } else if acc >= 50.0 { Color::Yellow } else { Color::Red };
            let when = history::format_ago(e.timestamp);
            Row::new([
                Cell::from(when),
                Cell::from(e.score.to_string()).style(Style::default().fg(Color::White)),
                Cell::from(format!("{}/{}", e.correct, e.total)),
                Cell::from(format!("{:.0}%", acc)).style(Style::default().fg(acc_color)),
                Cell::from(e.difficulty.clone()),
                Cell::from(format!("{}s", e.time_limit_secs)),
            ])
        }).collect();

        let col_widths = [
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Min(6),
        ];
        let table = Table::new(data_rows, col_widths)
            .header(header_row)
            .block(Block::default().title(format!("  {} sessions", app.history.len())).borders(Borders::ALL))
            .row_highlight_style(Style::default().bg(Color::DarkGray));

        let mut table_state = TableState::default();
        table_state.select(Some(app.history_scroll));
        f.render_stateful_widget(table, chunks[1], &mut table_state);
    }

    let help = Paragraph::new("  ↑↓ scroll   Esc / Q back to results")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}
