#!/usr/bin/env bash
set -euo pipefail

_YEAR=$1
_PKG=$2
DAY=$3

echo "Quick solve for day $DAY..."
just download "$DAY"
just run-submit "$DAY" puzzle
