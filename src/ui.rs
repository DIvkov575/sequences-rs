use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
};
use crate::app::{App, AppState, ConfigSection};
use crate::config::{Difficulty, TimeLimit};
use crate::sequences::SequenceKind;

pub fn ui(f: &mut Frame, app: &App) {
    match app.state {
        AppState::Configuration => draw_configuration(f, app),
        AppState::Testing => draw_testing(f, app),
        AppState::Results => draw_results(f, app),
    }
}

fn draw_configuration(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),   // title
            Constraint::Min(10),     // main split
            Constraint::Length(3),   // help bar
        ])
        .split(area);

    // Title
    let title = Paragraph::new("sequences-rs  ─  Sequence Trainer")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[0]);

    // Main layout: types | settings
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[1]);

    // --- Sequence types list ---
    let types_items: Vec<ListItem> = SequenceKind::ALL.iter().map(|kind| {
        let checked = if app.config.enabled.contains(kind) { "✓" } else { " " };
        ListItem::new(format!("[{}] {}", checked, kind.label()))
    }).collect();

    let types_style = if app.config_section == ConfigSection::Types {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let types_list = List::new(types_items)
        .block(Block::default()
            .title("  Sequence Types  [Tab to switch section]")
            .borders(Borders::ALL)
            .border_style(types_style))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    if app.config_section == ConfigSection::Types {
        list_state.select(Some(app.cursor));
    }
    f.render_stateful_widget(types_list, main_chunks[0], &mut list_state);

    // --- Settings column ---
    let settings_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),   // difficulty
            Constraint::Length(7),   // time
            Constraint::Min(2),      // presets
        ])
        .split(main_chunks[1]);

    // Difficulty
    let diff_style = if app.config_section == ConfigSection::Difficulty {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let diff_items: Vec<ListItem> = Difficulty::ALL.iter().enumerate().map(|(_i, d)| {
        let selected = app.config.difficulty == *d;
        let sym = if selected { "●" } else { "○" };
        let style = if selected { Style::default().fg(Color::Green) } else { Style::default() };
        ListItem::new(format!("  {} {}", sym, d.label())).style(style)
    }).collect();
    let diff_list = List::new(diff_items)
        .block(Block::default().title("  Difficulty").borders(Borders::ALL).border_style(diff_style))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));
    let mut diff_state = ListState::default();
    if app.config_section == ConfigSection::Difficulty {
        diff_state.select(Some(Difficulty::ALL.iter().position(|d| *d == app.config.difficulty).unwrap_or(0)));
    }
    f.render_stateful_widget(diff_list, settings_chunks[0], &mut diff_state);

    // Time limit
    let time_style = if app.config_section == ConfigSection::TimeLimit {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let time_items: Vec<ListItem> = TimeLimit::ALL.iter().map(|t| {
        let selected = app.config.time_limit == *t;
        let sym = if selected { "●" } else { "○" };
        let style = if selected { Style::default().fg(Color::Green) } else { Style::default() };
        ListItem::new(format!("  {} {}", sym, t.label())).style(style)
    }).collect();
    let time_list = List::new(time_items)
        .block(Block::default().title("  Time Limit").borders(Borders::ALL).border_style(time_style))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));
    let mut time_state = ListState::default();
    if app.config_section == ConfigSection::TimeLimit {
        time_state.select(Some(TimeLimit::ALL.iter().position(|t| *t == app.config.time_limit).unwrap_or(0)));
    }
    f.render_stateful_widget(time_list, settings_chunks[1], &mut time_state);

    // Presets
    let preset_text = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  [O]", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(" Optiver preset   "),
            Span::styled("[F]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Flow Traders preset"),
        ]),
    ])
    .block(Block::default().title("  Presets").borders(Borders::ALL))
    .wrap(Wrap { trim: false });
    f.render_widget(preset_text, settings_chunks[2]);

    // Help bar
    let help = Paragraph::new(
        "  ↑↓ navigate   Space/Enter select/toggle   Tab next section   Enter start (on Types)   Q quit"
    )
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_testing(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),   // score/stats bar
            Constraint::Min(8),      // sequence display
            Constraint::Length(3),   // input box
            Constraint::Length(2),   // time gauge
            Constraint::Length(2),   // help
        ])
        .split(area);

    // Score bar
    let elapsed = app.elapsed_secs();
    let score_text = format!(
        "  Score: {}   Correct: {}/{}   Time: {}s",
        app.score, app.correct, app.total, elapsed
    );
    let score_color = if app.score > 0 { Color::Green } else { Color::White };
    let score = Paragraph::new(score_text)
        .style(Style::default().fg(score_color).add_modifier(Modifier::BOLD));
    f.render_widget(score, chunks[0]);

    // Sequence display
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
        .block(Block::default().borders(Borders::ALL).title("  Next term?"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(seq_para, chunks[1]);

    // Input box
    let input_display = format!("  > {}_", app.input);
    let input_para = Paragraph::new(input_display)
        .block(Block::default().borders(Borders::ALL).title("  Your answer"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(input_para, chunks[2]);

    // Time gauge
    let ratio = app.remaining_ratio();
    let label = format!(
        "{:.0}s remaining",
        app.config.time_limit.seconds().saturating_sub(app.elapsed_secs())
    );
    let gauge_color = if ratio > 0.5 { Color::Green } else if ratio > 0.25 { Color::Yellow } else { Color::Red };
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(ratio)
        .label(label);
    f.render_widget(gauge, chunks[3]);

    // Help
    let help = Paragraph::new("  Enter submit   Esc skip   Q quit")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[4]);
}

fn draw_results(f: &mut Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(4),   // title + score
            Constraint::Min(8),      // breakdown
            Constraint::Length(2),   // actions
        ])
        .split(area);

    // Header
    let accuracy = if app.total > 0 {
        app.correct as f64 / app.total as f64 * 100.0
    } else { 0.0 };
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Results", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(format!("Score: {}   Correct: {}/{}   Accuracy: {:.0}%",
                app.score, app.correct, app.total, accuracy),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Breakdown by type
    let items: Vec<ListItem> = app.per_kind.iter()
        .filter(|(_, _, t)| *t > 0)
        .map(|(kind, c, t)| {
            let pct = *c as f64 / *t as f64 * 100.0;
            let color = if pct >= 80.0 { Color::Green } else if pct >= 50.0 { Color::Yellow } else { Color::Red };
            ListItem::new(
                Line::from(vec![
                    Span::raw(format!("  {:<16}", kind.label())),
                    Span::styled(format!("{}/{}", c, t), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                    Span::raw(format!("  ({:.0}%)", pct)),
                ])
            )
        })
        .collect();

    let breakdown = List::new(items)
        .block(Block::default().title("  Breakdown by type").borders(Borders::ALL));
    f.render_widget(breakdown, chunks[1]);

    // Actions
    let actions = Paragraph::new("  [R] Restart   [Q] Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(actions, chunks[2]);
}
