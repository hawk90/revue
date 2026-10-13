#!/bin/sh
# Fail when a staged change widens a struct field's visibility: a private
# field that becomes `pub(super)`, `pub(crate)` or `pub` in the same hunk.
#
# Splitting a type's methods into sibling files must not open its fields so
# the new files compile; make the new files child modules of the module that
# defines the type instead (docs/anti-patterns/catalog.yaml, REV-RUST-011).
#
# A deliberate change of the public surface can bypass the check with
# ALLOW_FIELD_WIDENING=1 and must say why in the commit message.
[ "${ALLOW_FIELD_WIDENING:-0}" = "1" ] && exit 0

git diff --cached -U0 -- '*.rs' | awk '
/^diff --git/ { file = $4; sub(/^b\//, "", file) }
/^@@/ { split("", removed); next }
/^-[ \t]+[a-z_][a-z0-9_]*[ \t]*:[^:]/ {
    line = $0; sub(/^-[ \t]+/, "", line); sub(/[ \t]*:.*/, "", line)
    removed[line] = 1; next
}
/^\+[ \t]+pub(\((super|crate)\))?[ \t]+[a-z_][a-z0-9_]*[ \t]*:[^:]/ {
    line = $0; sub(/^\+[ \t]+pub(\((super|crate)\))?[ \t]+/, "", line); sub(/[ \t]*:.*/, "", line)
    if (line in removed) {
        printf "%s: field `%s` widened: %s\n", file, line, substr($0, 2)
        bad = 1
    }
}
END { exit bad }
'
