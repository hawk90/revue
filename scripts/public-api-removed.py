#!/usr/bin/env python3
"""The snapshot lines removed since a base, minus path-only changes.

Usage: public-api-removed.py OLD_SNAPSHOT DIFF

OLD_SNAPSHOT is tests/public-api.txt at the merge base; DIFF is
`git diff -U0` of it from there. Prints each removed line that is a real
removal or change and exits 1 when there are any.

A removed line is not a removal when an added line is the same line with
only type paths changed, each from a path the old snapshot never lists as
an item (so no user could name it, e.g. a type in a private module) to a
path ending in the same name. Re-exporting such a type changes how the
signatures that use it print, and nothing breaks (#1445).
"""
import re
import sys

PATH = re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)+")
# An item line: optional attributes, `pub`, a kind, then the item's path
ITEM = re.compile(
    r"^(?:#\[[^\]]*\]\s*)*pub\s+(?:unsafe\s+|const\s+|async\s+|extern\s+\S+\s+)*"
    r"(?:struct|enum|union|trait|type|fn|const|static|mod|macro|auto\s+trait)\s+"
    r"([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)"
)


def named_paths(lines):
    """Every item path the snapshot lists"""
    named = set()
    for line in lines:
        m = ITEM.match(line)
        if m:
            named.add(m.group(1))
    return named


def masked(line, keep):
    """`line` with each path replaced by its last segment unless `keep`
    says it must match exactly"""
    out = []
    last = 0
    for m in PATH.finditer(line):
        out.append(line[last:m.start()])
        path = m.group(0)
        out.append(path if keep(path) else "@" + path.rsplit("::", 1)[1])
        last = m.end()
    out.append(line[last:])
    return "".join(out)


def main():
    old_snapshot, diff = sys.argv[1], sys.argv[2]
    with open(old_snapshot, encoding="utf-8") as f:
        named = named_paths(f.read().splitlines())
    removed, added = [], []
    with open(diff, encoding="utf-8") as f:
        for line in f.read().splitlines():
            if line.startswith("---") or line.startswith("+++"):
                continue
            if line.startswith("-"):
                removed.append(line[1:])
            elif line.startswith("+"):
                added.append(line[1:])

    # A removed line: its unnameable paths may become any path with the
    # same last segment; every other path must stay as it was
    def removal_key(line):
        return masked(line, lambda p: p in named)

    def unnameable_names(line):
        paths = (m.group(0) for m in PATH.finditer(line))
        return {p.rsplit("::", 1)[1] for p in paths if p not in named}

    real = []
    for line in removed:
        names = unnameable_names(line)
        if names:
            key = removal_key(line)
            # In an added line, mask only the paths ending in one of those
            # names; the rest must match the removed line's exactly
            ok = any(
                masked(a, lambda p: p.rsplit("::", 1)[1] not in names) == key for a in added
            )
            if ok:
                continue
        real.append(line)

    for line in real:
        print("-" + line)
    sys.exit(1 if real else 0)


if __name__ == "__main__":
    main()
