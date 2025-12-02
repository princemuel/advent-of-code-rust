#!/usr/bin/env bash
# Download puzzle input from Advent of Code
# Usage: ./download_input.sh YEAR DAY

set -euo pipefail

YEAR="${1:-}"
DAY="${2:-}"

if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
    echo "Usage: $0 YEAR DAY"
    exit 1
fi

# Pad day to 2 digits for file naming
DAY=$(printf "%02d" "$DAY")
DAY_NUM=$((10#$DAY)) # Remove leading zeros for URL

OUTPUT_DIR="inputs/$YEAR"
OUTPUT_FILE="$OUTPUT_DIR/d$DAY.txt"

# Create directory if needed
mkdir -p "$OUTPUT_DIR"

# Check if file already exists
if [ -f "$OUTPUT_FILE" ]; then
    read -p "Input file exists. Overwrite? (y/N) " -n 1 -r
    echo
    [[ ! $REPLY =~ ^[Yy]$ ]] && exit 0
fi

# Load session token from .env
if [ ! -f .env ]; then
    echo "Error: .env file not found"
    echo "Create .env with: TOKEN=your_session_token"
    exit 1
fi

set -a
source .env
set +a

if [ -z "${TOKEN:-}" ]; then
    echo "Error: TOKEN not found in .env"
    exit 1
fi

# Download input
echo "Downloading input for Year $YEAR Day $DAY_NUM..."
HTTP_CODE=$(curl -s -w "%{http_code}" \
    --cookie "session=$TOKEN" \
    -H "User-Agent: github.com/advent-of-code-rust" \
    "https://adventofcode.com/$YEAR/day/$DAY_NUM/input" \
    -o "$OUTPUT_FILE")

if [ "$HTTP_CODE" = "200" ]; then
    echo "✓ Successfully downloaded to $OUTPUT_FILE"
    # Show preview
    LINE_COUNT=$(wc -l <"$OUTPUT_FILE")
    echo "   ($LINE_COUNT lines)"
    head -n 3 "$OUTPUT_FILE"
    [ "$LINE_COUNT" -gt 3 ] && echo "   ..."
else
    echo "✗ Failed to download (HTTP $HTTP_CODE)"
    [ -f "$OUTPUT_FILE" ] && cat "$OUTPUT_FILE" # Show error message
    rm -f "$OUTPUT_FILE"
    exit 1
fi
