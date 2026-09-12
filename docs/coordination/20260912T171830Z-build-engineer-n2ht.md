---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260912T171830Z-build-engineer-n2ht
timestamp: 2026-09-12T17:18:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T170714Z-coder-tester-k6d4
  - 20260912T155648Z-v2101-ruw-generic-artifact-validation
requires-ack: true
topic: v2101-artifact-candidate-build-review
artifacts_owned: []
---

# Build-engineer review: v2.10.1 A1+A3+A4-structural candidate, exact head f74fe840

Per `k6d4`'s handoff, reviewed `codex/v2101-artifact-validation` at exact
head `f74fe84`0db0d181a089193769a92a2d073aeb5d7`, based on rewritten
`main` `57b178b4`. Scope check first: diff against `main` touches only
`agf/schemas/artifact-config.schema.json` (new, this feature's own
schema), docs, plan/coordination records, and the Rust crates
(`agov-validators`, `agov`, `agov-mcp`) plus their tests/fixtures. No
`.gitlab-ci.yml`, no `agf/agents/*.yaml`, no existing coordination
schema touched — matches the plan's approved boundary exactly.

**Independent verification (not trusting Codex's self-report alone):**
built and ran the candidate myself in an isolated worktree
(`.worktrees/v2101-review`, removed after review, per this project's
worktree-location rule).

- `cargo test --release -p agov --test artifact`: 5/5 passed
  (deterministic-golden-output, missing/invalid-config split, all
  checked-in negative fixtures, cross-file duplicate detection,
  symlinked config/schema/artifact rejection through the CLI).
- `cargo test --release -p agov-validators artifact`: 12/12 passed
  (config validation including RFC 6901 pointer syntax and lexical
  traversal rejection; file discovery including the symlink and
  oversize-input tests; index-building including duplicate/dangling/
  cycle detection, external-`$ref` rejection, and byte-deterministic
  repeated output).
- `cargo clippy --release -p agov -p agov-validators -p agov-mcp
  --all-targets -- -D warnings`: clean.

All figures match Codex's self-reported counts exactly.

## Containment/security assessment (the review focus `k6d4` named)

Read `artifact_files.rs`, `artifact_config.rs`, and `artifact_index.rs`
directly rather than trusting test names:

- **Path syntax filter** (`contained_relative_path`): rejects empty
  segments, `.`/`..`, backslashes, colons (Windows drive-letter/ADS
  confusion), null bytes, and control characters, before any
  filesystem access. Good first-line defense.
- **Symlink rejection**: checked component-by-component via
  `fs::symlink_metadata` during path resolution (`checked()`) — not
  just the final component — and independently during glob directory
  traversal (`expand()`) via `entry.file_type().is_symlink()`. A
  concrete test creates both a direct symlinked file and a symlinked
  intermediate directory pointing outside the fixture root and
  confirms both `read()` and `discover()` reject them.
- **Path-escape containment**: after component-wise symlink rejection,
  `path.canonicalize()...starts_with(&self.root)` is a defense-in-depth
  check, not the primary control — by the time it runs, no symlink
  should have been traversable, so this mainly guards against
  canonicalization surprises rather than being the sole containment
  mechanism.
- **External `$ref` rejection**: `compile_local_schema` recursively
  walks the entire schema JSON tree and rejects any `$ref` not starting
  with `#` (fragment-only). Test uses a concrete
  `{"$ref":"file:///etc/passwd"}` payload and confirms rejection —
  this is a real adversarial-shaped test, not just a placeholder.
- **Size cap**: 16 MiB enforced both via pre-read metadata check and
  post-read `take(MAX_FILE_BYTES + 1)` sentinel, so a file that grows
  between checks still can't silently exceed the cap.
- **Glob traversal bound**: `MAX_VISITS = 100_000` state cap prevents
  unbounded work on pathological patterns.

**On the TOCTOU point `k6d4` explicitly flagged:** confirmed accurate,
not overclaimed. `read()` re-checks path validity and compares
before/after size and mtime, which detects (not prevents) a file
changed during the read window — a real but narrow gap remains between
`checked()`'s path validation and `File::open()`'s actual open, where a
sufficiently well-timed concurrent filesystem mutation could swap a
path component. This is a correct, honest disclosure of a real
limitation rather than a false claim of atomicity, and matches the
docs' explicit "stable working tree" scope limitation. I agree with
Codex's own framing: assess this as an accepted, disclosed limitation
against the stated threat model (a stable local repository, not an
actively hostile concurrent filesystem mutator), not as a defect to
block on.

## Verdict and remaining requirement

**Build-engineer review: CLEAR.** No blocking finding. Independently
verified tests, clippy, and the specific containment claims the plan
required scrutiny of.

**This is not the "real containment security review" the plan and
`k6d4` require before Gate 2.** Per `AGENTS.md`'s own role table,
`infosec-sentinel` in this project is CI-mechanical only ("Mechanically
blocks configured checks on credential or cross-record leaks in
learnings") — not equipped for a design-level security assessment of
new filesystem-facing code. The established precedent for this class of
review is the same one used for CP-B-1/CP-B-2: a named, non-author
independent reviewer appointed by governor decision (the Beauvoir
appointment, `20260912T095349Z-governor-reviewer-smmp`). Requesting the
same here before Gate 2 — an appointment decision, not something I can
supply myself as the implementing-adjacent build reviewer.

No Gate 2 readiness, merge, or release authority is claimed by this
review.
