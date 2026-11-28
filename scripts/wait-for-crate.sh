#!/usr/bin/env bash
# Wait until a crate version is listed in the crates.io sparse index.
# Usage: scripts/wait-for-crate.sh <crate> <version> [timeout-seconds]
#
# `cargo publish` waits for the index before it returns, but only as seen
# from the machine that published. Another machine (the next CI job) can
# still be served an older copy of the index file for a while, and then
# `cargo publish` of a dependent crate fails to select the new version
# (#1368). Run this on the machine that publishes the dependent crate.
set -euo pipefail

crate="${1:?usage: $0 <crate> <version> [timeout-seconds]}"
version="${2:?usage: $0 <crate> <version> [timeout-seconds]}"
timeout="${3:-600}"
interval=10

# Sparse index path: 1/<name>, 2/<name>, 3/<c>/<name>, else <ab>/<cd>/<name>,
# all lowercase.
name=$(printf '%s' "$crate" | tr '[:upper:]' '[:lower:]')
case ${#name} in
    1) path="1/$name" ;;
    2) path="2/$name" ;;
    3) path="3/${name:0:1}/$name" ;;
    *) path="${name:0:2}/${name:2:2}/$name" ;;
esac
url="https://index.crates.io/$path"

start=$SECONDS
while :; do
    # One JSON object per published version; match the version exactly.
    # Read the whole file first: `curl | grep -q` fails under pipefail when
    # grep stops reading early.
    index=$(curl -fsS "$url" 2>/dev/null) || index=""
    if grep -qF "\"vers\":\"$version\"" <<<"$index"; then
        echo "$crate $version is in the crates.io index"
        exit 0
    fi
    elapsed=$((SECONDS - start))
    if [ "$elapsed" -ge "$timeout" ]; then
        echo "error: $crate $version is not in the crates.io index after ${timeout}s ($url)" >&2
        exit 1
    fi
    echo "waiting for $crate $version in the crates.io index (${elapsed}s/${timeout}s)"
    sleep "$interval"
done
