#!/usr/bin/env bash
set -euo pipefail

PKG=$1
DAY=$2
INPUT=$3

echo "Running $PKG day $DAY (debug) with $INPUT..."
cat "$INPUT" | cargo run -p "$PKG" --bin d"$DAY"
