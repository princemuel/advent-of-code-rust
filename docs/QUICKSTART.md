# Advent of Code - Quick Reference

## 🚀 Initial Setup

```bash
# 1. Create project
mkdir advent-of-code && cd advent-of-code

# 2. Save all files (justfile, setup.sh, scripts/*)
# Make them executable
chmod +x setup.sh scripts/*.sh

# 3. Run setup
./setup.sh 2025

# 4. Add your session token to .env
echo "TOKEN=your_token_here" > .env

# 5. Install just (if not already)
yay -S just
```

## 📁 Project Structure

```console
advent-of-code/
├── justfile                    # Build commands
├── Cargo.toml                  # Workspace config
├── .env                        # Session token (gitignored)
├── scripts/                    # Helper scripts
│   ├── download_input.sh
│   ├── check_status.sh
│   ├── submit.sh
│   └── run_submit.sh
├── crates/
│   └── aoc2025/
│       ├── Cargo.toml         # Package dependencies
│       └── src/
│           ├── lib.rs         # Shared utilities
│           └── bin/
│               ├── d01.rs     # Day 1 solution
│               ├── d02.rs     # Day 2 solution
│               └── ...
├── inputs/2025/               # Puzzle inputs (gitignored)
│   ├── d01.txt
│   └── ...
└── answers/2025/              # Generated answers (gitignored)
    ├── submit_d01.txt
    └── ...
```

## 🎯 Daily Workflow

### Quick Solve (One Command)

```bash
just solve 5          # Downloads input, runs solution, prompts for submission
```

### Step-by-Step

```bash
# 1. Create new day
just new-day 5

# 2. Edit solution
$EDITOR crates/aoc2025/src/bin/d05.rs

# 3. Download input
just download 5

# 4. Test with example
echo "example input" | cargo run -p aoc2025 --bin d05

# 5. Run with real input
just run-release 5 puzzle

# 6. Submit answer
just submit 5 1       # Submit part 1
just submit 5 2       # Submit part 2
```

## 📝 Solution Template

Each day starts with this template:

```rust
use aoc2025::*;

fn part1(input: &str) -> u64 {
    // Your solution here
    0
}

fn part2(input: &str) -> u64 {
    // Your solution here
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

    const EXAMPLE: &str = ""; // Add example input

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }
}
```

## 🛠️ Common Commands

### Building

```bash
just build              # Build all days (debug)
just build-day 5        # Build specific day (debug)
just release            # Build all days (optimized)
just release-day 5      # Build specific day (optimized)
```

### Testing

```bash
just test               # Run all tests
just test-day 5         # Test specific day
cargo test -p aoc2025 --bin d05  # Run tests for d05.rs
```

### Running

```bash
just run 5 inputs/2025/d05.txt    # Run with custom input (debug)
just run-release 5 puzzle         # Run with puzzle input (optimized)
just run-current puzzle           # Run most recent day
```

### Development

```bash
just watch 5            # Auto-rebuild on file changes
just open 5             # Open problem in browser
just check-status 5     # Check completion status
just stats              # Show project statistics
```

### Quality

```bash
just lint               # Run clippy + format check
just fmt                # Format code
just check              # Cargo check
```

### Aliases

```bash
just b                  # Build
just t                  # Test
just r 5 puzzle         # Run release
just l                  # Lint
just c                  # Check
```

## 📚 Shared Utilities

The `aoc2025` crate provides utilities in `src/lib.rs`:

```rust
use aoc2025::*;

// Read input from stdin
let input = read_input();

// Parse as lines
let lines = lines(input);

// Parse as numbers
let numbers: Vec<i32> = parse_numbers(input);

// Grid utilities
use aoc2025::grid::*;
let neighbors = neighbors4((0, 0));  // 4-directional
let all = neighbors8((0, 0));        // 8-directional
```

## 🎨 Adding Dependencies

Edit `crates/aoc2025/Cargo.toml`:

```toml
[dependencies]
itertools = "0.14"
regex = "1"
rayon = "1"
```

All days will have access to these dependencies.

## 🔧 Troubleshooting

### "No year found"

```bash
# Initialize a new year
just init-year 2025
```

### "TOKEN not found"

```bash
# Add your session token
echo "TOKEN=your_token_here" > .env
```

### "Input file not found"

```bash
# Download the input first
just download 5

# Or specify full path
just run-release 5 inputs/2025/d05.txt
```

### Slow compilation

```bash
# Use debug builds for development
cargo run -p aoc2025 --bin d05

# Only use release for final runs
just run-release 5 puzzle
```

## 🌟 Pro Tips

1. **Use `just solve N`** for the full workflow
2. **Add common parsing to `lib.rs`** to share across days
3. **Run tests frequently**: `just watch 5` auto-tests on save
4. **Use debug builds** while developing (faster compile)
5. **Check status** before submitting: `just check-status 5`
6. **Read hints** - scripts show if answer is too high/low

## 📖 Example Session

```bash
# Morning routine
just solve 5

# Outputs:
# ----------------------------------
# Running Year 2025 Day 5
# ----------------------------------
# Building aoc2025 day 05...
# Running solution...
#
# Part 1: 12345
# Part 2: 67890
# Elapsed time: 0.0234 seconds
#
# ----------------------------------
# Year 2025 Day 5 - Status
# ----------------------------------
# Part 1: ○ Incomplete
# Part 2: ○ Incomplete
# ----------------------------------
#
# Submit Part 1 answer (12345)? (y/N) y
# ✓ Correct answer!
# → Part 2 unlocked!
#
# Waiting 45 seconds before Part 2 submission...
# Submit Part 2 answer (67890)? (y/N) y
# ✓ Correct answer!
# 🌟 Day 5 complete!
```

## 🎯 Getting Session Token

1. Go to <https://adventofcode.com>
2. Log in
3. Open Developer Tools (F12)
4. Go to Storage/Application → Cookies
5. Find cookie named `session`
6. Copy its value
7. Add to `.env`: `TOKEN=<value>`
