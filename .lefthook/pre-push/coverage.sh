#!/usr/bin/env bash
# pre-push `coverage.sh` (lefthook.yml): quick coverage check when any commit
# being pushed changes src/. Local only. A lefthook script for the same reason
# as test.sh.
set -u
[ -n "${CI:-}" ] && exit 0
cd "$(git rev-parse --show-toplevel)" || exit 1

PUSHED=$(scripts/pushed-files.sh)
case $? in
  0) printf '%s\n' "$PUSHED" | grep -q '^src/' || exit 0 ;;
  2) ;;
  *) exit 1 ;;
esac

echo "📊 Running coverage check..."
# This will fail if any tests fail
cargo llvm-cov --lib --no-report
