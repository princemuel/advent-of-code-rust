# Advent of Code Solutions

Solutions for [Advent of Code](https://adventofcode.com) implemented in Rust, and organized by year.

## Project Structure

```console
crates/aoc2025/
├── Cargo.toml
└── src/
    ├── lib.rs          # Shared utilities
    └── bin/
        ├── d01.rs      # Day 1 solution
        ├── d02.rs      # Day 2 solution
        └── ...
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [just](https://github.com/casey/just) - `cargo install just`
- Optional: [cargo-watch](https://github.com/watchexec/cargo-watch) - `cargo install cargo-watch`

## Quick Start

1. Add your session token to `.env`:

   ```bash
   TOKEN=your_session_token_here
   ```

2. Create a new day:

   ```bash
   just new-day 1
   ```

3. Download input and solve:

   ```bash
   just solve 1
   ```

## Available Commands

Run `just` or `just --list` to see all available commands.

### Build & Test

- `just build` - Build all days
- `just test` - Run all tests
- `just release` - Build optimized binaries

### Run Solutions

- `just run-release 1 puzzle` - Run day 1 with puzzle input
- `just run-current puzzle` - Run most recent day

### Advent of Code Integration

- `just download 1` - Download day 1 input
- `just solve 1` - Download, run, and submit day 1
- `just check-status 1` - Check completion status

### Development

- `just watch 1` - Auto-rebuild on changes
- `just lint` - Run clippy and format check
- `just fmt` - Format all code

## Shared Utilities

The `aoc2025` crate provides common utilities in `lib.rs`:

```rust
use aoc2025::*;

fn solve(input: &str) {
    let lines = lines(input);           // Parse as lines
    let nums = parse_numbers(input);    // Parse as numbers
    // ... grid utilities available in grid module
}
```

## License

MIT
