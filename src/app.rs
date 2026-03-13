use rand::{SeedableRng, rngs::SmallRng, Rng};
use std::time::Instant;
use crate::config::TestConfig;
use crate::sequences::{self, SequenceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Configuration,
    Testing,
    Results,
}

pub struct Question {
    pub visible_terms: Vec<i64>,
    pub answer: i64,
    pub kind: SequenceKind,
}

pub struct App {
    pub state: AppState,
    pub config: TestConfig,

    // --- config screen cursor ---
    pub cursor: usize,
    pub config_section: ConfigSection,

    // --- testing ---
    pub current: Option<Question>,
    pub input: String,
    pub score: u32,
    pub total: u32,
    pub correct: u32,
    pub start_time: Option<Instant>,
    pub per_kind: Vec<(SequenceKind, u32, u32)>, // (kind, correct, total)

    rng: SmallRng,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSection {
    Types,
    Difficulty,
    TimeLimit,
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
    }

    pub fn start_testing(&mut self) {
        self.state = AppState::Testing;
        self.start_time = Some(Instant::now());
        self.next_question();
    }

    pub fn next_question(&mut self) {
        if self.config.enabled.is_empty() { return; }
        let idx = self.rng.random_range(0..self.config.enabled.len());
        let kind = self.config.enabled[idx];
        let seq = sequences::generate(kind, self.config.difficulty, &mut self.rng);
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
            let entry = self.per_kind.iter_mut().find(|(k, _, _)| *k == kind);
            if let Some((_, c, t)) = entry { *t += 1; if user == Some(correct_ans) { *c += 1; } }
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
            if let Some((_, _, t)) = self.per_kind.iter_mut().find(|(k,_,_)| *k == kind) { *t += 1; }
        }
        self.next_question();
    }

    pub fn is_time_up(&self) -> bool {
        match self.start_time {
            Some(t) => t.elapsed().as_secs() >= self.config.time_limit.seconds(),
            None => false,
        }
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
        match self.config_section {
            ConfigSection::Types => {
                if self.cursor > 0 { self.cursor -= 1; }
            }
            ConfigSection::Difficulty | ConfigSection::TimeLimit => {}
        }
    }

    pub fn config_down(&mut self) {
        match self.config_section {
            ConfigSection::Types => {
                if self.cursor + 1 < SequenceKind::ALL.len() { self.cursor += 1; }
            }
            ConfigSection::Difficulty | ConfigSection::TimeLimit => {}
        }
    }

    pub fn config_next_section(&mut self) {
        self.config_section = match self.config_section {
            ConfigSection::Types => ConfigSection::Difficulty,
            ConfigSection::Difficulty => ConfigSection::TimeLimit,
            ConfigSection::TimeLimit => ConfigSection::Types,
        };
        self.cursor = 0;
    }

    pub fn config_prev_section(&mut self) {
        self.config_section = match self.config_section {
            ConfigSection::Types => ConfigSection::TimeLimit,
            ConfigSection::Difficulty => ConfigSection::Types,
            ConfigSection::TimeLimit => ConfigSection::Difficulty,
        };
        self.cursor = 0;
    }

    pub fn config_toggle_or_select(&mut self) {
        match self.config_section {
            ConfigSection::Types => {
                let kind = SequenceKind::ALL[self.cursor];
                if self.config.enabled.contains(&kind) {
                    self.config.enabled.retain(|&k| k != kind);
                } else {
                    self.config.enabled.push(kind);
                }
            }
            ConfigSection::Difficulty => {
                use crate::config::Difficulty;
                self.config.difficulty = Difficulty::ALL[self.cursor];
            }
            ConfigSection::TimeLimit => {
                use crate::config::TimeLimit;
                self.config.time_limit = TimeLimit::ALL[self.cursor];
            }
        }
    }

    pub fn apply_optiver_preset(&mut self) {
        use crate::config::OPTIVER_PRESET;
        self.config.enabled = OPTIVER_PRESET.to_vec();
    }

    pub fn apply_flow_traders_preset(&mut self) {
        use crate::config::FLOW_TRADERS_PRESET;
        self.config.enabled = FLOW_TRADERS_PRESET.to_vec();
    }
}
