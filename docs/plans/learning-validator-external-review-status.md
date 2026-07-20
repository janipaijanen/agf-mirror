---
id: learning-validator-external-review-status
status: complete
topic: learning-validator-self-sufficiency
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: determination note only; no code change proposed
  - role: contribution-auditor
    status: not-applicable
    reason: determination note only; no code change proposed
---

# Determination: issue #19 (learning validator self-sufficiency) is already resolved

**Date:** 2026-07-20T07:55:03Z
**Status:** No implementation plan needed — recording why.

## The issue as filed

An adopter preparing an upstream-submittable learning record found that
`agov validate learnings` only validated the canonical adopter register
under `agf/learnings/`, and that direct Python-based schema validation
failed locally due to a missing YAML parser module, requiring a Ruby
fallback. The recommendation was: either make `agov` self-sufficient for
validating a named learning file outside `agf/learnings/`, or clearly
document the external prerequisites, and distinguish canonical adopter
registers from upstream-submittable packets in the docs.

## What was checked before concluding this needs no new plan

1. **Does `docs/external-reviews/` exist as a real directory in this repo?**
   No (`ls docs/external-reviews` → no such file or directory). It appears
   only as an *example path* in documentation, not as a directory this repo
   or the tooling assumes exists. This matches the task instruction to
   verify the real structure rather than plan against a directory that
   doesn't exist.

2. **Does `agov validate learnings` already accept an arbitrary path?**
   Yes. `tooling/rust/crates/agov-validators/src/learnings.rs`
   (`validate_paths`, and the CLI wiring in
   `tooling/rust/crates/agov/src/main.rs`, `ValidateCommand::Learnings {
   paths: Vec<PathBuf> }`) already accepts zero or more explicit file or
   directory paths, defaulting to `AGOV_LEARNINGS_DIR` / `agf/learnings`
   only when no path is given. This is the Rust `agov` binary — no Python
   package, no Ruby script, no separate JSON-schema tool is required; it is
   the same self-sufficient binary already used for every other validator
   in this repo.

3. **Is the canonical-vs-upstream-submittable distinction documented?**
   Yes, in three places, already present on `main`:
   - `agf/docs/site/docs/advanced/contracts-and-validators.md`, section
     "Validating upstream learning submissions" — states the default-vs-explicit-path
     behavior and explicitly says "no Python package, Ruby script, or
     separate JSON-schema tooling is required."
   - `agf/docs/site/reference/environment-variables.md` — documents
     `AGOV_LEARNINGS_DIR` and recommends `agov validate learnings
     <path-to-learning.yaml>` for one-off packets instead of changing the
     environment variable.
   - `agf/docs/site/docs/start/adopter-faq.md`, "How do we push a learning
     upstream?" — step 2 explicitly instructs running `agov validate
     learnings <path-to-learning.yaml>` for a packet stored outside the
     canonical register.

4. **When was this fixed?** `git log` shows commit `98e9fcf` ("fix: validate
   upstream learning packets", 2026-06-25) implemented the path-accepting
   behavior, and commit `22e9e24` ("docs: harden external review adoption
   guidance", same day) added the documentation above. Both predate this
   session (today is 2026-07-20) and are already on `main`.

## Conclusion

Both remedies the issue recommended — tooling self-sufficiency and clear
documentation of the canonical-vs-upstream-submittable distinction — were
already implemented approximately four weeks before this planning pass,
as part of the Rust `agov` port. This issue is stale relative to current
`main`.

## Recommendation

Close issue #19 with a comment referencing commits `98e9fcf` and `22e9e24`
and the three documentation locations above, rather than opening a new
implementation branch. No code or doc changes are proposed by this note.

If a reviewer wants an even lighter-weight confirmation that the fix truly
resolves the reported symptom, the exact repro from the original report can
be re-run: `agov validate learnings <any-single-file.yaml>` outside
`agf/learnings/`, and confirm it succeeds without any Python or Ruby
dependency — this was spot-checked by reading the Rust source directly
(`validate_paths` in `learnings.rs`) rather than by re-running the
adopter's original broken environment, which no longer applies now that
validation runs through the Rust binary.
