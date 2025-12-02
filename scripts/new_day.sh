#!/usr/bin/env bash
set -euo pipefail

_YEAR=$1
PKG=$2
DAY_RAW=$3

DAY=$(printf "%02d" "$DAY_RAW")
FILE="crates/$PKG/src/bin/d$DAY.rs"

if [[ -f "$FILE" ]]; then
    echo "Day $DAY already exists: $FILE"
    exit 1
fi

echo "Creating $FILE..."

if [[ -f "templates/day_template.rs" ]]; then
    cp templates/day_template.rs "$FILE"
else
    cat >"$FILE" <<EOF
use $PKG::*;

fn part1(input: &str) -> u64 { 0 }
fn part2(input: &str) -> u64 { 0 }

fn main() {
    let input = input();
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.4} seconds", start.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "";

    #[test] fn test_part1() { assert_eq!(part1(EXAMPLE), 0); }
    #[test] fn test_part2() { assert_eq!(part2(EXAMPLE), 0); }
}
EOF
fi

echo "✓ Created $FILE"
