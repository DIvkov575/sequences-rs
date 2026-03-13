use crate::sequences::SequenceKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub const ALL: &'static [Difficulty] = &[Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn n_visible(&self) -> usize {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Medium => 4,
            Difficulty::Hard => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeLimit {
    Sixty,
    TwoMinutes,
    FiveMinutes,
}

impl TimeLimit {
    pub const ALL: &'static [TimeLimit] = &[TimeLimit::Sixty, TimeLimit::TwoMinutes, TimeLimit::FiveMinutes];

    pub fn label(&self) -> &'static str {
        match self {
            TimeLimit::Sixty => "60s",
            TimeLimit::TwoMinutes => "120s",
            TimeLimit::FiveMinutes => "300s",
        }
    }

    pub fn seconds(&self) -> u64 {
        match self {
            TimeLimit::Sixty => 60,
            TimeLimit::TwoMinutes => 120,
            TimeLimit::FiveMinutes => 300,
        }
    }
}

pub const OPTIVER_PRESET: &[SequenceKind] = &[
    SequenceKind::Arithmetic,
    SequenceKind::Geometric,
    SequenceKind::Quadratic,
    SequenceKind::Fibonacci,
    SequenceKind::Tribonacci,
    SequenceKind::Alternating,
];

pub const FLOW_TRADERS_PRESET: &[SequenceKind] = &[
    SequenceKind::Arithmetic,
    SequenceKind::Geometric,
    SequenceKind::Fibonacci,
    SequenceKind::Squares,
    SequenceKind::Cubes,
    SequenceKind::Alternating,
];

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub enabled: Vec<SequenceKind>,
    pub difficulty: Difficulty,
    pub time_limit: TimeLimit,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            enabled: SequenceKind::ALL.to_vec(),
            difficulty: Difficulty::Medium,
            time_limit: TimeLimit::Sixty,
        }
    }
}
