# Advent of Code - Rust Solutions
# Run `just` or `just --list` to see available commands

# =======================
# CONFIGURATION
# =======================
year := `ls -d crates/aoc* 2>/dev/null | sed 's|crates/aoc||' | sort -r | head -n 1 || echo ""`
pkg := "aoc" + year
current_day := `ls crates/aoc{{year}}/src/bin/d*.rs 2>/dev/null | sort -r | head -n 1 | sed 's/.*d\([0-9]*\)\.rs/\1/' || echo ""`

# Default action
default: test lint

# =======================
# BUILD
# =======================

# Build all days in workspace
build:
    @echo "Building {{pkg}}..."
    cargo build -p {{pkg}}

# Build specific day
build-day day:
    @echo "Building {{pkg}} day {{day}}..."
    cargo build -p {{pkg}} --bin d{{day}}

# Build all in release mode
release:
    @echo "Building {{pkg}} in release mode..."
    cargo build -p {{pkg}} --release

# Build specific day in release
release-day day:
    @echo "Building {{pkg}} day {{day}} in release mode..."
    cargo build -p {{pkg}} --release --bin d{{day}}

# =======================
# TEST
# =======================

# Run all tests
test:
    @echo "Running all tests..."
    cargo test -p {{pkg}}

# Test specific day
test-day day:
    @echo "Testing {{pkg}} day {{day}}..."
    cargo test -p {{pkg}} --bin d{{day}}

# =======================
# QUALITY CHECKS
# =======================

# Run both clippy and format check
lint: clippy fmt-check

# Run clippy linter
clippy:
    @echo "Running clippy..."
    cargo clippy -p {{pkg}} --all-targets --all-features -- -D warnings

# Format all code
fmt:
    @echo "Formatting code..."
    cargo fmt --all

# Check if code is formatted
fmt-check:
    @echo "Checking formatting..."
    cargo fmt --all -- --check

# Run cargo check
check:
    @echo "Running cargo check..."
    cargo check -p {{pkg}}

# =======================
# CLEAN
# =======================

# Clean build artifacts
clean:
    @echo "Cleaning workspace..."
    cargo clean

# =======================
# BENCHMARKING
# =======================

# Run benchmarks for all days
bench:
    @echo "Running benchmarks..."
    cargo bench -p {{pkg}}

# Benchmark specific day
bench-day day:
    @echo "Benchmarking {{pkg}} day {{day}}..."
    cargo bench -p {{pkg}} --bin d{{day}}

# =======================
# RUNNING SOLUTIONS
# =======================

# Run day in debug mode with input file
run day input:
    @echo "Running {{pkg}} day {{day}} (debug) with {{input}}..."
    @cat {{input}} | cargo run -p {{pkg}} --bin d{{day}}

# Run day in release mode with input file or keyword
run-release day input:
    #!/usr/bin/env bash
    set -euo pipefail
    DAY=$(printf "%02d" {{day}})
    INPUT="{{input}}"

    # Handle special input keywords
    if [ "$INPUT" = "puzzle" ] || [ "$INPUT" = "puzzle_input" ]; then
        if [ -f "inputs/{{year}}/d$DAY.txt" ]; then
            INPUT="inputs/{{year}}/d$DAY.txt"
        elif [ -f "inputs/{{year}}/input.txt" ]; then
            INPUT="inputs/{{year}}/input.txt"
        else
            echo "No puzzle input found in inputs/{{year}}/"
            exit 1
        fi
    fi

    if [ ! -f "$INPUT" ]; then
        echo "Input file not found: $INPUT"
        exit 1
    fi

    echo "Running {{pkg}} day {{day}} (release) with $INPUT..."
    cargo build -p {{pkg}} --release --bin d$DAY
    cat "$INPUT" | target/release/d$DAY

# Run current/latest day
run-current input:
    @if [ -z "{{current_day}}" ]; then echo "No days found!"; exit 1; fi
    @echo "Running day {{current_day}}..."
    @just run-release {{current_day}} {{input}}

# =======================
# DAY CREATION
# =======================

# Create new day from template
new-day day:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ -z "{{year}}" ]; then
        echo "No year found. Run: just init-year 2025"
        exit 1
    fi

    DAY=$(printf "%02d" {{day}})
    DAY_FILE="crates/{{pkg}}/src/bin/d$DAY.rs"

    if [ -f "$DAY_FILE" ]; then
        echo "Error: $DAY_FILE already exists!"
        exit 1
    fi

    echo "Creating $DAY_FILE..."

    # Create from template if it exists
    if [ -f "templates/day_template.rs" ]; then
        cp templates/day_template.rs "$DAY_FILE"
    else
        # Create minimal template
        cat > "$DAY_FILE" << 'EOF'
    use aoc{{year}}::*;

    fn part1(input: &str) -> u64 {
        0
    }

    fn part2(input: &str) -> u64 {
        0
    }

    fn main() {
        let input = read_input();

        let start = std::time::Instant::now();
        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
        println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE: &str = "";

        #[test]
        fn test_part1() {
            assert_eq!(part1(EXAMPLE), 0);
        }

        #[test]
        fn test_part2() {
            assert_eq!(part2(EXAMPLE), 0);
        }
    }
    EOF
        # Fix year placeholder
        sed -i "s/aoc{{year}}/{{pkg}}/g" "$DAY_FILE"
    fi

    echo "✓ Created $DAY_FILE"
    echo "  Edit: $DAY_FILE"
    echo "  Run:  just run-release {{day}} puzzle"

# Initialize new year structure
init-year year:
    #!/usr/bin/env bash
    set -euo pipefail

    PKG="aoc{{year}}"
    echo "Initializing year {{year}} ($PKG)..."

    # Create directory structure
    mkdir -p "crates/$PKG/src/bin"
    mkdir -p "inputs/{{year}}"
    mkdir -p "answers/{{year}}"

    # Create Cargo.toml for the year
    cat > "crates/$PKG/Cargo.toml" << EOF
    [package]
    name = "$PKG"
    version = "0.1.0"
    edition = "2024"

    [dependencies]
    # Add common dependencies here
    # itertools = "0.14"
    # regex = "1"
    # rayon = "1"
    EOF

    # Create lib.rs with utilities
    cat > "crates/$PKG/src/lib.rs" << 'EOF'
    use std::io::{self, Read};

    /// Read input from stdin
    pub fn read_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    /// Parse input as lines
    pub fn lines(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    /// Parse input as numbers (one per line)
    pub fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        input.lines()
            .map(|line| line.parse().expect("Failed to parse number"))
            .collect()
    }
    EOF

    # Add to workspace
    if ! grep -q "\"crates/$PKG\"" Cargo.toml; then
        sed -i "/members = \[/a\\    \"crates/$PKG\"," Cargo.toml
        echo "Added $PKG to workspace"
    fi

    echo "✓ Created crates/$PKG, inputs/{{year}}, answers/{{year}}"
    echo "  Next: just new-day 1"

# =======================
# AOC INTEGRATION
# =======================

# Download puzzle input
download day:
    @bash scripts/download_input.sh {{year}} {{day}}

# Check completion status
check-status day:
    @bash scripts/check_status.sh {{year}} {{day}}

# Submit answer for specific part
submit day part:
    @bash scripts/submit.sh {{year}} {{day}} {{part}}

# Run and interactively submit
run-submit day input:
    @bash scripts/run_submit.sh {{year}} {{day}} {{input}}

# =======================
# DEVELOPMENT HELPERS
# =======================

# Watch and auto-rebuild (requires cargo-watch)
watch day:
    @echo "Watching {{pkg}} day {{day}}..."
    cargo watch -x 'check -p {{pkg}} --bin d{{day}}' -x 'test -p {{pkg}} --bin d{{day}}'

# Quick solve workflow: download, run, submit
solve day:
    @echo "Quick solve for day {{day}}..."
    @just download {{day}}
    @just run-submit {{day}} puzzle

# Open day's problem in browser
open day:
    @xdg-open "https://adventofcode.com/{{year}}/day/{{day}}" 2>/dev/null || \
     open "https://adventofcode.com/{{year}}/day/{{day}}" 2>/dev/null || \
     echo "Visit: https://adventofcode.com/{{year}}/day/{{day}}"

# =======================
# UTILITY
# =======================

# Show statistics about solutions
stats:
    #!/usr/bin/env bash
    echo "=== Advent of Code {{year}} Stats ==="
    echo "Days completed: $(ls -1 crates/{{pkg}}/src/bin/d*.rs 2>/dev/null | wc -l)"
    echo "Total lines: $(find crates/{{pkg}}/src -name '*.rs' -exec cat {} \; | wc -l)"
    echo "Test coverage: $(cargo test -p {{pkg}} 2>&1 | grep -o '[0-9]* passed' || echo 'N/A')"

# Create all necessary directories
setup:
    @bash setup.sh {{year}}

# =======================
# ALIASES
# =======================

alias b := build
alias t := test
alias r := run-release
alias l := lint
alias c := check
