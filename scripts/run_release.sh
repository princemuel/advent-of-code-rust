#!/usr/bin/env bash
set -euo pipefail

YEAR=$1
PKG=$2
DAY_RAW=$3
INPUT=$4

DAY=$(printf "%02d" "$DAY_RAW")

# Resolve puzzle input keyword
if [[ "$INPUT" = "puzzle" || "$INPUT" = "puzzle_input" ]]; then
    if [ -f "inputs/$YEAR/d$DAY.txt" ]; then
        INPUT="inputs/$YEAR/d$DAY.txt"
    elif [ -f "inputs/$YEAR/input.txt" ]; then
        INPUT="inputs/$YEAR/input.txt"
    else
        echo "No puzzle input found for $YEAR."
        exit 1
    fi
fi

if [[ ! -f "$INPUT" ]]; then
    echo "Input file does not exist: $INPUT"
    exit 1
fi

echo "Running $PKG day $DAY (release) with $INPUT..."
cargo build -p "$PKG" --release --bin d"$DAY"
cat "$INPUT" | target/release/d"$DAY"
