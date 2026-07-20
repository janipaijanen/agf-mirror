# docs: state append-only audit trail policy vs GDPR erasure

Branch: `docs/append-only-audit-trail-policy`

## Summary

States, for the first time, an explicit policy that this repository's git
history and coordination channel are append-only by design — closer to a
ledger than a working document — and names the GDPR Art. 17 tension that
creates for any project storing author identities in that trail.

## Why

`docs/sources.md` (Batch 3, filed this session) flagged this as an
unresolved amendment: "resolve GDPR erasure vs append-only tension before
publishing the exit guarantee." This MR resolves it with a policy
statement rather than leaving it implicit.

## Files changed

| File | Change |
|---|---|
| `README.md` | New "Git history is a ledger, not a scratchpad" section — developer-facing, states this repo's own policy |
| `agf/docs/site/contribute.md` | New "Audit trail policy: append-only, not erasable" section — adopter/contributor-facing, states the framework-level design stance; points to identity minimisation + documented lawful basis as the upstream fix, not history rewriting |

## Evidence status

Policy statement, not a capability claim — no maturity level applies.
Both sections defer to `disclaimer.md` for the "not legal advice" boundary.

## Review checklist

- [ ] Both sections read correctly in isolation
- [ ] No duplicate/contradictory claim vs. existing `disclaimer.md` /
      `governance.md` append-only language
- [ ] No `Co-authored-by` trailers in commit
