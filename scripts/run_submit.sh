#!/usr/bin/env bash
# Run solution and interactively submit answers
# Usage: ./run_submit.sh YEAR DAY INPUT

set -euo pipefail

YEAR="${1:-}"
DAY="${2:-}"
INPUT="${3:-}"

if [[ -z "$YEAR" || -z "$DAY" || -z "$INPUT" ]]; then
    echo "Usage: $0 YEAR DAY INPUT"
    exit 1
fi

DAY=$(printf "%02d" "$DAY")
DAY_NUM=$((DAY))
PKG="aoc$YEAR"

# ensure package exists
if ! cargo metadata --no-deps --format-version 1 2>/dev/null | grep -q "\"name\": \"$PKG\""; then
    echo "Error: Cargo package '$PKG' not found in workspace."
    exit 1
fi

# Special input keywords
case "$INPUT" in
download)
    bash scripts/download_input.sh "$YEAR" "$DAY"
    INPUT="inputs/$YEAR/d$DAY.txt"
    ;;
puzzle | puzzle_input)
    if [[ -f "inputs/$YEAR/d$DAY.txt" ]]; then
        INPUT="inputs/$YEAR/d$DAY.txt"
    elif [[ -f "inputs/$YEAR/input.txt" ]]; then
        INPUT="inputs/$YEAR/input.txt"
    else
        echo "Error: No puzzle input found in inputs/$YEAR/" >&2
        echo "Run: just download $DAY_NUM" >&2
        exit 1
    fi
    ;;
esac

# Input file must exist
if [[ ! -f "$INPUT" ]]; then
    echo "Error: Input file not found: $INPUT" >&2
    exit 1
fi

mkdir -p "answers/$YEAR"
ANSWER_FILE="answers/$YEAR/submit_d$DAY.txt"

echo "----------------------------------"
echo "Running Year $YEAR Day $DAY"
echo "----------------------------------"

# Build
echo "Building $PKG day $DAY..."
cargo build -p "$PKG" --release --bin "d$DAY" 2>&1 |
    grep -vE "Compiling|Finished" || true

# Run
echo "Running solution..."
echo ""

TIMEOUT="${AOC_TIMEOUT:-60}"

if ! OUTPUT=$(timeout "$TIMEOUT"s target/release/d"$DAY" <"$INPUT" 2>&1); then
    echo "✗ Solution failed or timed out"
    exit 1
fi

echo "$OUTPUT"
echo ""
echo "----------------------------------"

# Extract answers (flexible)
# shellcheck disable=SC1087
# extract_answer() {
#     local part="$1"
#     echo "$OUTPUT" |
#         grep -iE "part[[:space:]]*$part[[:space:]]*:" |
#         sed -E "s/.*part[[:space:]]*$part[[:space:]]*:[[:space:]]*//I" |
#         head -n1
# }

extract_answer() {
    local part="$1"

    printf '%s\n' "$OUTPUT" |
        grep -iE "part[[:space:]]*$part[[:space:]]*:" |
        sed -E "s/.*part[[:space:]]*$part[[:space:]]*:[[:space:]]*//I" |
        head -n1
}

PART1="$(extract_answer 1)"
PART2="$(extract_answer 2)"

# Save answers
: >"$ANSWER_FILE"
[[ -n "$PART1" ]] && echo "Part1: $PART1" >>"$ANSWER_FILE"
[[ -n "$PART2" ]] && echo "Part2: $PART2" >>"$ANSWER_FILE"

if [[ -z "$PART1" && -z "$PART2" ]]; then
    echo "⚠ Could not extract answers."
    exit 1
fi

echo "Answers saved to $ANSWER_FILE"
echo ""

STATUS_OUTPUT=$(bash scripts/check_status.sh "$YEAR" "$DAY")
echo "$STATUS_OUTPUT"
echo ""

PART1_DONE=$(echo "$STATUS_OUTPUT" | grep -F "Part 1: ✓" || true)
PART2_DONE=$(echo "$STATUS_OUTPUT" | grep -F "Part 2: ✓" || true)

SUBMITTED_PART1=0
SUBMITTED_PART2=0

# Submit part 1
if [[ -n "$PART1" && -z "$PART1_DONE" ]]; then
    read -p "Submit Part 1 ($PART1)? (y/N) " -n 1 -r
    echo
    if [[ "$REPLY" =~ ^[Yy]$ ]]; then
        bash scripts/submit.sh "$YEAR" "$DAY" 1
        SUBMITTED_PART1=1
    fi
fi

# If part 1 just accepted and part 2 available
if [[ $SUBMITTED_PART1 -eq 1 && -n "$PART2" ]]; then
    if grep -q "Part1:.*\[Status: Correct\]" "$ANSWER_FILE"; then
        echo ""
        echo "Part 1 correct! Cooling down 45 seconds..."
        for ((i = 45; i >= 1; i--)); do
            printf "\rWaiting... %2ds" "$i"
            sleep 1
        done
        printf "\r\n"

        read -p "Submit Part 2 ($PART2)? (y/N) " -n 1 -r
        echo
        if [[ "$REPLY" =~ ^[Yy]$ ]]; then
            bash scripts/submit.sh "$YEAR" "$DAY" 2
            SUBMITTED_PART2=1
        fi
    fi
fi

# Submit part 2 when part 1 already solved earlier
if [[ -n "$PART2" && -z "$PART2_DONE" && -n "$PART1_DONE" && $SUBMITTED_PART1 -eq 0 ]]; then
    read -p "Submit Part 2 ($PART2)? (y/N) " -n 1 -r
    echo
    if [[ "$REPLY" =~ ^[Yy]$ ]]; then
        bash scripts/submit.sh "$YEAR" "$DAY" 2
        SUBMITTED_PART2=1
    fi
fi

if [[ $SUBMITTED_PART2 -eq 1 ]]; then
    echo "Part 2 was submitted in this run."
fi

echo ""
echo "----------------------------------"
STATUS1=$(grep "Part1:" "$ANSWER_FILE" | grep -o "\[Status: .*\]" || echo "[Not submitted]")
STATUS2=$(grep "Part2:" "$ANSWER_FILE" | grep -o "\[Status: .*\]" || echo "[Not submitted]")
[[ -n "$PART1" ]] && echo "Part 1: $PART1 $STATUS1"
[[ -n "$PART2" ]] && echo "Part 2: $PART2 $STATUS2"
echo "----------------------------------"
