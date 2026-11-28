#!/usr/bin/env bash
# pushed-files.sh — list the files a `git push` changes on the remote, for the
# pre-push hook to choose tests from (#939).
#
#   scripts/pushed-files.sh < pre-push stdin
#
# git hands the pre-push hook one line per ref being pushed:
#   <local ref> <local sha> <remote ref> <remote sha>
# For each ref this diffs <remote sha>..<local sha>, so every commit being pushed
# counts, not only the last one. A new branch (remote sha all zeros) or a remote
# sha this clone does not have is diffed from its merge base with origin/main.
# A deleted ref (local sha all zeros) pushes nothing to test.
#
# Prints the changed paths (old and new path of a rename), one per line, sorted.
# Exits 2, with the reason on stderr, when the hook cannot tell what to test and
# should run everything:
#   - no ref lines on stdin (e.g. `lefthook run pre-push` by hand),
#   - no merge base with origin/main for a new branch,
#   - a pushed commit whose tree is not HEAD's, e.g. `git push origin other-branch`:
#     the tests build the working tree, not the commit being pushed.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 2

full() {
  echo "pushed-files: $*; running everything" >&2
  exit 2
}

if [ -t 0 ]; then
  full "no pre-push refs on stdin"
fi

zero='^0+$'
head_tree=$(git rev-parse --verify --quiet 'HEAD^{tree}')
refs=0
changed=""
while read -r local_ref local_sha remote_ref remote_sha; do
  [ -n "${local_ref:-}" ] || continue
  refs=$((refs + 1))
  # Deleting a remote ref pushes no content.
  if [[ $local_sha =~ $zero ]]; then
    continue
  fi
  # A tag pushes its commit; anything that is no commit (a tag on a tree or blob)
  # has no tree to compare.
  local_tree=$(git rev-parse --verify --quiet "$local_sha^{tree}") ||
    full "$local_ref is not a commit"
  if [ "$local_tree" != "$head_tree" ]; then
    full "$local_ref ($local_sha) is not the checked-out HEAD, so the working tree is not what is pushed to $remote_ref"
  fi
  if [[ ! $remote_sha =~ $zero ]] && git cat-file -e "$remote_sha^{commit}" 2>/dev/null; then
    base=$remote_sha
  else
    base=$(git merge-base origin/main "$local_sha" 2>/dev/null) ||
      full "no merge base between $local_ref and origin/main"
  fi
  files=$(git diff --name-only --no-renames "$base" "$local_sha") ||
    full "git diff $base $local_sha failed"
  changed+="$files"$'\n'
done

[ "$refs" -gt 0 ] || full "no pre-push refs on stdin"

printf '%s' "$changed" | sed '/^$/d' | sort -u
