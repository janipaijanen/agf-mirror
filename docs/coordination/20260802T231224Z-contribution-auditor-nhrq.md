---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T231224Z-contribution-auditor-nhrq
timestamp: 2026-08-02T23:12:24Z
model: codex-gpt-5
immutable: true
references-to: []
topic: schema-record-versioning-learning-review
requires-ack: true
artifacts_owned: []
---

# Contribution audit: schema/record-versioning learning

Scope: MR !986 exact head
`f7cf3ff2371c8f6e8bc93a3a1f25fa572a766ee8`; sole changed file
`agf/learnings/foundry.yaml`, new entry `20260802T230821Z-sv7q`.

Coverage: COMPLETE for the new learning's count, current-schema inspection,
release-provenance precedent, migration claim, recommendation, evidence, and
coordination path.

Recommendation: BLOCK merge until the two semantic findings below are
corrected.

Blocking findings: 2

Advisory findings: 2

Open findings with accepted risk: 0

Authority: recommendation only; governor-reviewer decides Gate 2.

Evidence: `agov validate learnings`, `agov validate infosec`, and
`agov validate contributions` pass (the contributions validator emits one
existing corpus warning unrelated to this entry); the exact range passes
`git diff --check`. Exact-head pipeline `2725655424` was running when this
review completed. Direct inspection confirms nine JSON schema files, with
`schema_version` required only inside both current record variants of
`release-provenance.schema.json`.

## B-SV-1 — the gap cohort includes the intentionally unversioned legacy schema

Check: the count and proposed remediation classify like artifacts

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-SV-1

Impact: material

Enforcement: blocking

Basis: verifiable-claims rule and contribution integrity

Finding state: open

Risk disposition: none

The literal file-presence count “8 of 9 schema files contain no
`schema_version`” is true. It is not, however, the relevant gap count:
`release-provenance-legacy.schema.json` intentionally recognizes records
whose defining characteristic is that they predate and lack
`schema_version`. Treating that legacy recognizer as one of eight schemas
that should decide whether to add a version field contradicts its purpose.

Excluding the current provenance schema and its deliberately unversioned
legacy recognizer leaves seven current non-legacy schema files without a
dedicated per-record schema-version marker: agent, coordination-message,
framework, gate1-plan, learning, rule-projection-manifest, and rule. The
framework record's `contract_version` remains correctly distinguished as
framework semver, not schema-shape versioning.

Resolution condition: distinguish the literal 8-of-9 inventory observation
from the actionable cohort. Reframe the gap/recommendation as seven current
non-legacy schemas lacking a dedicated marker; describe the eighth unversioned
file as the intentional legacy half of the precedent, not a candidate for the
same remediation. Update the title if needed so it cannot be read as eight
equivalent gaps.

## B-SV-2 — the precedent classifies versions; it does not migrate records

Check: the claimed precedent provides the capability attributed to it

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-SV-2

Impact: material

Enforcement: blocking

Basis: implementation-honesty rule and verifiable-claims rule

Finding state: open

Risk disposition: none

The learning calls the release-provenance pair a working answer to “how do
you migrate old records to a new schema shape.” The implementation explicitly
does not migrate them. `provenance::validate_record()` dispatches records with
`schema_version: "2.4"` to the current schema and records with no marker to
the legacy schema; recognized legacy records receive `assurance_level:
legacy` and are “never upgraded implicitly.” This is deterministic version
classification and validation, not record transformation.

A marker can enable deterministic dispatch, but safe migration additionally
needs retained versioned schemas or handlers, a defined transformation and
compatibility policy, tests for source-to-target conversion, and an explicit
decision about whether historical records are rewritten at all. None follows
from stamping a field alone.

Resolution condition: describe the existing precedent as deterministic
classification/dispatch across current and legacy shapes. Present migration
as a separate design problem that a future plan must specify and test; do not
claim the provenance validator already demonstrates migration.

## Advisory findings

### A-SV-1 — scope “no way to know” to intrinsic record evidence

Git history, dates, or repository releases may sometimes let an operator infer
which schema was current when a record was authored. The real gap is that the
record itself contains no dedicated, machine-readable discriminator. Replace
the absolute “no way” wording with “no intrinsic deterministic marker” or
equivalent.

### A-SV-2 — the review request was not coordinated canonically

The MR description requests contribution-auditor action, but no coordination
record was published first. Per the coordination visibility rule, future
review obligations need a validated coordination handoff before the MR note or
description bridge. This review proceeds because the obligation was visible;
the missing handoff does not invalidate the content, but should not become a
workflow precedent.

## Required next sequence

1. Correct B-SV-1 and B-SV-2, plus the two advisories, without proposing or
   implementing schema changes on this branch.
2. Publish a corrected exact head and green pipeline.
3. Publish a canonical coordination handoff requesting bounded renewed review
   of only these findings.
