#!/usr/bin/env bash
# sync-deps.sh — install dependencies whose lockfile changed, so a `git pull` /
# `git checkout` leaves node_modules and the cargo cache matching the tree.
#
#   scripts/sync-deps.sh                 # compare ORIG_HEAD..HEAD (post-merge / post-rewrite)
#   scripts/sync-deps.sh --since <rev>   # compare <rev>..HEAD   (post-checkout passes the old HEAD)
#   scripts/sync-deps.sh --all           # install everything (bootstrap / fresh clone)
#
# Runs npm ci / pnpm install --frozen-lockfile / cargo fetch only for the lockfiles that
# changed. Never runs in CI. Exit code is 0 even if an install fails, so a hook can't
# block git; failures are printed.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 0
[ -n "${CI:-}" ] && exit 0
[ -n "${SYNC_DEPS_SKIP:-}" ] && exit 0

mode="range"; since="ORIG_HEAD"
case "${1:-}" in
  --all) mode="all" ;;
  --since) since="${2:-ORIG_HEAD}" ;;
esac

changed() {  # print tracked paths matching the pathspecs that changed in the range (or all, in --all mode)
  if [ "$mode" = all ]; then git ls-files -- "$@"
  else git rev-parse -q --verify "$since" >/dev/null 2>&1 || return 0
       git diff --name-only "$since" HEAD -- "$@" 2>/dev/null
  fi
}

run() {  # run <dir> <label> <cmd...>
  local dir=$1 label=$2; shift 2
  echo "sync-deps: $label in ./$dir"
  ( cd "$dir" && "$@" ) || echo "sync-deps: WARNING: '$*' failed in ./$dir — run it by hand"
}

while IFS= read -r lock; do [ -n "$lock" ] || continue
  d=$(dirname "$lock"); [ -f "$d/package.json" ] || continue
  run "$d" "npm ci" npm ci --no-fund --no-audit
done < <(changed 'package-lock.json' '*/package-lock.json' '**/package-lock.json' | grep -v node_modules | sort -u)

while IFS= read -r lock; do [ -n "$lock" ] || continue
  d=$(dirname "$lock"); [ -f "$d/package.json" ] || continue
  run "$d" "pnpm install --frozen-lockfile" pnpm install --frozen-lockfile
done < <(changed 'pnpm-lock.yaml' '*/pnpm-lock.yaml' '**/pnpm-lock.yaml' | grep -v node_modules | sort -u)

while IFS= read -r lock; do [ -n "$lock" ] || continue
  d=$(dirname "$lock"); [ -f "$d/Cargo.toml" ] || continue
  command -v cargo >/dev/null 2>&1 || continue
  run "$d" "cargo fetch" cargo fetch --quiet
done < <(changed 'Cargo.lock' '*/Cargo.lock' '**/Cargo.lock' | sort -u)
exit 0
