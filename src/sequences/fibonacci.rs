use rand::Rng;
use super::{Sequence, SequenceKind};
use crate::config::Difficulty;

pub struct FibonacciSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl FibonacciSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let (seed_range, _) = match difficulty {
            Difficulty::Easy => (1..=5i64, ()),
            Difficulty::Medium => (1..=20i64, ()),
            Difficulty::Hard => (1..=50i64, ()),
        };
        let a = rng.random_range(seed_range.clone());
        let b = rng.random_range(seed_range);
        let mut all = vec![a, b];
        while all.len() <= n_visible {
            let next = all[all.len() - 1] + all[all.len() - 2];
            all.push(next);
        }
        let answer = all.pop().unwrap();
        Self { terms: all, answer }
    }
}

impl Sequence for FibonacciSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Fibonacci }
}

pub struct TribonacciSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl TribonacciSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let seed_range = match difficulty {
            Difficulty::Easy => 1..=3i64,
            Difficulty::Medium => 1..=10i64,
            Difficulty::Hard => 1..=20i64,
        };
        let a = rng.random_range(seed_range.clone());
        let b = rng.random_range(seed_range.clone());
        let c = rng.random_range(seed_range);
        let mut all = vec![a, b, c];
        while all.len() <= n_visible {
            let n = all.len();
            all.push(all[n-1] + all[n-2] + all[n-3]);
        }
        let answer = all.pop().unwrap();
        Self { terms: all, answer }
    }
}

impl Sequence for TribonacciSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Tribonacci }
}
