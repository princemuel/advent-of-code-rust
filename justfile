# Advent of Code
# ==============================================================================
set shell := ["bash", "-cu"]
set dotenv-load := false

# -------------------------
# CONFIG
# -------------------------

# Detect latest available AoC year
year := `ls -d crates/aoc* 2>/dev/null | sed 's|crates/aoc||' | sort -r | head -n 1 || echo ""`
pkg  := "aoc" + year

# Detect latest day for a specific year
current_day := `
    ls crates/{{pkg}}/src/bin/d*.rs 2>/dev/null |
    sort -r |
    head -n 1 |
    sed 's/.*d\([0-9]*\)\.rs/\1/' ||
    echo ""
`

default: test lint

# -------------------------
# BUILD
# -------------------------

build:
    cargo build -p {{pkg}}

build-day day:
    cargo build -p {{pkg}} --bin d{{day}}

release:
    cargo build -p {{pkg}} --release

release-day day:
    cargo build -p {{pkg}} --release --bin d{{day}}

# -------------------------
# TEST
# -------------------------

test:
    cargo test -p {{pkg}}

test-day day:
    cargo test -p {{pkg}} --bin d{{day}}

# -------------------------
# LINT/FORMAT
# -------------------------

lint: clippy fmt-check

clippy:
    cargo clippy -p {{pkg}} --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

check:
    cargo check -p {{pkg}}

# -------------------------
# CLEAN
# -------------------------

clean:
    cargo clean

# -------------------------
# BENCH
# -------------------------

bench:
    cargo bench -p {{pkg}}

bench-day day:
    cargo bench -p {{pkg}} --bin d{{day}}

# -------------------------
# RUNNERS
# -------------------------

run day input:
    @scripts/run_debug.sh {{pkg}} {{day}} {{input}}

run-release day input:
    @scripts/run_release.sh {{year}} {{pkg}} {{day}} {{input}}

run-current input:
    @scripts/run_current.sh {{year}} {{pkg}} {{current_day}} {{input}}

# -------------------------
# DAY CREATION
# -------------------------

new-day day:
    @scripts/new_day.sh {{year}} {{pkg}} {{day}}

init-year year:
    @scripts/init_year.sh {{year}}

# -------------------------
# AOC INTEGRATION
# -------------------------

download day:
    @scripts/download_input.sh {{year}} {{day}}

check-status day:
    @scripts/check_status.sh {{year}} {{day}}

submit day part:
    @scripts/submit.sh {{year}} {{day}} {{part}}

run-submit day input:
    @scripts/run_submit.sh {{year}} {{day}} {{input}}

# -------------------------
# DEV HELPERS
# -------------------------

watch day:
    cargo watch -x "check -p {{pkg}} --bin d{{day}}" \
                -x "test -p {{pkg}} --bin d{{day}}"

solve day:
    @scripts/solve.sh {{year}} {{pkg}} {{day}}

open day:
    @scripts/open_day.sh {{year}} {{day}}

browse year day:
    sh xdg-open "https://adventofcode.com/{{year}}/day/{{day}}" \
      || open "https://adventofcode.com/{{year}}/day/{{day}}" \
      || echo "Visit: https://adventofcode.com/{{year}}/day/{{day}}"

# -------------------------
# STATS + SETUP
# -------------------------

stats:
    @scripts/stats.sh {{year}} {{pkg}}

setup:
    @scripts/setup.sh {{year}}

# -------------------------
# ALIASES
# -------------------------

alias b := build
alias t := test
alias r := run-release
alias l := lint
alias c := check
