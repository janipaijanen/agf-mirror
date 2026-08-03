---
artifacts_owned: []
from: planner-architect
id: 20260803T204257Z-planner-architect-sjqm
immutable: true
kind: review
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-08-03T20:42:57Z
to: all
topic: v250-rc1-three-track-external-evaluation-consolidated
---

# Consolidated three-track external AI evaluation — v2.5.0-rc.1

**Target:** tag `v2.5.0-rc.1`, commit `a3c7ce48d6b56d3395bf1eb21c09222787c51c3f`,
tag object `623e782b22b5e17e277c8a50ba6d79bedacfe3e6`. Method per the
dossier's own instructions: three bounded tracks (documentation,
artifact/tooling, adversarial governance), reviewer receives the
published release rather than `main`, exact commands recorded, no
repository changes from the review itself.

**Sources reconciled here:** Grok (Track 1, two independent Track 2
passes, Track 3), Lumo (Tracks 1-3, documentation-only, explicitly
labelled as such), and this session (a direct fact-check pass plus two
backgrounded subagents that actually executed Track 2 and Track 3 in
isolated sandboxes against the real published release, with the most
surprising external claims independently re-reproduced by hand
afterward). All sandbox work happened under `/tmp` in this session's
scratch area, never touched this working checkout, and nothing was
pushed to any remote from within a sandbox.

## Track 1 — Documentation review

Grok's pass was sound and closed its own most severe finding:
"release-identity unconfirmed" (tag vs `agf/VERSION` mismatch) was
real at crawl time but is **stale** — independently reconfirmed here
that tag, `agf/VERSION`, and the docs-build commit all agree exactly.

Lumo's pass surfaced a reviewer-reliability concern, not a
documentation defect: it asserted specific technical facts that do not
exist in the repository — a schema architecture
(`agent-capabilities.schema.json`, `permissions.schema.json`,
`policy-constraints.schema.json`, plus pluralized near-misses of two
real filenames) and CLI syntax (`agov apply --policy`, `agov validate
--schema`) that were fabricated, not observed, plus "governance-policy
endpoint" language suggesting partial conflation with an unrelated
hosted-service product despite Lumo's own non-identity disclaimer.
Verified directly against the actual downloaded `v2.5.0-rc.1` archive:
none of Lumo's named schema files exist; the real CLI has no `apply`
subcommand at all. Recommendation: weight Lumo's Track 1 findings
accordingly, and note for future external-review coordination that
documentation-only conclusions from models without execution access
need this kind of spot-check before being treated as evidence, per the
dossier's own instruction that such conclusions "must not be treated
as validation of AGF functionality."

Lumo's softer findings (terminology density on first scan, no
concrete "minimal viable activation" example for a solo adopter) are
plausible UX observations independent of the fabrication issue and
worth keeping.

## Track 2 — Artifact and tooling review: convergent, high confidence

Three independent real executions (this session's subagent, Grok pass
1, Grok pass 2) against the actual published release assets, with no
contradictions on any load-bearing claim:

- Release identity/provenance chain (tag -> commit -> `agf/VERSION` ->
  checksums -> provenance JSON -> binary self-report) verified intact
  by all three.
- OCI image identity (T2-07) is now closed by two independent methods
  that agree exactly: this session's subagent pulled the real image
  via podman and matched the manifest digest
  `sha256:698ce2d35fdbf71bc77c38f7e0bde377da1fdf7dc8792bbaf3fbeda5902537b4`;
  separately, with neither docker nor podman available in its own
  sandbox, Grok queried the GitLab Container Registry HTTP API
  directly (anonymous pull token, manifest-by-tag, config blob) and
  got the identical `Docker-Content-Digest`, plus confirmed the config
  blob's `org.opencontainers.image.version` (`2.5.0-rc.1`) and
  `.revision` (`a3c7ce48d6b56d3395bf1eb21c09222787c51c3f`) labels agree
  with the provenance record's `labels.version`/`labels.revision` and
  with the release commit under review. Two different verification
  paths, same artifact, same answer. Neither method executed the
  image's entrypoint; that remains open if anyone wants it.
- Lumo's fabricated schema/CLI claims are recontradicted here from a
  real downloaded artifact, not just a clone — same real filenames and
  CLI surface both other passes independently confirm.
- `agov validate` / `agov coord validate` behave correctly on positive
  (real templates, real live coordination files) and negative
  (deliberately malformed fixtures) inputs, matching the documented
  exit-code contract (0 / 1 / 2).
- MCP server: real JSON-RPC handshake, exactly the 10 documented tools,
  and Grok's deeper pass adds a precise detail this session's run
  didn't note explicitly — the wire framing is newline-delimited JSON,
  not Content-Length-prefixed, which the MCP docs don't currently
  state (low-severity doc-precision gap, T2-09 below).
- Archive overlay / upgrade-safety (issue #79): downloaded real
  v2.4.11 and v2.5.0-rc.1 archives, simulated an adopter's customized
  `MANIFEST.yaml`, overlaid the new release on top — customization
  survived byte-identical, `MANIFEST.example.yaml` appeared, role
  `framework_version` fields updated correctly. Matches the
  CHANGELOG's own description of issue #79 exactly.

**One apparent cross-reviewer discrepancy, resolved by direct
reproduction, not a contradiction:** `agov plan validate` on the
shipped `gate1-plan.md` sample template returns `OK` when no
`MANIFEST.yaml` exists in the workspace, and `ERR` (advisory role not
active: `contribution-auditor`, `infosec-sentinel`) when `MANIFEST.yaml`
is a copy of `MANIFEST.example.yaml` with all roles still `skipped` —
reproduced both conditions directly with the real release binary.
**This state-dependency is itself worth flagging**: a workspace with no
`MANIFEST.yaml` at all gets a clean `OK` that looks identical to a
fully-verified pass, but the advisory-role cross-check silently never
ran. Nothing in the output distinguishes "checked and clean" from
"check not applicable." Recommend either a distinct advisory note when
`MANIFEST.yaml` is absent, or documenting the state-dependency plainly
in the tooling-conformance page.

**Low-severity items from Grok's deeper pass, not independently
disputed:**
- T2-08: archive member count (238 via `tar -tz`) vs `agov dist
  validate` file count (220) — plausibly file-vs-manifest-entry
  counting, but undocumented; worth a one-line clarification for
  reviewers doing a naive count comparison.
- T2-09: MCP NDJSON wire framing undocumented (see above).

## Track 3 — Adversarial governance review: real, convergent, actionable

Two independently-run adversarial passes (this session's subagent,
Grok) tested different layers and converge on the same structural
verdict via different specific attacks. Four findings below were
independently reproduced a second time, by hand, in this session,
after the fact — commands available on request, sandboxes were
ephemeral `/tmp` state.

**High severity:**

1. **Gate 1 approval has no identity or non-emptiness enforcement.**
   `status: approved` validates cleanly with `gate1_approved_by: ""`
   (empty) and separately with `gate1_approved_by: "coder-tester"` (an
   AI implementation role, not the human governor role) — both
   independently reproduced against the real `gate1-plan.schema.json`
   in this session. The schema enforces nothing about who, or whether
   anyone, approved a plan marked approved.
2. **Coordination-record authorship (`from:`) is unauthenticated free
   text.** A forged `kind: decision-record` with `from:
   governor-reviewer` falsely approving a fictitious Gate 2 validated
   cleanly in both adversarial passes and was indistinguishable from a
   real approval in `agov coord status`. No signature, no MANIFEST
   binding on write.
3. **Mutated historical records are served as authentic.** Edited an
   existing handoff's task text in place, leaving `id`, `timestamp`,
   and `immutable: true` untouched. `agov coord validate` still
   returned `OK`; `agov handoff next` served the mutated text live as
   the current authoritative task. Independently reproduced in this
   session: nothing hashes or checksums body content against what was
   originally recorded at that timestamp, so `immutable: true` is an
   asserted label, not an enforced guarantee.
4. **Gate 1/Gate 2 have zero local technical enforcement.** No
   `agov`-related git hooks exist anywhere in the tracked repo. A
   branch with no plan record and no Gate 2 approval merged instantly
   in a sandbox with no CI attached — `git merge --no-ff`, no
   resistance. This shows the observed technical merge backstop for
   this project is hosted pipeline/branch-protection configuration
   (`coord publication-check`, `only_allow_merge_if_pipeline_succeeds`
   in `.gitlab-ci.yml` and GitLab project settings) — but that
   backstop does not itself authenticate a governor, require a Gate 1
   plan or Gate 2 decision to exist for every substantive MR, or prove
   a validated approval record is binding. Human procedural authority
   (who is entitled to approve a gate), the hosted merge/pipeline
   controls (what technically blocks a merge), and the checks those
   pipelines actually run (shape validation, not identity or
   authority) are three different things; this finding is about the
   second and third, and does not reduce or restate the first. This
   reproduces, as fact rather than speculation, the concern Lumo
   raised only theoretically ("if CI is disabled or bypassed, gates
   become advisory only") — for the sandbox condition tested, which
   had no CI attached at all, not for this project's actual hosted
   configuration.

**Medium severity:**

5. **Semantic collisions are not detected.** Two records with distinct
   valid IDs, same topic/artifact path, contradictory outcomes (one
   approves, one blocks; or two active claims on the same file path)
   both validate individually and sit side-by-side with no conflict
   flag in `agov coord status`. Literal duplicate-ID collisions **are**
   correctly blocked (`ERR ... duplicate coordination id`, exit 1) —
   a real structural gate that works, confirmed by this session.
6. **`to: all` handoffs are silently invisible to the handoff queue.**
   `agov handoff next <role>` only ever resolves handoffs with an
   exact `to: <role>` match, confirmed with a clean control fixture
   (not just an adversarial one) in this session's subagent run — a
   likely bug, distinct from the separately-tested ambiguous
   comma-joined multi-role `to:` string, which is merely accepted
   without warning rather than silently dropped.

**Low / as-expected (not defects):**

7. Dangling `references-to` and invalid `next_agent` role names are
   not validated.
8. Prompt injection via governed rule-file content and coordination
   message bodies is, correctly, out of scope for a schema validator —
   both adversarial passes confirmed the tooling has no semantic
   content awareness here, and neither pass's agent complied with any
   embedded instruction in the fixtures it authored. This is
   structurally the right place to draw the line for a
   schema-validation tool; the mitigation is human/AI review before a
   file is merged, which should be stated plainly rather than left
   implicit.

**Confirmed working (positive controls):** `immutable: false` is
rejected (`const: true` enforced); handoff `status: complete` is
rejected pre-Gate-2; a `completion` record cannot carry non-empty
`artifacts_owned`; duplicate coordination IDs are blocked.

## Overall assessment

`agov` is, with high confidence after three convergent real Track 2
executions, a reliable compliance-assistance and audit-formatting tool
for **structural** correctness — required fields, enum values, a
handful of hand-coded cross-record invariants. It is not, and by
current design cannot be, an **authority enforcement** engine: actor
identity, evidence truthfulness, tamper-evidence over time, and
semantic agreement between records are all accepted at face value.
This matches the framework's own stated positioning (human governor +
CI + assistant discipline, not a runtime enforcer), but findings 1-4
above are concrete and independently reproduced, not theoretical, and
worth a disposition:

- Findings 1-2 (approval identity/non-emptiness) look like a
  straightforward schema-level fix: require `gate1_approved_by`
  non-empty and matching an active governor-equivalent role when
  `status: approved`.
- Finding 3 (mutation-served-as-authentic) is a real gap in the
  "append-only, immutable" claim this project makes repeatedly in its
  own documentation; even a lightweight content-hash-on-first-reference
  check would close most of it.
- Finding 4 (zero local Gate enforcement) may be an accepted,
  documented design tradeoff already — worth explicitly confirming
  that's the case rather than assuming, since it means the *observed
  technical merge backstop* for this project is hosted CI and
  branch-protection configuration, not `agov` itself — and that
  backstop, even correctly configured, checks record shape rather than
  authenticating who made a gate decision or whether they were
  entitled to. That is a narrower, more precise claim than "GitLab CI
  is the real enforcement," which would incorrectly collapse human
  governance authority into a CI property. The boundary between what
  the pipeline blocks and what still depends on human/procedural trust
  is currently implicit rather than stated plainly anywhere a new
  adopter would find it before relying on it.
- Finding 6 (`to: all` handoff blind spot) looks like a straightforward
  bug fix.

**Label, per the dossier's own instruction:** Track 1 conclusions from
a documentation-only reviewer (Lumo) must not be treated as
functionality validation, and Lumo's specific technical claims should
not be trusted without independent verification going forward. Tracks
2 and 3 here are empirical, against the actual published release
artifacts and real binary execution, not documentation review, and are
reproducible by anyone with the release archive and the commands
referenced above.
