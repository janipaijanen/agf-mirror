# Quick reference

One page. Roles, pipeline, coordination kinds, and the rules you will
reference most. For detail, follow the links.

## Agent roles

**Core (Phase 1)** — the governed pipeline:

| Role | One-line responsibility |
|---|---|
| planner-architect | Decomposes proposals into scoped, sequenced work items; owns plans and Gate 1 coherence |
| coder-tester | Implements approved changes and their tests; never approves its own work |
| governor-reviewer | Enforces boundaries, arbitrates disputes, approves Gate 1 plans and Gate 2 merges |
| contribution-auditor | Audits contributions (learnings, rules, agent definitions) for schema and policy compliance |
| infosec-sentinel | Screens externally-bound artifacts for leaks; gates anonymization at the codebase boundary |
| fact-checker | Verifies claims in documentation and changelogs against the code and evidence base |
| build-engineer | Owns CI configuration, build automation, and pipeline efficiency |
| academic-peer-reviewer | Reviews long-form/academic artifacts for methodological rigor |
| academic-visual-reviewer | Reviews visual quality, layout, and figure legibility |
| web-ux-reviewer | Reviews rendered web/documentation-site UX, layout, theming, and accessibility, independent of the implementer |
| independent-implementation-reviewer | Read-only exact-head review of an implementation against its approved plan and evidence, by a session/model distinct from the implementer; advisory, never a Gate 1 or Gate 2 approval |
| documentation-estate-steward | Owns consistency and currency of the adopter-facing documentation estate as a whole; runs estate-wide sweeps and dispositions in-scope findings |

**Extended (Phase 3-4)** — activate when the project needs them:

| Role | One-line responsibility |
|---|---|
| deployment-engineer (P3) | Translates source artifacts into deployable releases |
| documentation-writer (P3) | Writes audience-specific guides, tutorials, and reference docs |
| capability-developer (P4) | Analyzes accumulated evidence and proposes framework capability changes |
| improvement-propeller (P4, meta, optional) | Bounded weekly opportunity-scanning and slow governance-friction proposals; activated per-adopter via a governor decision naming player, evidence, wake mechanism, cadence, and cursor state |

A release archive ships `agents/MANIFEST.example.yaml`, a neutral template of
active/skipped role status — copy it to `agents/MANIFEST.yaml` (never itself
packaged in the archive) and edit before use; see
[manifest.md](../governance/manifest.md).

## Minimum commissioning

| Trial shape | Active roles | Use when |
|---|---|---|
| Solo trial | coder-tester, governor-reviewer | One human wants to test whether gates help |
| Standard governed project | planner-architect, coder-tester, governor-reviewer | A team wants plans before non-trivial implementation |
| CI/security-conscious project | Standard roles plus contribution-auditor, infosec-sentinel, fact-checker as needed | Security, compliance, or external evidence matters |
| Independent-review project | Standard or CI roles plus independent-implementation-reviewer, named per non-trivial Gate 2 candidate | Cross-model or cross-session implementation review evidence is needed before Gate 2 |

External gateways and agent runtimes are execution surfaces. They do not become
AGF approval authorities unless the governor records that role decision.

## Pipeline

```
proposal → Gate 1 (plan approval) → implement + test → Gate 2 (review) → merge → release
```

- **Gate 1** — governor-reviewer approves the plan before implementation
  starts. Output: a decision-record in the coordination channel.
- **Gate 2** — governor-reviewer reviews the implementation before merge.
  Always applies, even when Gate 1 was waived.
- **Fast track** — Gate 1 only may be waived for small tasks
  (≤ 3 files, no shared interface / data model / security boundary change,
  no claim conflict, no scope uncertainty). Any disqualifier returns the
  task to the full pipeline. See framework.md Pattern 6 for the criteria
  table.

## Coordination channel kinds

Append-only messages in `docs/coordination/`. Prefer
`agov coord brief --id-format readable --from <role>` for new entries and save
readable-ID messages as `<id>.md`; historical sequence and ULID filenames
remain valid:

| Kind | Meaning |
|---|---|
| `handoff` | Pass work to another agent with task, must-read, and do-not-touch lists |
| `decision-record` | A gate decision: approval, rejection, or condition |
| `completion` | Close a task; records what merged, where, and how verified |
| `note` | Status update, acknowledgment, or interim finding |
| `review` | Findings filed against a gate submission |
| `ack` | Explicit acknowledgment when one was required |
| `claim` | Claim ownership of files before editing to prevent collisions |
| `fast-track-waiver` | Record of a granted Gate 1 waiver and its criteria check |
| `lesson-proposal` | Proposed learning record for the evidence base |

## Five most-referenced rules

1. **merge-request-workflow** — every change to a protected branch arrives
   via a reviewed merge request; no direct pushes.
2. **tdd** — no production code without a failing test first;
   RED → GREEN → REFACTOR.
3. **immutable-tags** — a pushed tag is never moved or deleted; fix forward
   with a new version.
4. **systematic-code-review** — the full pre-commit checklist runs before
   every proposed commit; BLOCK items stop the commit.
5. **uncertainty-and-scope** — more than two modules means a plan file
   first; conflicts are surfaced, not silently resolved.

The full rule set lives in [rules/](../../rules/).

## See also

- [framework.md](../../framework.md) — principles and patterns (the theory)
- [ADOPTING.md](../../ADOPTING.md) — step-by-step adoption
- [templates](../../templates/) — copyable first-run files
- [docs/governance/manifest.md](../governance/manifest.md) — what ships in the distribution
