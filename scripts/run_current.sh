#!/usr/bin/env bash
set -euo pipefail

YEAR=$1
_PKG=$2
DAY=$3
INPUT=$4

if [[ -z "$DAY" ]]; then
    echo "No days found for year $YEAR."
    exit 1
fi

just run-release "$DAY" "$INPUT"
