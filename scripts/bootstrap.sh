#!/usr/bin/env bash
# bootstrap.sh — one-shot, idempotent local setup: toolchains from the version files,
# dependencies from the lockfiles, git hooks. Safe to re-run any time.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

if command -v mise >/dev/null 2>&1; then
  mise install                       # node (.node-version) and anything in mise.toml
else
  echo "bootstrap: mise not found — install it (brew install mise) so .node-version is honoured" >&2
fi
if [ -f rust-toolchain.toml ] && command -v rustup >/dev/null 2>&1; then
  rustup show active-toolchain >/dev/null   # installs the pinned toolchain on first use
fi
if command -v corepack >/dev/null 2>&1; then
  corepack enable >/dev/null 2>&1 || true   # honours "packageManager" in package.json
fi

scripts/sync-deps.sh --all

if [ -f lefthook.yml ] || [ -f lefthook.toml ]; then
  command -v lefthook >/dev/null 2>&1 && lefthook install || echo "bootstrap: lefthook not installed (brew install lefthook)" >&2
elif [ -d .githooks ]; then
  git config core.hooksPath .githooks
fi
echo "bootstrap: done"
