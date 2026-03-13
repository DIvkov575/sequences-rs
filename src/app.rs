use rand::{SeedableRng, rngs::SmallRng, Rng};
use std::time::Instant;
use crate::config::TestConfig;
use crate::history::{self, HistoryEntry};
use crate::sequences::{self, SequenceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Configuration,
    Testing,
    Results,
    History,
}

pub struct Question {
    pub visible_terms: Vec<i64>,
    pub answer: i64,
    pub kind: SequenceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSection {
    Types,
    Difficulty,
    Length,
    TimeLimit,
    Start,
    Stats,
}

impl ConfigSection {
    fn next(self) -> Self {
        match self {
            Self::Types      => Self::Difficulty,
            Self::Difficulty => Self::Length,
            Self::Length     => Self::TimeLimit,
            Self::TimeLimit  => Self::Start,
            Self::Start      => Self::Stats,
            Self::Stats      => Self::Types,
        }
    }
    fn prev(self) -> Self {
        match self {
            Self::Types      => Self::Stats,
            Self::Difficulty => Self::Types,
            Self::Length     => Self::Difficulty,
            Self::TimeLimit  => Self::Length,
            Self::Start      => Self::TimeLimit,
            Self::Stats      => Self::Start,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultsButton { Restart, History, Quit }

impl ResultsButton {
    #[allow(dead_code)]
    pub const ALL: &'static [ResultsButton] = &[
        ResultsButton::Restart,
        ResultsButton::History,
        ResultsButton::Quit,
    ];
    pub fn next(self) -> Self {
        match self {
            Self::Restart => Self::History,
            Self::History => Self::Quit,
            Self::Quit    => Self::Restart,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Restart => Self::Quit,
            Self::History => Self::Restart,
            Self::Quit    => Self::History,
        }
    }
}

pub struct App {
    pub state: AppState,
    pub config: TestConfig,

    // config screen
    pub cursor: usize,
    pub config_section: ConfigSection,

    // testing
    pub current: Option<Question>,
    pub input: String,
    pub score: u32,
    pub total: u32,
    pub correct: u32,
    pub start_time: Option<Instant>,
    pub per_kind: Vec<(SequenceKind, u32, u32)>,

    // results
    pub results_button: ResultsButton,
    pub session_duration: u64,

    // history screen
    pub history: Vec<HistoryEntry>,
    pub history_scroll: usize,
    pub history_return: AppState,

    rng: SmallRng,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Configuration,
            config: TestConfig::default(),
            cursor: 0,
            config_section: ConfigSection::Types,
            current: None,
            input: String::new(),
            score: 0,
            total: 0,
            correct: 0,
            start_time: None,
            per_kind: SequenceKind::ALL.iter().map(|&k| (k, 0, 0)).collect(),
            results_button: ResultsButton::Restart,
            session_duration: 0,
            history: history::load(),
            history_scroll: 0,
            history_return: AppState::Results,
            rng: SmallRng::from_os_rng(),
        }
    }

    pub fn reset(&mut self) {
        self.state = AppState::Configuration;
        self.cursor = 0;
        self.config_section = ConfigSection::Types;
        self.current = None;
        self.input.clear();
        self.score = 0;
        self.total = 0;
        self.correct = 0;
        self.start_time = None;
        self.per_kind = SequenceKind::ALL.iter().map(|&k| (k, 0, 0)).collect();
        self.results_button = ResultsButton::Restart;
        self.session_duration = 0;
    }

    pub fn start_testing(&mut self) {
        self.state = AppState::Testing;
        self.start_time = Some(Instant::now());
        self.next_question();
    }

    pub fn finish(&mut self) {
        self.session_duration = self.elapsed_secs();
        let entry = HistoryEntry {
            timestamp: history::now_secs(),
            score: self.score,
            correct: self.correct,
            total: self.total,
            difficulty: self.config.difficulty.label().to_string(),
            time_limit_secs: self.config.time_limit.seconds(),
            duration_secs: self.session_duration,
        };
        let _ = history::append(&entry);
        self.history.insert(0, entry);
        self.state = AppState::Results;
    }

    pub fn next_question(&mut self) {
        if self.config.enabled.is_empty() { return; }
        let idx = self.rng.random_range(0..self.config.enabled.len());
        let kind = self.config.enabled[idx];
        let seq = sequences::generate(kind, self.config.difficulty, self.config.seq_len, &mut self.rng);
        self.current = Some(Question {
            visible_terms: seq.visible_terms().to_vec(),
            answer: seq.answer(),
            kind,
        });
        self.input.clear();
    }

    pub fn submit_answer(&mut self) {
        if let Some(ref q) = self.current {
            let kind = q.kind;
            let correct_ans = q.answer;
            let user: Option<i64> = self.input.parse().ok();
            self.total += 1;
            if let Some((_, c, t)) = self.per_kind.iter_mut().find(|(k, _, _)| *k == kind) {
                *t += 1;
                if user == Some(correct_ans) { *c += 1; }
            }
            if user == Some(correct_ans) {
                self.score += 1;
                self.correct += 1;
            }
        }
        self.next_question();
    }

    pub fn skip_question(&mut self) {
        if let Some(ref q) = self.current {
            let kind = q.kind;
            self.total += 1;
            if let Some((_, _, t)) = self.per_kind.iter_mut().find(|(k, _, _)| *k == kind) { *t += 1; }
        }
        self.next_question();
    }

    pub fn is_time_up(&self) -> bool {
        self.start_time.map_or(false, |t| t.elapsed().as_secs() >= self.config.time_limit.seconds())
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0)
    }

    pub fn remaining_ratio(&self) -> f64 {
        let total = self.config.time_limit.seconds() as f64;
        let elapsed = self.elapsed_secs() as f64;
        ((total - elapsed) / total).max(0.0)
    }

    // --- config navigation ---

    pub fn config_up(&mut self) {
        use crate::config::{Difficulty, TimeLimit, SEQ_LEN_OPTIONS};
        match self.config_section {
            ConfigSection::Types => { if self.cursor > 0 { self.cursor -= 1; } }
            ConfigSection::Difficulty => {
                let cur = Difficulty::ALL.iter().position(|d| *d == self.config.difficulty).unwrap_or(0);
                if cur > 0 { self.config.difficulty = Difficulty::ALL[cur - 1]; }
            }
            ConfigSection::Length => {
                let cur = SEQ_LEN_OPTIONS.iter().position(|&n| n == self.config.seq_len).unwrap_or(0);
                if cur > 0 { self.config.seq_len = SEQ_LEN_OPTIONS[cur - 1]; }
            }
            ConfigSection::TimeLimit => {
                let cur = TimeLimit::ALL.iter().position(|t| *t == self.config.time_limit).unwrap_or(0);
                if cur > 0 { self.config.time_limit = TimeLimit::ALL[cur - 1]; }
            }
            ConfigSection::Start | ConfigSection::Stats => {}
        }
    }

    pub fn config_down(&mut self) {
        use crate::config::{Difficulty, TimeLimit, SEQ_LEN_OPTIONS};
        match self.config_section {
            ConfigSection::Types => {
                if self.cursor + 1 < SequenceKind::ALL.len() { self.cursor += 1; }
            }
            ConfigSection::Difficulty => {
                let cur = Difficulty::ALL.iter().position(|d| *d == self.config.difficulty).unwrap_or(0);
                if cur + 1 < Difficulty::ALL.len() { self.config.difficulty = Difficulty::ALL[cur + 1]; }
            }
            ConfigSection::Length => {
                let cur = SEQ_LEN_OPTIONS.iter().position(|&n| n == self.config.seq_len).unwrap_or(0);
                if cur + 1 < SEQ_LEN_OPTIONS.len() { self.config.seq_len = SEQ_LEN_OPTIONS[cur + 1]; }
            }
            ConfigSection::TimeLimit => {
                let cur = TimeLimit::ALL.iter().position(|t| *t == self.config.time_limit).unwrap_or(0);
                if cur + 1 < TimeLimit::ALL.len() { self.config.time_limit = TimeLimit::ALL[cur + 1]; }
            }
            ConfigSection::Start | ConfigSection::Stats => {}
        }
    }

    pub fn config_next_section(&mut self) {
        self.config_section = self.config_section.next();
        self.cursor = 0;
    }

    pub fn config_prev_section(&mut self) {
        self.config_section = self.config_section.prev();
        self.cursor = 0;
    }

    pub fn config_toggle(&mut self) {
        if self.config_section == ConfigSection::Types {
            let kind = SequenceKind::ALL[self.cursor];
            if self.config.enabled.contains(&kind) {
                self.config.enabled.retain(|&k| k != kind);
            } else {
                self.config.enabled.push(kind);
            }
        }
    }

    pub fn apply_optiver_preset(&mut self) {
        self.config.enabled = crate::config::OPTIVER_PRESET.to_vec();
    }

    pub fn apply_flow_traders_preset(&mut self) {
        self.config.enabled = crate::config::FLOW_TRADERS_PRESET.to_vec();
    }

    pub fn enter_history(&mut self) {
        self.history_return = self.state;
        self.state = AppState::History;
        self.history_scroll = 0;
    }

    // --- history navigation ---

    pub fn history_up(&mut self) {
        if self.history_scroll > 0 { self.history_scroll -= 1; }
    }

    pub fn history_down(&mut self) {
        if self.history_scroll + 1 < self.history.len() { self.history_scroll += 1; }
    }
}
