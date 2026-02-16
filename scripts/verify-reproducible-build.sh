#!/usr/bin/env bash
# Verify reproducible builds by comparing checksums between two builds.
# Usage: ./scripts/verify-reproducible-build.sh [target]
set -euo pipefail

TARGET="${1:-x86_64-pc-windows-msvc}"
BINARY_NAME="nos-autoclicker"

echo "=== Reproducible Build Verification ==="
echo "Target: $TARGET"
echo ""

# Build 1
echo "Building pass 1..."
cargo build --release --locked --target "$TARGET" 2>/dev/null || {
    echo "Cross-compile target not available. Falling back to native build..."
    TARGET=$(rustc -Vv | grep host | awk '{print $2}')
    cargo build --release --locked 2>/dev/null
}

if [[ "$TARGET" == *"windows"* ]]; then
    BINARY="target/$TARGET/release/${BINARY_NAME}.exe"
else
    BINARY="target/$TARGET/release/${BINARY_NAME}"
fi

# Fallback to native path if cross-compile target not found
if [[ ! -f "$BINARY" ]]; then
    BINARY="target/release/${BINARY_NAME}"
fi

if [[ ! -f "$BINARY" ]]; then
    echo "ERROR: Binary not found at $BINARY"
    exit 1
fi

HASH1=$(sha256sum "$BINARY" | awk '{print $1}')
echo "Pass 1 SHA-256: $HASH1"

# Save binary
cp "$BINARY" /tmp/build1_binary

# Build 2
echo "Building pass 2..."
cargo clean --release 2>/dev/null || true
if [[ "$TARGET" == *"windows"* ]]; then
    cargo build --release --locked --target "$TARGET" 2>/dev/null || cargo build --release --locked 2>/dev/null
else
    cargo build --release --locked 2>/dev/null
fi

if [[ ! -f "$BINARY" ]]; then
    BINARY="target/release/${BINARY_NAME}"
fi

HASH2=$(sha256sum "$BINARY" | awk '{print $1}')
echo "Pass 2 SHA-256: $HASH2"

echo ""
if [[ "$HASH1" == "$HASH2" ]]; then
    echo "✓ PASS: Builds are reproducible!"
    echo "  SHA-256: $HASH1"
    
    # Generate manifest
    echo ""
    echo "=== Release Manifest ==="
    echo "{"
    echo "  \"version\": \"$(cargo pkgid | cut -d# -f2)\","
    echo "  \"git_tag\": \"$(git describe --tags --always 2>/dev/null || echo 'none')\","
    echo "  \"git_commit\": \"$(git rev-parse HEAD 2>/dev/null || echo 'none')\","
    echo "  \"rust_toolchain\": \"$(rustc --version)\","
    echo "  \"target\": \"$TARGET\","
    echo "  \"binary_sha256\": \"$HASH1\""
    echo "}"
else
    echo "✗ FAIL: Builds are NOT reproducible!"
    echo "  Pass 1: $HASH1"
    echo "  Pass 2: $HASH2"
    exit 1
fi

# Cleanup
rm -f /tmp/build1_binary
