#!/usr/bin/env bash
# Run solution and interactively submit answers
# Usage: ./run_submit.sh YEAR DAY INPUT

set -euo pipefail

YEAR="${1:-}"
DAY="${2:-}"
INPUT="${3:-}"

if [ -z "$YEAR" ] || [ -z "$DAY" ] || [ -z "$INPUT" ]; then
    echo "Usage: $0 YEAR DAY INPUT"
    exit 1
fi

DAY=$(printf "%02d" "$DAY")
DAY_NUM=$((10#$DAY))
PKG="aoc$YEAR"

# Handle special input keywords
if [ "$INPUT" = "download" ]; then
    bash scripts/download_input.sh "$YEAR" "$DAY"
    INPUT="inputs/$YEAR/d$DAY.txt"
elif [ "$INPUT" = "puzzle" ] || [ "$INPUT" = "puzzle_input" ]; then
    if [ -f "inputs/$YEAR/d$DAY.txt" ]; then
        INPUT="inputs/$YEAR/d$DAY.txt"
    elif [ -f "inputs/$YEAR/input.txt" ]; then
        INPUT="inputs/$YEAR/input.txt"
    else
        echo "Error: No puzzle input found in inputs/$YEAR/"
        echo "Run: just download $DAY_NUM"
        exit 1
    fi
fi

# Verify input file exists
if [ ! -f "$INPUT" ]; then
    echo "Error: Input file not found: $INPUT"
    exit 1
fi

# Create answers directory
mkdir -p "answers/$YEAR"
ANSWER_FILE="answers/$YEAR/submit_d$DAY.txt"

echo "----------------------------------"
echo "Running Year $YEAR Day $DAY"
echo "----------------------------------"

# Build in release mode
echo "Building $PKG day $DAY..."
cargo build -p "$PKG" --release --bin d$DAY 2>&1 | grep -v "Compiling\|Finished" || true

# Run solution and capture output
echo "Running solution..."
echo ""

if ! OUTPUT=$(cat "$INPUT" | timeout 60s target/release/d$DAY 2>&1); then
    echo "✗ Solution failed or timed out"
    exit 1
fi

echo "$OUTPUT"
echo ""
echo "----------------------------------"

# Extract answers (case-insensitive, flexible spacing)
PART1=$(echo "$OUTPUT" | grep -i "part 1:" | sed 's/.*part 1:\s*//i' | tr -d ' ' | head -n1)
PART2=$(echo "$OUTPUT" | grep -i "part 2:" | sed 's/.*part 2:\s*//i' | tr -d ' ' | head -n1)

# Save answers
> "$ANSWER_FILE"
[ -n "$PART1" ] && echo "Part1: $PART1" >> "$ANSWER_FILE"
[ -n "$PART2" ] && echo "Part2: $PART2" >> "$ANSWER_FILE"

if [ -z "$PART1" ] && [ -z "$PART2" ]; then
    echo "⚠ Could not extract answers from output"
    echo "Make sure your solution prints:"
    echo "  Part 1: <answer>"
    echo "  Part 2: <answer>"
    exit 1
fi

echo "Answers saved to $ANSWER_FILE"

# Check current status
echo ""
STATUS_OUTPUT=$(bash scripts/check_status.sh "$YEAR" "$DAY")
echo "$STATUS_OUTPUT"
echo ""

# Parse status
PART1_DONE=$(echo "$STATUS_OUTPUT" | grep "Part 1: ✓" || true)
PART2_DONE=$(echo "$STATUS_OUTPUT" | grep "Part 2: ✓" || true)

# Interactive submission
SUBMITTED_PART1=0
SUBMITTED_PART2=0

# Part 1 submission
if [ -n "$PART1" ] && [ -z "$PART1_DONE" ]; then
    read -p "Submit Part 1 answer ($PART1)? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        bash scripts/submit.sh "$YEAR" "$DAY" 1
        SUBMITTED_PART1=1

        # Check if part 1 was accepted
        if grep -q "Part1:.*\[Status: Correct\]" "$ANSWER_FILE"; then
            # If part 2 answer exists, offer to submit it after cooldown
            if [ -n "$PART2" ]; then
                echo ""
                echo "Part 1 accepted! Waiting 45 seconds before Part 2 submission..."
                echo "(This avoids rate limiting)"

                # Countdown
                for i in {45..1}; do
                    printf "\rWaiting... %2ds" $i
                    sleep 1
                done
                printf "\r"
                echo "Ready!"
                echo ""

                read -p "Submit Part 2 answer ($PART2)? (y/N) " -n 1 -r
                echo
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    bash scripts/submit.sh "$YEAR" "$DAY" 2
                    SUBMITTED_PART2=1
                fi
            fi
        fi
    fi
fi

# Part 2 submission (if part 1 wasn't just submitted)
if [ -n "$PART2" ] && [ -z "$PART2_DONE" ] && [ -n "$PART1_DONE" ] && [ $SUBMITTED_PART1 -eq 0 ]; then
    read -p "Submit Part 2 answer ($PART2)? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        bash scripts/submit.sh "$YEAR" "$DAY" 2
        SUBMITTED_PART2=1
    fi
fi

# Final summary
echo ""
echo "----------------------------------"
if [ -n "$PART1" ]; then
    STATUS1=$(grep "Part1:" "$ANSWER_FILE" | grep -o "\[Status: .*\]" || echo "[Not submitted]")
    echo "Part 1: $PART1 $STATUS1"
fi
if [ -n "$PART2" ]; then
    STATUS2=$(grep "Part2:" "$ANSWER_FILE" | grep -o "\[Status: .*\]" || echo "[Not submitted]")
    echo "Part 2: $PART2 $STATUS2"
fi
echo "----------------------------------"
