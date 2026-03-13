pub mod arithmetic;
pub mod geometric;
pub mod quadratic;
pub mod fibonacci;
pub mod special;

use rand::Rng;
use crate::config::Difficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SequenceKind {
    Arithmetic,
    Geometric,
    Quadratic,
    Fibonacci,
    Tribonacci,
    Triangular,
    Squares,
    Cubes,
    Alternating,
}

impl SequenceKind {
    pub const ALL: &'static [SequenceKind] = &[
        SequenceKind::Arithmetic,
        SequenceKind::Geometric,
        SequenceKind::Quadratic,
        SequenceKind::Fibonacci,
        SequenceKind::Tribonacci,
        SequenceKind::Triangular,
        SequenceKind::Squares,
        SequenceKind::Cubes,
        SequenceKind::Alternating,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SequenceKind::Arithmetic => "Arithmetic",
            SequenceKind::Geometric => "Geometric",
            SequenceKind::Quadratic => "Quadratic",
            SequenceKind::Fibonacci => "Fibonacci",
            SequenceKind::Tribonacci => "Tribonacci",
            SequenceKind::Triangular => "Triangular",
            SequenceKind::Squares => "Squares (n²)",
            SequenceKind::Cubes => "Cubes (n³)",
            SequenceKind::Alternating => "Alternating",
        }
    }
}

pub trait Sequence {
    fn visible_terms(&self) -> &[i64];
    fn answer(&self) -> i64;
    #[allow(dead_code)]
    fn kind(&self) -> SequenceKind;
}

pub fn generate(kind: SequenceKind, difficulty: Difficulty, n: usize, rng: &mut impl Rng) -> Box<dyn Sequence> {
    match kind {
        SequenceKind::Arithmetic  => Box::new(arithmetic::ArithmeticSeq::generate(difficulty, n, rng)),
        SequenceKind::Geometric   => Box::new(geometric::GeometricSeq::generate(difficulty, n, rng)),
        SequenceKind::Quadratic   => Box::new(quadratic::QuadraticSeq::generate(difficulty, n, rng)),
        SequenceKind::Fibonacci   => Box::new(fibonacci::FibonacciSeq::generate(difficulty, n, rng)),
        SequenceKind::Tribonacci  => Box::new(fibonacci::TribonacciSeq::generate(difficulty, n, rng)),
        SequenceKind::Triangular  => Box::new(special::TriangularSeq::generate(difficulty, n, rng)),
        SequenceKind::Squares     => Box::new(special::SquaresSeq::generate(difficulty, n, rng)),
        SequenceKind::Cubes       => Box::new(special::CubesSeq::generate(difficulty, n, rng)),
        SequenceKind::Alternating => Box::new(special::AlternatingSeq::generate(difficulty, n, rng)),
    }
}
