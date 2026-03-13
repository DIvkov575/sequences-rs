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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeLimit {
    One,
    Five,
    Ten,
    Fifteen,
}

impl TimeLimit {
    pub const ALL: &'static [TimeLimit] = &[
        TimeLimit::One,
        TimeLimit::Five,
        TimeLimit::Ten,
        TimeLimit::Fifteen,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            TimeLimit::One     => "1 min",
            TimeLimit::Five    => "5 min",
            TimeLimit::Ten     => "10 min",
            TimeLimit::Fifteen => "15 min",
        }
    }

    pub fn seconds(&self) -> u64 {
        match self {
            TimeLimit::One     => 60,
            TimeLimit::Five    => 300,
            TimeLimit::Ten     => 600,
            TimeLimit::Fifteen => 900,
        }
    }
}

pub const SEQ_LEN_OPTIONS: &[usize] = &[5, 6, 7];

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
    pub seq_len: usize,
    pub time_limit: TimeLimit,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            enabled: SequenceKind::ALL.to_vec(),
            difficulty: Difficulty::Medium,
            seq_len: 6,
            time_limit: TimeLimit::Five,
        }
    }
}
