mod app;
mod config;
mod sequences;
mod ui;

use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use app::{App, AppState, ConfigSection};
use config::{Difficulty, TimeLimit};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        // Auto-transition when time is up
        if app.state == AppState::Testing && app.is_time_up() {
            app.state = AppState::Results;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match app.state {
                    AppState::Configuration => handle_config(app, key.code),
                    AppState::Testing => {
                        if handle_testing(app, key.code) { break; }
                    }
                    AppState::Results => {
                        if handle_results(app, key.code) { break; }
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_config(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => std::process::exit(0),
        KeyCode::Char('o') | KeyCode::Char('O') => app.apply_optiver_preset(),
        KeyCode::Char('f') | KeyCode::Char('F') => app.apply_flow_traders_preset(),

        KeyCode::Up => {
            match app.config_section {
                ConfigSection::Types => app.config_up(),
                ConfigSection::Difficulty => {
                    let cur = Difficulty::ALL.iter().position(|d| *d == app.config.difficulty).unwrap_or(0);
                    if cur > 0 { app.config.difficulty = Difficulty::ALL[cur - 1]; }
                }
                ConfigSection::TimeLimit => {
                    let cur = TimeLimit::ALL.iter().position(|t| *t == app.config.time_limit).unwrap_or(0);
                    if cur > 0 { app.config.time_limit = TimeLimit::ALL[cur - 1]; }
                }
            }
        }
        KeyCode::Down => {
            match app.config_section {
                ConfigSection::Types => app.config_down(),
                ConfigSection::Difficulty => {
                    let cur = Difficulty::ALL.iter().position(|d| *d == app.config.difficulty).unwrap_or(0);
                    if cur + 1 < Difficulty::ALL.len() { app.config.difficulty = Difficulty::ALL[cur + 1]; }
                }
                ConfigSection::TimeLimit => {
                    let cur = TimeLimit::ALL.iter().position(|t| *t == app.config.time_limit).unwrap_or(0);
                    if cur + 1 < TimeLimit::ALL.len() { app.config.time_limit = TimeLimit::ALL[cur + 1]; }
                }
            }
        }

        KeyCode::Tab => app.config_next_section(),
        KeyCode::BackTab => app.config_prev_section(),

        KeyCode::Char(' ') => {
            if app.config_section == ConfigSection::Types {
                app.config_toggle_or_select();
            }
        }

        KeyCode::Enter => {
            match app.config_section {
                ConfigSection::Types => app.config_toggle_or_select(),
                ConfigSection::Difficulty | ConfigSection::TimeLimit => {}
            }
            if app.config_section == ConfigSection::TimeLimit && !app.config.enabled.is_empty() {
                app.start_testing();
            }
        }

        KeyCode::Char('s') | KeyCode::Char('S') => {
            if !app.config.enabled.is_empty() {
                app.start_testing();
            }
        }

        _ => {}
    }
}

fn handle_testing(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => { app.state = AppState::Results; }
        KeyCode::Esc => app.skip_question(),
        KeyCode::Enter => app.submit_answer(),
        KeyCode::Backspace => { app.input.pop(); }
        KeyCode::Char('-') if app.input.is_empty() => app.input.push('-'),
        KeyCode::Char(c) if c.is_ascii_digit() => app.input.push(c),
        _ => {}
    }
    false
}

fn handle_results(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => return true,
        KeyCode::Char('r') | KeyCode::Char('R') => app.reset(),
        _ => {}
    }
    false
}
