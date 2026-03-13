use rand::Rng;
use super::{Sequence, SequenceKind};
use crate::config::Difficulty;

pub struct QuadraticSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl QuadraticSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        // Generate sequence: a*n^2 + b*n + c  (n = 1, 2, 3, ...)
        let (a_range, b_range, c_range) = match difficulty {
            Difficulty::Easy => (1..=2i64, 0..=3i64, 0..=10i64),
            Difficulty::Medium => (1..=4i64, 0..=5i64, 0..=20i64),
            Difficulty::Hard => (1..=6i64, 0..=10i64, 0..=50i64),
        };
        let a = rng.random_range(a_range);
        let b = rng.random_range(b_range);
        let c = rng.random_range(c_range);
        let mut terms: Vec<i64> = (1..=n_visible as i64 + 1)
            .map(|n| a * n * n + b * n + c)
            .collect();
        let answer = terms.pop().unwrap();
        Self { terms, answer }
    }
}

impl Sequence for QuadraticSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Quadratic }
}
