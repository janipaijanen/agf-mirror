<!-- Radar carrier. Retained verbatim from MR !1000 (source branch docs/llis-schema-learnings-deferred-todo, head ab9e36b9), file docs/TODO.md (the added lines only).
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->

## Candidate learnings: NASA LLIS comparison + learning-schema implementation audit

Status: deferred TODO, not yet recorded as `agf/learnings/foundry.yaml` entries.
Owner: coder-tester to draft, governor-reviewer to approve recording.

**Gate: do not submit any of these as real learning entries until the
learning-record schema itself is stable** — concretely, after
`docs/plans/learning-supersession-status-fields.md` (MR !995/!996) and
`docs/plans/learning-field-enforcement-audit.md` (MR !998) have both reached
Gate 1 approval and their implementations have shipped (targeted for
v2.5.0-rc.3 per current sequencing). Recording new learnings against a schema
that is itself mid-change risks writing entries in a shape the same audit is
about to revise (see candidate 4 below, which is exactly this risk applied to
the existing corpus). Revisit this gate condition if rc.3's scope changes.

Candidates identified while doing the LLIS comparison (MR !981) and the two
schema-audit plans above, verified against source but intentionally not yet
written up as full learning records:

1. **Primary-source recovery via the underlying data endpoint, not the
   rendered page.** `llis.nasa.gov/lesson/30101` renders as an empty
   JavaScript SPA; the record was only recoverable by finding its underlying
   data endpoint (`.../lesson/30101/_source`) and fetching that directly
   (contribution-auditor finding LLIS-C1). Generalizes beyond this one
   comparison: a page that renders empty is not automatically
   secondary-source-only — check for a JSON/data endpoint the page's own
   frontend consumes before downgrading a source's verification status.

2. **Comparative schema/field write-ups need field-by-field primary
   verification, not paraphrase from a first read.** LLIS-C1-R2: an earlier
   revision of the LLIS comparison misattributed NASA's `lesson`-field content
   (personnel-training, closeout-inspection guidance) to the `recommendation`
   field, caught only on a second independent review pass against the raw
   record. Same failure shape `boundary-shape-testing-rule.md` names for
   parsing code, applied here to descriptive/comparative writing instead of
   a parser.

3. **A newly-established design correction should be checked against
   already-shipped artifacts, not just future proposals.** Contribution-auditor's
   AGF-C4 finding corrected a *proposed* `related_learnings` field design
   (forward references on the new record; no edit to old record text; derived
   backlinks). Applying that same corrected pattern retroactively to the
   *already-shipped* `superseded_by` field found it has the identical flaw —
   it requires editing an already-published entry to populate, contradicting
   its own stated "never modifies older entry text" invariant
   (`docs/plans/learning-supersession-status-fields.md`, findings D1/D2).

4. **A fourth documented-but-unimplemented gate, this time in the learning
   validator itself.** `agf/schemas/invariants.yaml`'s
   `learnings.append_only_supersession` invariant claims
   `enforced_by: validate_learnings`, `severity: error`; grepping
   `agov-validators/src/learnings.rs` directly shows zero references to
   `supersed` anywhere in it — the claimed gate does not exist. This is the
   same shape `always/gate-self-test-rule.md` already documents three
   instances of (v0.25.0, v0.38.0, v0.41.0 tag/hook gates); this is a fourth,
   found not from an incident but from retroactively applying finding 3 above.
   Worth recording both as its own learning and as a citation back into
   `gate-self-test-rule.md`'s evidence base.

5. **Schema permissiveness (no `required` fields, unrestricted
   `additionalProperties`) let two mutually-unaware corpus shapes coexist
   silently for an unknown period.** `docs/plans/learning-field-enforcement-audit.md`
   finding E6: roughly half of `agf/learnings/foundry.yaml`'s 44 entries use a
   schema-undocumented field vocabulary (`summary`/`detail`/`framework_impact`)
   instead of the documented one (`title`/`description`/`business_impact`),
   and nothing detected the drift because neither shape is actually required
   or restricted. Related to the already-recorded
   `documented-tool-sync-relationships-drift-silently-without-mechanical-enforcement`
   (`20260726T121943Z-w5jj`) — same family (documented convention, no
   mechanical check, silent drift) but a different subsystem (corpus data
   shape, not tool/doc binary sync) — worth cross-referencing rather than
   treating as a fourth instance of that exact entry's finding.

Do not expand this list opportunistically before the gate above is met —
capture new candidates here as they're noticed, but the recording step waits
for schema stability as stated.
