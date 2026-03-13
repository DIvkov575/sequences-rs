use rand::Rng;
use super::{Sequence, SequenceKind};
use crate::config::Difficulty;

pub struct ArithmeticSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl ArithmeticSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let (start_range, step_range) = match difficulty {
            Difficulty::Easy => (1..=20i64, 1..=10i64),
            Difficulty::Medium => (1..=100i64, 1..=20i64),
            Difficulty::Hard => (1..=500i64, 1..=50i64),
        };
        let start = rng.random_range(start_range);
        let step = rng.random_range(step_range) * if rng.random_bool(0.5) { 1 } else { -1 };
        let mut terms: Vec<i64> = (0..=n_visible as i64)
            .map(|i| start + i * step)
            .collect();
        let answer = terms.pop().unwrap();
        Self { terms, answer }
    }
}

impl Sequence for ArithmeticSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Arithmetic }
}
