#!/usr/bin/env bash
# Submit an answer to Advent of Code
# Usage: ./submit.sh YEAR DAY PART

set -euo pipefail

YEAR="${1:-}"
DAY="${2:-}"
PART="${3:-}"

if [ -z "$YEAR" ] || [ -z "$DAY" ] || [ -z "$PART" ]; then
    echo "Usage: $0 YEAR DAY PART"
    exit 1
fi

DAY=$(printf "%02d" "$DAY")
DAY_NUM=$((10#$DAY))

ANSWER_FILE="answers/$YEAR/submit_d$DAY.txt"

# Check if answer file exists
if [ ! -f "$ANSWER_FILE" ]; then
    echo "Error: No answer file found at $ANSWER_FILE"
    echo "Run the solution first to generate answers."
    exit 1
fi

# Extract answer for the specified part
ANSWER=$(grep "^Part$PART:" "$ANSWER_FILE" | cut -d':' -f2 | sed 's/\[Status:.*\]//g' | tr -d ' ' | head -n1)

if [ -z "$ANSWER" ]; then
    echo "Error: No answer found for Part $PART in $ANSWER_FILE"
    exit 1
fi

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

# Check if part 2 requires part 1 completion
if [ "$PART" = "2" ]; then
    if ! grep -q "Part1:.*\[Status: Correct\]" "$ANSWER_FILE"; then
        echo "⚠ Warning: Part 1 not marked as correct"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        [[ ! $REPLY =~ ^[Yy]$ ]] && exit 0
    fi
fi

echo "----------------------------------"
echo "Submitting Year $YEAR Day $DAY_NUM Part $PART"
echo "Answer: $ANSWER"
echo "----------------------------------"

# Submit answer
RESPONSE=$(curl -s -X POST \
    --cookie "session=$TOKEN" \
    -H "User-Agent: github.com/advent-of-code-rust" \
    -d "level=$PART&answer=$ANSWER" \
    "https://adventofcode.com/$YEAR/day/$DAY_NUM/answer")

# Parse response and provide feedback
if echo "$RESPONSE" | grep -q "That's the right answer"; then
    echo "✓ Correct answer!"
    sed -i "s/^Part$PART: $ANSWER.*$/Part$PART: $ANSWER [Status: Correct]/" "$ANSWER_FILE"

    # Check for completion messages
    if [ "$PART" = "1" ] && echo "$RESPONSE" | grep -q "You've finished every puzzle"; then
        echo "🎉 You've completed all available puzzles!"
    elif [ "$PART" = "1" ]; then
        echo "→ Part 2 unlocked!"
    elif [ "$PART" = "2" ]; then
        echo "🌟 Day $DAY_NUM complete!"
    fi

elif echo "$RESPONSE" | grep -q "You gave an answer too recently"; then
    if echo "$RESPONSE" | grep -q "You have [0-9]*m [0-9]*s"; then
        WAIT_TIME=$(echo "$RESPONSE" | grep -o "You have [0-9]*m [0-9]*s" | head -1)
        echo "⏳ Rate limited: $WAIT_TIME"
    else
        echo "⏳ Rate limited: Please wait before submitting again"
    fi

elif echo "$RESPONSE" | grep -q "That's not the right answer"; then
    echo "✗ Incorrect answer"

    # Provide hints
    if echo "$RESPONSE" | grep -q "too high"; then
        echo "   Hint: Answer is too high"
    elif echo "$RESPONSE" | grep -q "too low"; then
        echo "   Hint: Answer is too low"
    fi

    # Check for cooldown period
    if echo "$RESPONSE" | grep -q "lease wait"; then
        WAIT_TIME=$(echo "$RESPONSE" | grep -o "wait [^.]*" | head -1)
        echo "   Please $WAIT_TIME before trying again"
    fi

    sed -i "s/^Part$PART: $ANSWER.*$/Part$PART: $ANSWER [Status: Incorrect]/" "$ANSWER_FILE"

elif echo "$RESPONSE" | grep -q "You don't seem to be solving the right level"; then
    echo "⚠ Already solved or wrong level"
    echo "   Check status with: just check-status $DAY_NUM"

elif echo "$RESPONSE" | grep -q "Did you already complete it"; then
    echo "✓ Already completed!"
    sed -i "s/^Part$PART: $ANSWER.*$/Part$PART: $ANSWER [Status: Correct]/" "$ANSWER_FILE"

else
    echo "⚠ Unexpected response"
    echo "   Check manually at: https://adventofcode.com/$YEAR/day/$DAY_NUM"
fi

echo "----------------------------------"
