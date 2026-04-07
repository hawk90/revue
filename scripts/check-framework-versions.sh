#!/usr/bin/env bash
# Check latest versions of competing TUI frameworks
# Used by CI to detect when comparison table needs updating
set -euo pipefail

echo "## TUI Framework Version Check"
echo ""
echo "| Framework | Latest Version | Source |"
echo "|-----------|---------------|--------|"

# ratatui (Rust)
RATATUI=$(curl -s "https://crates.io/api/v1/crates/ratatui" | grep -o '"newest_version":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "| ratatui | ${RATATUI:-unknown} | crates.io |"

# reratui (Rust)
RERATUI=$(curl -s "https://crates.io/api/v1/crates/reratui" | grep -o '"newest_version":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "| reratui | ${RERATUI:-unknown} | crates.io |"

# cursive (Rust)
CURSIVE=$(curl -s "https://crates.io/api/v1/crates/cursive" | grep -o '"newest_version":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "| cursive | ${CURSIVE:-unknown} | crates.io |"

# textual (Python)
TEXTUAL=$(curl -s "https://pypi.org/pypi/textual/json" | grep -o '"version":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "| textual | ${TEXTUAL:-unknown} | pypi.org |"

echo ""

# Check last verified date in README
LAST_VERIFIED=$(grep -o 'Last verified: [0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}' README.md 2>/dev/null | grep -o '[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}' || echo "not found")
echo "Last verified in README: ${LAST_VERIFIED}"

# Calculate days since last verification
if [ "$LAST_VERIFIED" != "not found" ]; then
  LAST_EPOCH=$(date -d "$LAST_VERIFIED" +%s 2>/dev/null || date -j -f "%Y-%m-%d" "$LAST_VERIFIED" +%s 2>/dev/null || echo 0)
  NOW_EPOCH=$(date +%s)
  if [ "$LAST_EPOCH" -gt 0 ]; then
    DAYS_AGO=$(( (NOW_EPOCH - LAST_EPOCH) / 86400 ))
    echo "Days since last check: ${DAYS_AGO}"
    if [ "$DAYS_AGO" -gt 90 ]; then
      echo ""
      echo "⚠️ Framework comparison table has not been verified in ${DAYS_AGO} days."
      echo "Please review and update the comparison in README.md."
      exit 1
    fi
  fi
fi
