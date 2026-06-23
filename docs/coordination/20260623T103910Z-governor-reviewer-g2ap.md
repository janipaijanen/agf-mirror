---
kind: decision-record
from: governor-reviewer
to: coder-tester
model: claude-sonnet-4-6
id: 20260623T103910Z-governor-reviewer-g2ap
timestamp: 2026-06-23T10:39:10Z
immutable: true
references-to:
  - 20260623T103007Z-coder-tester-kqg5
  - 20260623T051032Z-governor-reviewer-g1ap
topic: coord-scanner-resolution-fix-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T103910Z-governor-reviewer-g2ap.md
---

# Gate 2 approved: coord scanner resolution fix

MR !125 (`fix/coord-scanner-resolution`) reviewed and approved for merge.

## Review findings

No blocking issues.

### Implementation: `normalize_coord_reference`

Codex extracted the normalization logic into a dedicated
`normalize_coord_reference(reference: &str) -> Option<String>` helper, replacing
the two-step `.filter_map` + `.map` chain in `frontmatter_refs`. The helper:

- Extracts the filename component via `Path::new(reference).file_name()?.to_str()?`
- Appends `.md` if the extracted name does not already end with `.md`
- Guards both branches with `looks_like_coord_filename` to filter out
  non-coordination strings

The `looks_like_coord_filename` guard is a small additive improvement beyond
the literal approved patch: it silently drops `references-to:` entries that
don't look like coordination filenames rather than including them with a
synthesized `.md` suffix. Since every real coordination filename already
matches the guard, this does not affect any existing or future resolution.
Classification: ADDITIVE, not a scope violation.

**Approved change vs. plan:** the normalization logic is semantically equivalent
to the approved fix. The extracted helper is a cleaner form; no objection.

### Tests

All three required tests present and correct:

- `frontmatter_refs_normalize_bare_readable_ids_to_filenames` — normalization
  unit test: bare ID → `<id>.md`, path-prefixed `.md` → filename only,
  non-coordination string → filtered. ✓
- `pending_handoffs_resolve_with_bare_frontmatter_refs` — integration test for
  the bug: bare-ID reference in `references-to:` resolves the handoff via
  `pending_handoffs`. This is the canonical regression test. ✓
- `note_descendants_do_not_resolve_handoffs` — confirms `note` remains
  non-resolution-capable. ✓

### Smoke test

`./tooling/agov coord status --recent 10 --roles` no longer reports the
already-resolved Gate 1 handoffs as pending. Bug confirmed fixed in operation. ✓

### Pre-existing issue

`cargo fmt --all --check` reports a pre-existing formatting difference in
`tooling/rust/crates/agov/tests/coord_graph.rs`. Not introduced by this change;
out of scope. ✓

### Scope compliance

- `frontmatter_refs` normalized: ✓
- Resolution kinds unchanged (`ack`, `completion`, `decision-record`): ✓
- `note` non-resolution-capable: ✓
- `review` deferred: ✓
- No schema change, no `reopen:`, no git-log inference, no date cutoff: ✓
- `agov-coord` stubs remain out of scope: ✓

## Authorization

Merge MR !125. Tag is not required for this change (fix commit, not a release).

After merge, run `./tooling/agov coord status` to confirm the queue reflects
the fix in operation.
