#!/usr/bin/env bash
set -euo pipefail

# Input validation
if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <YEAR> <PKG>" >&2
    exit 1
fi

YEAR=$1
PKG=$2
PKG_PATH="crates/$PKG"

# Verify package exists
if [[ ! -d "$PKG_PATH" ]]; then
    echo "Error: Package directory '$PKG_PATH' not found" >&2
    exit 1
fi

echo "=== Advent of Code $YEAR Stats ==="

# Count completed days
shopt -s nullglob
files=("$PKG_PATH"/src/bin/d*.rs)
echo "Days completed: ${#files[@]}"
shopt -u nullglob

# Count total lines
total_lines=$(find "$PKG_PATH/src" -name 'd*.rs' -type f -print0 |
    xargs -0 wc -l |
    tail -1 |
    awk '{print $1}')
echo "Total lines: $total_lines"

# Test results
echo -n "Tests passed: "
if test_output=$(cargo test -p "$PKG" --quiet 2>&1); then
    echo "$test_output" | grep -o '[0-9]* passed' || echo 'N/A'
else
    echo "N/A (tests failed or package not found)"
fi
