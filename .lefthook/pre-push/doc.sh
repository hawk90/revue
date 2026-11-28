#!/usr/bin/env bash
# pre-push `doc.sh` (lefthook.yml): build the docs with the same flags as CI and
# CD, so a broken intra-doc link is an error, not a warning. A lefthook script
# for the same reason as test.sh: lefthook skips a pre-push `run` command when
# HEAD has nothing to push, even if the push sends another ref (#1580).
set -u
cd "$(git rev-parse --show-toplevel)" || exit 1

# test.sh reports what pushed-files.sh says; say it once
PUSHED=$(scripts/pushed-files.sh 2>/dev/null)
case $? in
  0) [ -n "$PUSHED" ] || exit 0 ;; # nothing new is pushed (a deleted ref, a tag main has)
  2) ;;
  3) exit 0 ;; # refused: test.sh fails the push and says why
  *) exit 1 ;;
esac

RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc --no-deps --all-features --document-private-items
