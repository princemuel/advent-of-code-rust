#!/usr/bin/env bash
# Check completion status for a specific day
# Usage: ./check_status.sh YEAR DAY

set -euo pipefail

YEAR="${1:-}"
DAY="${2:-}"

if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
    echo "Usage: $0 YEAR DAY"
    exit 1
fi

DAY=$(printf "%02d" "$DAY")
DAY_NUM=$((10#$DAY))

# Load session token
if [ ! -f .env ]; then
    echo "Error: .env file not found"
    exit 1
fi

set -a
source .env
set +a

if [ -z "${TOKEN:-}" ]; then
    echo "Error: TOKEN not found in .env"
    exit 1
fi

# Fetch puzzle page
RESPONSE=$(curl -s \
    --cookie "session=$TOKEN" \
    -H "User-Agent: github.com/advent-of-code-rust" \
    "https://adventofcode.com/$YEAR/day/$DAY_NUM")

echo "----------------------------------"
echo "Year $YEAR Day $DAY_NUM - Status"
echo "----------------------------------"

# Check completion status with various patterns
PART1_COMPLETE=0
PART2_COMPLETE=0

if echo "$RESPONSE" | grep -q "Both parts of this puzzle are complete"; then
    PART1_COMPLETE=1
    PART2_COMPLETE=1
elif echo "$RESPONSE" | grep -q "The first half of this puzzle is complete"; then
    PART1_COMPLETE=1
elif echo "$RESPONSE" | grep -q "one gold star"; then
    PART1_COMPLETE=1
fi

# Additional check for part 2
if echo "$RESPONSE" | grep -q "You have completed Day $DAY_NUM"; then
    PART2_COMPLETE=1
fi

# Display status
if [ $PART1_COMPLETE -eq 1 ]; then
    echo "Part 1: ✓ Complete"
else
    echo "Part 1: ○ Incomplete"
fi

if [ $PART2_COMPLETE -eq 1 ]; then
    echo "Part 2: ✓ Complete"
else
    echo "Part 2: ○ Incomplete"
fi

echo "----------------------------------"

# Update answer file status if it exists
ANSWER_FILE="answers/$YEAR/submit_d$DAY.txt"
if [ -f "$ANSWER_FILE" ]; then
    if [ $PART1_COMPLETE -eq 1 ]; then
        # Mark part 1 as correct if not already marked
        sed -i 's/^Part1: \(.*\)\(\[Status:.*\]\)\?$/Part1: \1 [Status: Correct]/' "$ANSWER_FILE" 2>/dev/null || true
    fi
    if [ $PART2_COMPLETE -eq 1 ]; then
        # Mark part 2 as correct if not already marked
        sed -i 's/^Part2: \(.*\)\(\[Status:.*\]\)\?$/Part2: \1 [Status: Correct]/' "$ANSWER_FILE" 2>/dev/null || true
    fi
fi
