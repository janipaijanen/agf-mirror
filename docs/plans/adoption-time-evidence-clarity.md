---
id: adoption-time-evidence-clarity
status: approved
topic: adoption-time-evidence-clarity
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-21T13:55:01Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Public documentation evidence correction only; no credential,
      access-control, runtime, schema, or security-boundary change.
  - role: contribution-auditor
    status: consulted
    reason: Critical review is routed in coordination record
      20260721T135501Z-planner-architect-36db before implementation; advisory
      feedback may refine wording but cannot broaden the approved scope.
---

# Plan: state adoption time at the strength of the evidence

## Trigger and approved direction

The public home currently says both “an afternoon to install” and
“twenty-minute path.” The governor-reviewer identified that this apparent
error margin damages adoption credibility and supplied direct evidence: in
projects where the maintainer has deployed AGF, mechanical installation has
taken no more than ten minutes.

The governor accepted the proposed evidence boundary and directed the work to
proceed. The companion decision record
`20260721T135501Z-governor-reviewer-1hph` records that Gate 1 authority.

## Canonical message

Documentation will distinguish three different quantities:

1. **Mechanical installation:** no more than ten minutes in the maintainer's
   deployments. This is empirically informed maintainer evidence, not yet
   independent-adopter validation.
2. **Initial commissioning:** reviewing applicable rules and completing the
   first governed change; duration varies by project and is not assigned an
   unsupported numeric estimate.
3. **Per-change governor attention:** existing ranges are planning estimates
   informed by maintainer-operated use, not service-level promises or
   independently validated benchmarks.
4. **Exit effort:** removal is one atomic cleanup commit. Duration has not yet
   been measured and varies with CI, hooks, and project-owned evidence. No
   numerical exit-time claim is made until a real decommission is measured
   from a defined start to a verified clean state.

“Twenty-minute path” becomes “short evaluation-and-install path.” “An
afternoon to install” is removed rather than softened or relocated.

## Affected files

- `README.md`
- `agf/docs/site/index.md`
- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/start/adopter-faq.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`
- `agf/docs/site/docs/adopt/exit-and-decommission.md`
- `agf/CHANGELOG.md`
- coordination records for review and Gate 2 handoff

Search all public Markdown for the superseded phrases and update any additional
current claim surface found. Historical plans, reviews, and immutable
coordination evidence are not rewritten.

## Release boundary

This is an independent documentation evidence correction, not silent expansion
of the frozen v2.3.1 recovery plan. Its merge/release ordering must preserve
that boundary unless the governor-reviewer separately amends v2.3.1 scope.

## Verification

1. Search current public documentation for `afternoon`, `twenty-minute`,
   `5-15 minutes`, and equivalent time claims; every retained claim must carry
   the correct evidence boundary.
2. Confirm the exit page states the unmeasured-duration boundary and does not
   imply that “one atomic removal commit” is a time estimate.
3. Confirm README and site home use the same canonical distinction.
4. Run `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`.
5. Run `agov validate adopter-docs` and `agov dist validate`.
6. Run `git diff --check`.

## Risks and controls

| Risk | Control |
|---|---|
| Maintainer deployments are presented as independent evidence | State the population explicitly and say independent validation is pending. |
| Ten minutes is misread as full commissioning | Name mechanical installation and commissioning separately wherever the number appears. |
| Existing governor-time estimates imply benchmark precision | Label them maintainer-informed planning estimates and retain the high-risk exception. |
| A one-commit exit is misread as a ten-minute exit | State that duration is unmeasured and name the sources of variability. |
| Copy drifts across entry pages | Use one canonical message and verify all current time-claim surfaces. |

## Rollback

Revert the documentation paragraphs and changelog entry. No schema, tooling,
runtime, or adopter data changes are involved.
