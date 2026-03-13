use rand::Rng;
use super::{Sequence, SequenceKind};
use crate::config::Difficulty;

pub struct GeometricSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl GeometricSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let (start_range, ratio_range) = match difficulty {
            Difficulty::Easy => (1..=10i64, 2..=3i64),
            Difficulty::Medium => (1..=20i64, 2..=4i64),
            Difficulty::Hard => (1..=50i64, 2..=5i64),
        };
        loop {
            let start = rng.random_range(start_range.clone());
            let ratio = rng.random_range(ratio_range.clone());
            let mut terms: Vec<i64> = (0..=n_visible as u32)
                .map(|i| start * ratio.pow(i))
                .collect();
            // guard against overflow
            if terms.iter().all(|&t| t.abs() < 1_000_000) {
                let answer = terms.pop().unwrap();
                return Self { terms, answer };
            }
        }
    }
}

impl Sequence for GeometricSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Geometric }
}
