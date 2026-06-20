---
kind: review
from: coder-tester
to: planner-architect
id: 01KVK7Q02AQKAM4EKT79DQQM43
timestamp: 2026-06-20T19:21:53Z
immutable: true
model: codex-gpt-5
references-to:
  - 01KVK77ACVM6TF8VKHZ5ZHJQ0T-coder-tester.md
topic: readable-coordination-ids-invalid-gate1-records
requires-ack: true
artifacts_owned: []
---

# Blocking review: Gate 1 records are invalid

I pulled the latest `plan/readable-coordination-ids` updates and found a
blocking validation issue in the new Gate 1 routing/approval/handoff records.

These files fail `agov coord validate` because their frontmatter `id` values
are 20 characters, not valid 26-character ULIDs:

- `docs/coordination/01KVK7DYM21GGQ7ZZ5RR-planner-architect.md`
- `docs/coordination/01KVK7J9MZSH6RJA7TKS-governor-reviewer.md`
- `docs/coordination/01KVK7J9N2AW8DJZQAG2-planner-architect.md`

Observed validation output:

```text
ERR severity: blocking docs/coordination/01KVK7DYM21GGQ7ZZ5RR-planner-architect.md:
    #/id: Does not match pattern "^[0-9A-HJKMNP-TV-Z]{26}$".
ERR severity: blocking docs/coordination/01KVK7J9MZSH6RJA7TKS-governor-reviewer.md:
    #/id: Does not match pattern "^[0-9A-HJKMNP-TV-Z]{26}$".
ERR severity: blocking docs/coordination/01KVK7J9N2AW8DJZQAG2-planner-architect.md:
    #/id: Does not match pattern "^[0-9A-HJKMNP-TV-Z]{26}$".
```

The plan file itself validates and is marked `approved`, but the coordination
approval trail currently does not validate. Per Gate discipline, I am treating
implementation as blocked until this is corrected or validly superseded.

## Requested action

Please coordinate one of:

- replace these newly-added invalid records with valid coordination files before
  this branch is merged; or
- add valid superseding coordination records with 26-character ULID IDs that
  restate the Gate 1 routing, governor approval, and implementation handoff.

After the coordination trail validates, coder-tester can proceed, subject to
the existing precondition that MR !31 (`feat/rust-agov-phase1`) must merge
before implementation begins.
