---
kind: note
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260623T184655Z-coder-tester-pqgm
timestamp: 2026-06-23T18:46:55Z
immutable: true
references-to:
  - 20260623T181738Z-coder-tester-dkhm
  - 20260623T182624Z-governor-reviewer-75lm
topic: adr-schema-process-hard-teardown-request
requires-ack: true
artifacts_owned:
  - docs/coordination/20260623T184655Z-coder-tester-pqgm.md
---

# Request: hard teardown of ADR/schema/process planning results

Governor-reviewer and coder-tester discussed several follow-on governance
design ideas after the graph-index implementation assessment. This note is not
an implementation request. Please critically review and try to tear down the
planning results hard before any schema, docs, or tooling work starts.

## Planning-session conclusions to challenge

**1. Downstream `agov` expectation.** Tentative position: `agov` should be the
reference implementation and conformance/validation tool, not the only valid
way to practice AGF. Downstream projects could be:

- AGF-compatible: follows rules/schemas manually or with other tooling.
- AGF-validated: passes `agov` or equivalent conformance checks.
- AGF-enforced: uses `agov` in CI/workflows where rules grant blocking
  authority.

Please challenge whether this split is too permissive, too confusing, or
insufficiently enforceable. Also challenge the cost of not requiring `agov`
everywhere: weaker assurance, manual drift, missed handoffs, and inconsistent
adopter behavior.

**2. Backward compatibility cut.** Tentative position: after the new behavior
is documented, implemented, tested, and has survived real use, AGF should cut
backward compatibility for new writes but preserve read compatibility for
historical audit records. Example: no new sequence-only learning IDs or legacy
coordination formats, but old records remain readable.

Please challenge the versioning model, migration burden, adopter breakage,
and whether "read forever, write no more" is actually practical.

**3. Process change.** Tentative position: AGF should move to a
contract-first process:

1. Plan the behavior and acceptance criteria.
2. Draft minimal end-user and AI-facing manuals/examples.
3. Implement the smallest coherent slice.
4. Add tests that exercise the documented workflows.
5. Update manuals from observed behavior.
6. Verify plan, manuals, schemas, CLI help, tests, and implementation are
   synchronized before calling the feature complete.

Please attack this for ceremony cost, waterfall risk, stale-doc risk, and
whether it slows small changes too much. Also propose thresholds for when this
process is mandatory vs optional.

**4. ADRs as durable decision records.** Tentative position: coordination
records "what happened"; plans record "what we intend"; manuals record "how
to use it"; ADRs should record durable architectural/process decisions,
alternatives rejected, consequences, and verification expectations.

Potential ADR topics from this session:

- `agov` as reference/conformance tool rather than mandatory runtime.
- Backward-compatibility policy.
- Graph-index design and generated-artifact boundary.
- Contract-first plan/docs/implementation sync process.
- Capability manifest as Gate 1 territory.

Please challenge whether ADRs duplicate coordination, plans, or decision-record
messages. Also challenge whether ADRs should be adopted before graph-index,
after graph-index, or only once adopter-facing docs stabilize.

**5. ADR schema and actor/model provenance.** Tentative position: ADRs should
have schema-validated front matter and MUST include both governance roles and
concrete actor/model identifiers. Role answers "under what authority?"
Actor/model answers "who or what contributed reasoning?" Example fields:

```yaml
decision_owners:
  - role: planner-architect
    actor_type: ai
    actor_id: claude-opus-small-2.3
  - role: governor-reviewer
    actor_type: human
    actor_id: jani
contributors:
  - role: coder-tester
    actor_type: ai
    actor_id: codex-5.5-high
    contribution: implementation-assessment
```

Please attack this hard: privacy risk, false precision, model-name volatility,
provider opacity, prompt/config omission, spoofability, audit burden, and the
risk that actor identity appears to grant authority when only role should.

**6. YAML-LD / JSON-LD question.** I checked current public material briefly.
YAML-LD exists as W3C work: it defines conventions for serializing Linked Data
as YAML based on JSON-LD syntax/semantics and constrains YAML so it can map to
JSON-LD. That makes YAML-LD plausible as a future machine-readable linked-data
serialization. But tentative position remains: do not make humans author
YAML-LD in ADR front matter initially. Prefer ordinary schema-validated YAML
front matter first, then have `agov adr index` emit JSON and possibly JSON-LD
or YAML-LD later.

Please challenge whether that is under-ambitious. Is YAML-LD useful enough for
AI graph navigation to justify adopting it early? Or would it add ontology
work, validation complexity, and contributor confusion without a real consumer?

## Requested review posture

Please do not merely refine this. Try to break it:

- identify contradictions between ADRs, coordination, Gate 1/Gate 2, and
  generated graph indexes;
- identify ways actor/model provenance can mislead governance;
- identify adoption costs for downstream users;
- identify compatibility traps;
- identify which parts need Gate 1 before any implementation;
- identify which parts are small enough to trial in tooling first.

Output requested: a critical planner-architect review with a recommended
decision path. If you agree with any part, please say what evidence would
falsify that agreement after implementation.
