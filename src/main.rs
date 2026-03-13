mod app;
mod config;
mod history;
mod sequences;
mod ui;

use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use app::{App, AppState, ConfigSection, ResultsButton};

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

        if app.state == AppState::Testing && app.is_time_up() {
            app.finish();
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match app.state {
                    AppState::Configuration => handle_config(app, key.code),
                    AppState::Testing       => handle_testing(app, key.code),
                    AppState::Results       => { if handle_results(app, key.code) { break; } }
                    AppState::History       => handle_history(app, key.code),
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

        KeyCode::Tab     => app.config_next_section(),
        KeyCode::BackTab => app.config_prev_section(),

        KeyCode::Up   => app.config_up(),
        KeyCode::Down => app.config_down(),

        KeyCode::Char(' ') => app.config_toggle(),

        KeyCode::Enter => {
            match app.config_section {
                ConfigSection::Types => app.config_toggle(),
                ConfigSection::Difficulty | ConfigSection::Length | ConfigSection::TimeLimit => {}
                ConfigSection::Start => {
                    if !app.config.enabled.is_empty() { app.start_testing(); }
                }
            }
        }

        KeyCode::Char('s') | KeyCode::Char('S') => {
            if !app.config.enabled.is_empty() { app.start_testing(); }
        }

        _ => {}
    }
}

fn handle_testing(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.finish(),
        KeyCode::Esc => app.skip_question(),
        KeyCode::Enter => app.submit_answer(),
        KeyCode::Backspace => { app.input.pop(); }
        KeyCode::Char('-') if app.input.is_empty() => app.input.push('-'),
        KeyCode::Char(c) if c.is_ascii_digit() => app.input.push(c),
        _ => {}
    }
}

fn handle_results(app: &mut App, code: KeyCode) -> bool {
    match code {
        // keyboard shortcuts
        KeyCode::Char('r') | KeyCode::Char('R') => app.reset(),
        KeyCode::Char('h') | KeyCode::Char('H') => app.enter_history(),
        KeyCode::Char('q') | KeyCode::Char('Q') => return true,

        // tab through buttons
        KeyCode::Tab | KeyCode::Right => app.results_button = app.results_button.next(),
        KeyCode::BackTab | KeyCode::Left => app.results_button = app.results_button.prev(),

        // activate focused button
        KeyCode::Enter | KeyCode::Char(' ') => match app.results_button {
            ResultsButton::Restart => app.reset(),
            ResultsButton::History => app.enter_history(),
            ResultsButton::Quit    => return true,
        },

        _ => {}
    }
    false
}

fn handle_history(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up | KeyCode::Char('k') => app.history_up(),
        KeyCode::Down | KeyCode::Char('j') => app.history_down(),
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.state = AppState::Results,
        _ => {}
    }
}
