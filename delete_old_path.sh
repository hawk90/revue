#!/bin/bash
# Script to delete the old path.rs file and test the new module structure

set -e

echo "Deleting old src/utils/path.rs file..."
rm /Users/hawk/Workspaces/revue/src/utils/path.rs

echo "Running cargo check..."
cargo check

echo "Running cargo test..."
cargo test --package revue --lib utils::path

echo "All tests passed!"
