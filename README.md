# Advent of Code

This repository contains solutions for [Advent of Code](https://adventofcode.com), written in Rust and organized per year.
A custom CLI tool, **`aocctl`**, automates day creation, template selection, input downloading, running, and submissions.

> [!IMPORTANT]
> The code in this repo may be _sometimes_ (or most times 😉) verbose. That's just me finding out how far I can push my boundaries with Rust. I do try to implement the algorithms in a simpler form first, but they are in a private repo, so you're just gonna take my word for it.

## Project Structure

```console
crates/aoc[year]/
├── Cargo.toml
└── src/
    ├── lib.rs          # Shared utilities
    └── bin/
        ├── d01.rs      # Day 1 solution
        ├── d02.rs      # Day 2 solution
        └── ...
```

Additional generated folders:

```console
inputs/[year]/    # Downloaded puzzle inputs
answers/[year]/   # Submission records
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [just](https://github.com/casey/just) - `cargo install just`
- Optional: [cargo-watch](https://github.com/watchexec/cargo-watch) - `cargo install cargo-watch`
- Advent of Code session cookie stored in any of these:
  - .env SESSION in .env
  - env var SESSION on the command line
  - .session file in the working dir
  - ~/.config/aoc/.session

## Quick Start

### 1. Initialize the year

```bash
aocctl init 2025
```

### 2. Create a new day

```bash
just new 1
# or with a specific template:
aocctl new 1 --template fast
```

### 3. Download input & solve

```bash
just solve 1
```

## Available Commands

### Build & Test

- `just build` — Build workspace
- `just test` — Run all tests
- `just release` — Build optimized binaries

### Run Solutions

- `just run-day 1 puzzle` — Run day 1 with puzzle input
- `just run` — Run latest day
- `just solve 1` — Download, run, and optionally submit

### Advent of Code Integration

- `just input 1` — Download input
- `just solve 1` — Execute and submit
- `just open 1` — Open problem in browser
- `just templates` — List templates

### Development

- `just watch 1` — Auto rebuild
- `just lint` — Clippy + fmt checks
- `just fmt` — Format code

## Template System

Templates define the structure of generated day solutions.

| Template  | Description                              |
| --------- | ---------------------------------------- |
| minimal   | Simple, small, stdin-based               |
| buffered  | Efficient file/stdin input via BufReader |
| streaming | Iterator-based, no full allocation       |
| fast      | Zero-copy &str parsing                   |

List available templates:

```bash
aocctl list-templates
```

Example:

```bash
aocctl new 5 --template streaming
```

## Daily Workflow

```bash
just new 7
just input 7
just run-day 7 puzzle
just solve 7
```

Dry-run submission:

```bash
aocctl solve 7 --dry-run
```

## Shared Utilities

```rust
use aoc2025::*;

fn solve(input: &str) {
    let lines = lines(input);
    let nums = parse_numbers(input);
    // grid helpers, coordinate math, iter utils, etc.
}
```

## Troubleshooting

### Missing session token

```bash
echo "<your_cookie>" > .session
```

### Input not downloaded

```bash
just input <day>
```

### File doesn’t exist

```bash
aocctl new <day>
```

### Performance issues

Use streaming or fast template:

```bash
aocctl new <day> --template fast
```

## Contributing

PRs improving templates, utilities, workflow, or documentation are welcome.

## License

MIT
