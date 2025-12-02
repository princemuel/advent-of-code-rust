#!/usr/bin/env bash
set -euo pipefail

YEAR=$1
PKG="aoc$YEAR"

echo "Initializing year $YEAR..."

mkdir -p "crates/$PKG/src/bin" \
    "inputs/$YEAR" \
    "answers/$YEAR"

cat >"crates/$PKG/Cargo.toml" <<EOF
[package]
name = "$PKG"
version = "0.1.0"
edition = "2024"

[dependencies]
EOF

cat >"crates/$PKG/src/lib.rs" <<'EOF'
use std::io::{self, Read};

pub fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read failed");
    s.trim().to_string()
}
EOF

if ! grep -q "\"crates/$PKG\"" Cargo.toml; then
    sed -i "/members = \[/a\    \"crates/$PKG\"," Cargo.toml
    echo "Added $PKG to workspace"
fi

echo "✓ Year $YEAR initialized."
