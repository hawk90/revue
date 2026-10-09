# docs/specs — feature registry

`features.yaml` is the **single source of truth for Revue's feature list and feature status**.
`docs/FEATURES.md` keeps only narrative (what the framework is for, design reasoning, known caveats);
don't put ✅/❌ status or per-feature checklists in Markdown. API details belong in rustdoc / docs.rs.

## Schema (`schema: 1`)

Each entry in `items:` has these fields, in this order (the file is written with PyYAML, `sort_keys=False`,
`allow_unicode`, indented lists, `width=120`; keep the order when editing by hand):

| Field | Required | Meaning |
|---|---|---|
| `id` | yes | `<AREA>-<NNN>`, e.g. `WID-025`. **Stable**: never renumber or reuse an id. New items take the next free number in their area. |
| `title` | yes | Short English name. |
| `area` | yes | `css` `layout` `reactive` `widget` `chart` `navigation` `unicode` `dx` `theme` `input` `util`. Prefix: CSS LAY RX WID CHART NAV TEXT DX THEME KEY UTIL. |
| `priority` | yes | `P0`..`P3`. P0 = core of the framework (cascade, layout, signals, basic widgets, testing); P1 = major feature; P2 = secondary widget/utility; P3 = niche. |
| `priority_inferred` | no | `true` when no source doc gave a priority (all items as of 2026-10-05 — FEATURES.md never had priorities). |
| `status` | yes | `done` · `partial` · `todo` · `dropped` (see below). |
| `phase` | no | String, release/milestone when one applies. Omit otherwise. |
| `acceptance` | yes | List of concrete, checkable criteria. |
| `evidence` | yes | List of repo paths proving the status: `path`, `path:line` or `path::symbol` (the symbol must appear literally in that file). Prefer the public constructor/method plus a test (`tests/x.rs::test_name`) or an example. Required non-empty for `done`/`partial`; `[]` for `todo`. Every path must exist. |
| `source` | yes | **List** of `"FEATURES.md §<section>"` strings. Sections refer to the pre-2026-10-05 `docs/FEATURES.md` layout (§1–§11, "Advanced Widgets", "Overview"); see git history for the old text. |
| `notes` | no | Gaps, caveats, doc-vs-code conflicts ("doc said X; code: Y"), known bugs. Required for `dropped`. |

## Status meanings

Revue is a library, so "usable" means reachable through the public API:

- **done**: the API is exported (crate root, `revue::widget`, `revue::prelude`, …) **and** covered by a
  test or used by an example, **and** the acceptance is met. A feature behind a Cargo feature flag counts
  (say which flag in `notes`).
- **partial**: some of the acceptance is met; or the code exists but is not wired (parsed but never
  applied, a flag nothing reads, a panel nothing mounts); or it is exported but neither tested nor used.
- **todo**: not started.
- **dropped**: intentionally out of scope or obsolete. The reason goes in `notes`.

When the docs and the code disagree, **the code wins**; record the difference in `notes`. Doc examples
that call non-existent methods (e.g. `.on_change(closure)` on state-based widgets) are API-shape conflicts,
not missing features — the item can still be `done`.

## How to update

1. Find the item: `grep -n "<keyword>" docs/specs/features.yaml`. Edit it in place; keep the `id`.
2. When the status changes, adjust `evidence` with real paths (a test name beats a file) and put the
   reason in `notes`. Verify against the code, not against another doc.
3. For a new feature, append it within its area block with the next free number and fill in `source`
   (or `"PR #NNN"` if it has no doc section).
4. Update the status summary table in `docs/FEATURES.md` if counts change.
5. Validate (from the repo root):
   ```bash
   python3 - <<'EOF'
   import yaml, os, re, collections
   d = yaml.safe_load(open('docs/specs/features.yaml')); ids = [i['id'] for i in d['items']]
   assert len(ids) == len(set(ids)), [k for k, v in collections.Counter(ids).items() if v > 1]
   bad = []
   for i in d['items']:
       ev = i.get('evidence') or []
       if i['status'] in ('done', 'partial') and not ev: bad.append((i['id'], 'no evidence'))
       for e in ev:
           p = re.sub(r':\d+(-\d+)?$', '', str(e).split('::')[0])
           if not os.path.exists(p): bad.append((i['id'], e)); continue
           if '::' in e and os.path.isfile(p) and not re.search(r'\b' + re.escape(e.split('::', 1)[1]) + r'\b', open(p).read()):
               bad.append((i['id'], e))
   print(len(ids), 'items;', collections.Counter(i['status'] for i in d['items']), '; bad:', bad)
   EOF
   ruby -ryaml -rdate -e 'YAML.load_file("docs/specs/features.yaml", permitted_classes: [Date])'
   ```
6. Bump `updated:` at the top of the file.

Snapshot on 2026-10-05 (after deep verification): 103 items — 82 done, 17 partial, 4 todo, 0 dropped.
Snapshot on 2026-10-06 (2.76.0, then the ignored-bug sweep): 105 items — 84 done, 17 partial, 4 todo, 0 dropped.
Snapshot on 2026-10-07 (stacks content-sized by default, LAY-006 done): 105 items — 85 done, 16 partial, 4 todo, 0 dropped.
Snapshot on 2026-10-07 (`revue dev` watches and restarts, DX-002 done): 105 items — 86 done, 15 partial, 4 todo, 0 dropped.
Snapshot on 2026-10-09 (SplitView holding widgets, WID-045 done): 107 items — 88 done, 15 partial, 4 todo, 0 dropped.
Snapshot on 2026-10-09 (TabView holding widgets, WID-046 done): 108 items — 89 done, 15 partial, 4 todo, 0 dropped.
Snapshot on 2026-10-09 (dock(), WID-047 done): 109 items — 90 done, 15 partial, 4 todo, 0 dropped.
