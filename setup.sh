#!/usr/bin/env bash
# Setup script for Advent of Code Rust project
# Usage: ./setup.sh [YEAR]

set -euo pipefail

YEAR="${1:-2025}"
PKG="aoc$YEAR"

echo "----------------------------------"
echo "Advent of Code $YEAR - Setup"
echo "----------------------------------"
echo ""

# Check if just is installed
if ! command -v just &> /dev/null; then
    echo "⚠ 'just' is not installed"
    echo "Install with: yay -S just"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    [[ ! $REPLY =~ ^[Yy]$ ]] && exit 0
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "✗ Rust/Cargo not found"
    echo "Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust/Cargo found: $(cargo --version)"
[ -x "$(command -v just)" ] && echo "✓ Just found: $(just --version)"
echo ""

# Create directory structure
echo "Creating directory structure..."
mkdir -p "crates/$PKG/src/bin"
mkdir -p "inputs/$YEAR"
mkdir -p "answers/$YEAR"
mkdir -p "scripts"
mkdir -p "templates"

echo "✓ Created directories"
echo ""

# Create .env template if it doesn't exist
if [ ! -f .env ]; then
    echo "Creating .env template..."
    cat > .env << 'EOF'
# Your Advent of Code session token
# Get it from: https://adventofcode.com (browser cookies after login)
TOKEN=
EOF
    echo "✓ Created .env template"
    echo "  → Add your session token to .env"
else
    echo "✓ .env already exists"
fi
echo ""

# Create .gitignore if it doesn't exist
if [ ! -f .gitignore ]; then
    echo "Creating .gitignore..."
    cat > .gitignore << 'EOF'
# Rust
target/
Cargo.lock
**/*.rs.bk
*.pdb

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# Advent of Code
.env
inputs/
answers/
EOF
    echo "✓ Created .gitignore"
else
    echo "✓ .gitignore already exists"
fi
echo ""

# Create workspace Cargo.toml if it doesn't exist
if [ ! -f Cargo.toml ]; then
    echo "Creating workspace Cargo.toml..."
    cat > Cargo.toml << EOF
[workspace]
resolver = "3"
members = ["crates/*"]

[workspace.package]
edition = "2024"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]

[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"

[profile.bench]
opt-level = 3
debug = false
lto = true
codegen-units = 1
EOF
    echo "✓ Created workspace Cargo.toml"
else
    echo "✓ Cargo.toml already exists"
fi
echo ""

# Create year crate Cargo.toml
echo "Creating $PKG crate..."
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
echo "✓ Created crates/$PKG/Cargo.toml"
echo ""

# Create lib.rs with utilities
echo "Creating utility library..."
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

/// Grid utilities
pub mod grid {
    pub type Point = (i32, i32);

    pub fn neighbors4((x, y): Point) -> [Point; 4] {
        [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]
    }

    pub fn neighbors8((x, y): Point) -> [Point; 8] {
        [
            (x+1, y), (x-1, y), (x, y+1), (x, y-1),
            (x+1, y+1), (x+1, y-1), (x-1, y+1), (x-1, y-1)
        ]
    }
}
EOF
echo "✓ Created crates/$PKG/src/lib.rs"
echo ""

# Create day template
echo "Creating day template..."
cat > templates/day_template.rs << EOF
use $PKG::*;

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
echo "✓ Created templates/day_template.rs"
echo ""

# Make scripts executable
if [ -d scripts ]; then
    echo "Setting script permissions..."
    chmod +x scripts/*.sh 2>/dev/null || true
    echo "✓ Scripts are executable"
    echo ""
fi

# Create README
if [ ! -f README.md ]; then
    echo "Creating README.md..."
    cat > README.md << EOF
# Advent of Code $YEAR - Rust Solutions

Solutions for [Advent of Code $YEAR](https://adventofcode.com/$YEAR) implemented in Rust.

## Project Structure

\`\`\`
crates/$PKG/
├── Cargo.toml
└── src/
    ├── lib.rs          # Shared utilities
    └── bin/
        ├── d01.rs      # Day 1 solution
        ├── d02.rs      # Day 2 solution
        └── ...
\`\`\`

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [just](https://github.com/casey/just) - \`yay -S just\`
- Optional: [cargo-watch](https://github.com/watchexec/cargo-watch) - \`cargo install cargo-watch\`

## Quick Start

1. Add your session token to \`.env\`:
   \`\`\`bash
   TOKEN=your_session_token_here
   \`\`\`

2. Create a new day:
   \`\`\`bash
   just new-day 1
   \`\`\`

3. Download input and solve:
   \`\`\`bash
   just solve 1
   \`\`\`

## Available Commands

Run \`just\` or \`just --list\` to see all available commands.

### Build & Test
- \`just build\` - Build all days
- \`just test\` - Run all tests
- \`just release\` - Build optimized binaries

### Run Solutions
- \`just run-release 1 puzzle\` - Run day 1 with puzzle input
- \`just run-current puzzle\` - Run most recent day

### Advent of Code Integration
- \`just download 1\` - Download day 1 input
- \`just solve 1\` - Download, run, and submit day 1
- \`just check-status 1\` - Check completion status

### Development
- \`just watch 1\` - Auto-rebuild on changes
- \`just lint\` - Run clippy and format check
- \`just fmt\` - Format all code

## Shared Utilities

The \`$PKG\` crate provides common utilities in \`lib.rs\`:

\`\`\`rust
use $PKG::*;

fn solve(input: &str) {
    let lines = lines(input);           // Parse as lines
    let nums = parse_numbers(input);    // Parse as numbers
    // ... grid utilities available in grid module
}
\`\`\`

## License

MIT
EOF
    echo "✓ Created README.md"
else
    echo "✓ README.md already exists"
fi
echo ""

echo "----------------------------------"
echo "✓ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Add your session token to .env"
echo "  2. Run: just new-day 1"
echo "  3. Run: just solve 1"
echo ""
echo "Run 'just' to see all available commands"
echo "----------------------------------"
