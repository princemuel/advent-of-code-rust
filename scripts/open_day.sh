#!/usr/bin/env bash
set -euo pipefail

YEAR=$1
DAY=$2

URL="https://adventofcode.com/$YEAR/day/$DAY"

xdg-open "$URL" 2>/dev/null || open "$URL" 2>/dev/null || echo "Visit $URL"
