# Force bash for more predictable behavior
set shell := ["bash", "-cu"]

# Build the aocctl binary
build:
    cargo build --package aocctl

# Detect operating system
os := if os() == "linux" {
    "linux"
} else if os() == "macos" {
    "darwin"
} else if os() == "windows" {
    "windows"
} else {
    error("Unsupported OS")
}

# Detect architecture
arch := if arch() == "x86_64" {
    "x86_64"
} else if arch() == "aarch64" {
    "aarch64"
} else {
    error("Unsupported architecture: " + arch())
}

# Determine Rust target triple
target := if os == "linux" {
    arch + "-unknown-linux-musl"
} else if os == "darwin" {
    arch + "-apple-darwin"
} else {
    arch + "-pc-windows-msvc"
}

# Install the CLI tool with auto-detected target
install:
    @echo "Detected OS: {{os}}"
    @echo "Detected ARCH: {{arch}}"
    @echo "Using Rust target: {{target}}"
    @if ! rustup target list | grep -q "{{target}} (installed)"; then \
        echo "Installing Rust target: {{target}}"; \
        rustup target add "{{target}}"; \
    fi
    cargo install --target "{{target}}" --path crates/aocctl
    @echo "Installed aocctl for {{target}} ✓"

# Create a new day using a selected template
# Usage: just new 3 minimal
new day template="minimal":
    cargo run --package aocctl -- new {{day}} --template {{template}}

# Run the latest day
run:
    cargo run --package aocctl -- current

# Run a specific day
run-day day input="puzzle":
    cargo run --package aocctl -- run {{day}} {{input}}

# Solve + optionally submit
solve day:
    cargo run --package aocctl -- solve {{day}}

# Download input
input day:
    cargo run --package aocctl -- input {{day}}

# Open puzzle page in browser
open day:
    cargo run --package aocctl -- open {{day}}

# Initialize a new year
init year:
    cargo run --package aocctl -- init {{year}}

# List templates
templates:
    cargo run --package aocctl -- list-templates

# Clean up build artifacts
clean:
    cargo clean
