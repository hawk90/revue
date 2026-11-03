#!/bin/sh
# The public API of revue, listed by cargo-public-api (#1172).
#
#   scripts/public-api.sh          fail when tests/public-api.txt is not the current API
#   scripts/public-api.sh --bless  rewrite tests/public-api.txt
#   scripts/public-api.sh --removed BASE
#                                  fail when the snapshot lost lines since BASE
#
# A PR that adds public items blesses the snapshot in the same PR, so the
# reviewers see every addition. CI also fails a PR that removes or changes a
# line of the snapshot unless its title marks it as breaking (`feat!:`), which
# keeps 3.x additive.
#
# rustdoc's JSON output exists only on nightly, its format changes between
# nightlies, and nightly-only auto traits (Freeze, UnsafeUnpin) show up in the
# list. So the nightly and the tool are pinned: bump both together and bless.
set -eu

NIGHTLY=nightly-2026-10-07
TOOL_VERSION=0.52.0
SNAPSHOT=tests/public-api.txt

cd "$(dirname "$0")/.."

# --removed BASE: list the snapshot lines removed since BASE (a changed item is
# a removed line and an added one) and fail when there are any. A line whose
# only change is the path of a type the old snapshot does not list (so no
# user could name it), now re-exported under the same name, is not counted
# (#1445; scripts/public-api-removed.py).
if [ "${1:-}" = "--removed" ]; then
    base=$(git merge-base "${2:?usage: --removed BASE}" HEAD)
    old=$(mktemp)
    diff=$(mktemp)
    trap 'rm -f "$old" "$diff"' EXIT
    git show "$base:$SNAPSHOT" >"$old"
    git diff -U0 "$base" HEAD -- "$SNAPSHOT" >"$diff"
    python3 scripts/public-api-removed.py "$old" "$diff"
    exit
fi

if ! cargo public-api --version 2>/dev/null | grep -qx "cargo-public-api $TOOL_VERSION"; then
    echo "error: needs cargo-public-api $TOOL_VERSION:" >&2
    echo "  cargo install cargo-public-api --version $TOOL_VERSION --locked" >&2
    echo "  rustup toolchain install $NIGHTLY --profile minimal" >&2
    exit 1
fi

current=$(mktemp)
trap 'rm -f "$current"' EXIT

# -s leaves out blanket impls (`impl<T> From<T> for T` and the like), which
# every type has. Auto trait impls stay: a type that stops being Send or Sync
# breaks its callers.
cargo "+$NIGHTLY" public-api -p revue --all-features -s --color never >"$current"

if [ "${1:-}" = "--bless" ]; then
    cp "$current" "$SNAPSHOT"
    echo "wrote $SNAPSHOT"
    exit 0
fi

if ! diff -u "$SNAPSHOT" "$current"; then
    echo >&2
    echo "error: the public API differs from $SNAPSHOT (- snapshot, + current)." >&2
    echo "Run scripts/public-api.sh --bless and commit $SNAPSHOT with the change." >&2
    exit 1
fi
echo "$SNAPSHOT matches the public API"
