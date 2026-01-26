#!/bin/bash
# Manual publish script for revue and revue-macros
# Usage: ./scripts/publish-manual.sh [version]
# If version is not provided, it will be read from .release-please-manifest.json

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Get version from argument or manifest
if [ -n "$1" ]; then
    VERSION="$1"
else
    VERSION=$(cat .release-please-manifest.json | jq -r '."."')
    if [ "$VERSION" = "null" ] || [ -z "$VERSION" ]; then
        echo "Error: Could not read version from .release-please-manifest.json"
        echo "Usage: $0 [version]"
        exit 1
    fi
fi

echo "=== Publishing revue v$VERSION ==="

# Check if token is set
if [ -z "$CRATES_IO_TOKEN" ]; then
    echo "Error: CRATES_IO_TOKEN environment variable is not set"
    echo "Set it with: export CRATES_IO_TOKEN=your_token"
    exit 1
fi

# Publish revue-macros first
echo ""
echo "Step 1: Publishing revue-macros..."
cd revue-macros
cargo publish --token "$CRATES_IO_TOKEN"

# Wait for revue-macros to be available on crates.io
echo ""
echo "Waiting for revue-macros to be available on crates.io..."
sleep 30

# Publish revue
echo ""
echo "Step 2: Publishing revue..."
cd ..
cargo publish --token "$CRATES_IO_TOKEN"

echo ""
echo "=== Successfully published revue v$VERSION ==="
