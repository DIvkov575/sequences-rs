# sequences-rs

A command-line tool for training number sequence recognition, written in Rust.

Presents timed drills where you identify the next term in a sequence. Designed for practice with the kinds of sequence questions that appear in quantitative finance interviews (Optiver, Flow Traders, etc.).

# Install

Install on macOS with Homebrew:
```
brew tap divkov575/sequences-rs https://github.com/DIvkov575/homebrew-sequences-rs
brew install sequences-rs
```

Install using cargo:
```
cargo install sequences-rs
```

Install from source:
```
git clone https://github.com/DIvkov575/sequences-rs
cd sequences-rs
cargo install --path .
```

# Execution

```
sequences
```
or
```
sequences-rs
```

# Sequence Types

- **Arithmetic** — constant difference
- **Geometric** — constant ratio
- **Quadratic** — second-order differences
- **Fibonacci / Tribonacci** — each term is sum of previous 2 or 3
- **Triangular / Squares / Cubes** — n(n+1)/2, n², n³
- **Alternating** — two interleaved arithmetic sequences

# Configuration

- **Difficulty** — controls the range of numbers generated (Easy / Medium / Hard)
- **Sequence Length** — number of visible terms (5 / 6 / 7)
- **Time Limit** — 1 / 5 / 10 / 15 minutes
- **Presets** — `O` for Optiver, `F` for Flow Traders

# Releasing

```
./release.sh <version>   # e.g. ./release.sh 0.3.0
```

Tags the repo, computes sha256, and pushes the updated formula to the Homebrew tap.
