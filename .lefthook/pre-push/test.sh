#!/usr/bin/env bash
# pre-push `test.sh` (lefthook.yml): run the tests for the Rust files changed by
# every commit being pushed. A lefthook script, not a `run` command, because
# lefthook skips a pre-push command when HEAD has nothing to push, even if the
# push sends another ref (`git push origin other-branch`).
set -u
cd "$(git rev-parse --show-toplevel)" || exit 1

PUSHED=$(scripts/pushed-files.sh)
case $? in
  0) ;;
  2)
    echo "Running all tests (cannot tell what is pushed)..."
    cargo test --all-features || exit 1
    exit 0
    ;;
  *) exit 1 ;;
esac

CHANGED=$(printf '%s\n' "$PUSHED" | grep '\.rs$' || true)
if [ -z "$CHANGED" ]; then
  echo "No Rust files changed, skipping tests"
  exit 0
fi

echo "Testing changed files:"
echo "$CHANGED"
# Run all tests if any src/ file changed
if echo "$CHANGED" | grep -q "^src/"; then
  echo "Running all tests (src/ changed)..."
  cargo test --all-features || exit 1
  exit 0
fi

# Run tests for specific test files that changed
TEST_TARGETS=()
for file in $CHANGED; do
  if echo "$file" | grep -qE "^tests/[^/]+/"; then
    # Test directories - run all tests
    echo "Test directory changed, running all tests..."
    cargo test --all-features || exit 1
    exit 0
  elif echo "$file" | grep -qE "^tests/[^/]+\.rs$" && [ -f "$file" ]; then
    # Top-level test files (e.g., tests/buffer_tests.rs); a deleted
    # one is no longer a target
    test_name=$(basename "$file" .rs)
    TEST_TARGETS+=("--test" "$test_name")
  fi
done
if [ ${#TEST_TARGETS[@]} -gt 0 ]; then
  echo "Running test targets: ${TEST_TARGETS[*]}"
  cargo test "${TEST_TARGETS[@]}" --all-features || exit 1
fi
