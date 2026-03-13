use rand::Rng;
use super::{Sequence, SequenceKind};
use crate::config::Difficulty;

// --- Triangular ---
pub struct TriangularSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl TriangularSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let start_n = match difficulty {
            Difficulty::Easy => rng.random_range(1..=4i64),
            Difficulty::Medium => rng.random_range(1..=8i64),
            Difficulty::Hard => rng.random_range(1..=15i64),
        };
        let mut terms: Vec<i64> = (start_n..start_n + n_visible as i64 + 1)
            .map(|n| n * (n + 1) / 2)
            .collect();
        let answer = terms.pop().unwrap();
        Self { terms, answer }
    }
}

impl Sequence for TriangularSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Triangular }
}

// --- Squares ---
pub struct SquaresSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl SquaresSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let start_n = match difficulty {
            Difficulty::Easy => rng.random_range(1..=4i64),
            Difficulty::Medium => rng.random_range(1..=8i64),
            Difficulty::Hard => rng.random_range(1..=12i64),
        };
        let mut terms: Vec<i64> = (start_n..start_n + n_visible as i64 + 1)
            .map(|n| n * n)
            .collect();
        let answer = terms.pop().unwrap();
        Self { terms, answer }
    }
}

impl Sequence for SquaresSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Squares }
}

// --- Cubes ---
pub struct CubesSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl CubesSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let start_n = match difficulty {
            Difficulty::Easy => rng.random_range(1..=3i64),
            Difficulty::Medium => rng.random_range(1..=5i64),
            Difficulty::Hard => rng.random_range(1..=8i64),
        };
        let mut terms: Vec<i64> = (start_n..start_n + n_visible as i64 + 1)
            .map(|n| n * n * n)
            .collect();
        let answer = terms.pop().unwrap();
        Self { terms, answer }
    }
}

impl Sequence for CubesSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Cubes }
}

// --- Alternating (two interleaved arithmetic sequences) ---
pub struct AlternatingSeq {
    terms: Vec<i64>,
    answer: i64,
}

impl AlternatingSeq {
    pub fn generate(difficulty: Difficulty, n_visible: usize, rng: &mut impl Rng) -> Self {
        let (range_a, range_b) = match difficulty {
            Difficulty::Easy => (1..=15i64, 10..=50i64),
            Difficulty::Medium => (1..=30i64, 10..=100i64),
            Difficulty::Hard => (1..=100i64, 10..=200i64),
        };
        let a_start = rng.random_range(range_a.clone());
        let a_step = rng.random_range(1..=5i64);
        let b_start = rng.random_range(range_b.clone());
        let b_step = rng.random_range(5..=20i64);

        // generate enough interleaved terms
        let needed = n_visible + 1;
        let mut terms = Vec::with_capacity(needed);
        let (mut ai, mut bi) = (0i64, 0i64);
        for i in 0..needed {
            if i % 2 == 0 {
                terms.push(a_start + ai * a_step);
                ai += 1;
            } else {
                terms.push(b_start + bi * b_step);
                bi += 1;
            }
        }
        let answer = terms.pop().unwrap();
        Self { terms, answer }
    }
}

impl Sequence for AlternatingSeq {
    fn visible_terms(&self) -> &[i64] { &self.terms }
    fn answer(&self) -> i64 { self.answer }
    fn kind(&self) -> SequenceKind { SequenceKind::Alternating }
}
