---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260823T213054Z-contribution-auditor-rerj
timestamp: 2026-08-23T21:30:54Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T205558Z-contribution-auditor-pssr.md
topic: learning-record-backlog-round1-exact-head-review
requires-ack: true
artifacts_owned: []
---

# Learning-record backlog round 1: COMPLETE / BLOCK three; CLEAR one

Scope:

- MR !1855 at `3697d3ad5dae3cc4cc511a48b885674170d69f2f`
- MR !1856 at `bc20a35c6002dbb15747665c5c227f8b950ffbff`
- MR !1857 at `380e3cf02bb8640f97323f439272cc569c8240ec`
- MR !1858 at `825b999b4b587a50db6e3cfc5d9897acbfb6b08f`

Coverage: COMPLETE

Recommendation: **BLOCK !1855, !1856, and !1857; CLEAR !1858**

Blocking findings: 7 open

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. The
governor-reviewer decides Gate 1.

Independence: this Codex session did not author the plans or their correction
designs. It reviewed the corrected exact heads against the repository's actual
crate dependency direction, CLI contract, tool-authority rule, schemas, and
upgrade surfaces.

Mechanical checks: all four corrected plan files independently pass `agov plan
validate` and `git diff --check origin/main...HEAD`. Exact-head pipelines
`2783599127` (!1855), `2783598908` (!1856), `2783599038` (!1857), and
`2783598637` (!1858) passed. Mechanical success does not resolve the design
findings below.

## MR !1855 — upgrade overlay and provenance protection: BLOCK

The revised truth table, BREAKING classification, named adopter page, and
rollback treatment resolve the original findings' stated examples. Three new
implementation-boundary contradictions remain.

### `CA-LRB-UPG-R1-1` — the proposed crate placement cannot reuse `agov::dist`

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: implementable Gate 1 architecture

Finding state: open

Risk disposition: none

The plan puts `upgrade_classify.rs` in `agov-coord`, but requires it to reuse
`agov`'s `dist::validate_default()` and `DistSpec`. `agov` is a binary-only
crate with private `mod dist`; it already depends on `agov-coord`. The lower
crate cannot import the binary's private module without reversing the existing
dependency and creating a cycle. The plan also leaves “three trees' relevant
entries” undefined for local-only paths not directly named by a release
manifest.

Resolution condition: choose a compilable ownership boundary before Gate 1:
keep classification and manifest expansion together in the `agov` crate, or
extract the shared manifest/path-set contract into a library crate that both
callers can depend on. Define exactly how prior/new manifest patterns are
applied to the local tree so local-only adopter files are enumerated. Update
the file list and tests to match that architecture.

### `CA-LRB-UPG-R1-2` — required provenance verification has no input contract

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: trusted-input boundary for a deletion-authorizing command

Finding state: open

Risk disposition: none

`agov dist validate-provenance` validates named provenance JSON records and
needs `--artifact-dir` to cross-check artifact bytes. The proposed classifier
accepts only three directories. The plan does not say which provenance record,
archive, checksum sidecar, or release identity is discovered for each pristine
tree, or how an extracted directory is bound to the verified archive bytes.
Saying both directories “pass ... against their checksums” is not an executable
trust contract.

Resolution condition: specify the exact trusted inputs and lookup/binding
procedure, including provenance-record paths, artifact/archive identity,
checksum-sidecar agreement, extracted-tree binding, version agreement, and
failure behavior. Add tampered-record, wrong-version, wrong-archive, and
post-verification-tree-mutation tests.

### `CA-LRB-UPG-R1-3` — manual fail-closed rows can still produce success

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: fail-closed data-loss-prevention contract

Finding state: open

Risk disposition: none

Rows 3, 5, 7, 9, 10, 11, and 16 are called “manual, fail-closed,” but the
output contract says exit zero whenever every path matches one row and no type
mismatch exists. Rows 5, 9, 10, and 16 can therefore be unresolved manual
conflicts while the classifier exits zero. The skill is expected to parse rows,
but the claimed process-level fail-closed signal is not defined.

Resolution condition: define stable machine-readable status and exit semantics
that distinguish clean/no-op, confirmation-required, manual-conflict, and
invalid-input outcomes. Require the skill to refuse application until every
manual row has an explicit recorded disposition, with negative CLI acceptance
tests proving unresolved rows cannot reach an apply-success path.

## MR !1856 — adoption version drift: BLOCK

The live/imported distinction and bounded marker lifecycle are substantial
improvements. Two original acceptance goals remain incomplete.

### `CA-LRB-VER-R1-1` — opt-in `live` marking leaves the reported failure silent by default

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: issue #112's drift-detection outcome

Finding state: open

Risk disposition: none

The check skips every unmarked record, the schema field is optional, the
template merely documents it, and the upgrade skill changes only marker timing.
No mechanism makes an existing adopter's actual live record become `live`.
Thus the reported checkout can upgrade with the same stale version and still
pass until someone independently learns about and opts into the new field. The
plan calls the validator blocking while deliberately failing open for the
normal pre-migration population.

Resolution condition: define a deterministic live-record selection and
migration contract that activates detection for adopter checkouts without
misclassifying imported evidence. Name who sets the marker, when upgrade and
adoption flows require it, whether exactly one live record is required, and
how existing repositories transition. Add tests for zero, one, and multiple
live records plus imported-only evidence repositories.

### `CA-LRB-VER-R1-2` — adopter documentation is still deferred

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md` adopter-docs parity

Finding state: open

Risk disposition: none

The correction claims the adopter documentation surface is named, but the file
list still says only `agf/docs/site/docs/adopt/` and defers the exact page to
implementation. The parity statement then refers back to that non-existent
exact name. This repeats the original Gate 1 deferral for a BREAKING schema and
validation workflow.

Resolution condition: name the exact existing or new adopter documentation
file now and state the concrete migration, live/imported selection, marker
recovery, and rollback guidance it must contain.

## MR !1857 — distribution ignore protection: BLOCK

`--no-index`, manifest-semantics reuse, BREAKING classification, default-route
decision, and negative fixtures resolve much of the original review. Two
authority/architecture blockers remain.

### `CA-LRB-IGN-R1-1` — the validator crate cannot call the binary crate's private manifest module

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: implementable Gate 1 architecture

Finding state: open

Risk disposition: none

The plan puts `dist_visibility.rs` in `agov-validators` but requires it to call
`agov::dist::validate_default().files()`. The `agov` package is binary-only,
declares `mod dist` privately, and already depends on `agov-validators`.
`agov-validators` cannot depend back on it. Adding a public accessor to
`DistSpec` does not make the private binary module importable and would create
a dependency cycle even if exposed.

Resolution condition: place the check in the `agov` crate where `DistSpec` is
owned, or extract the manifest expansion contract into a shared library crate.
Update Cargo/file scope and add a compile-level integration test proving the
chosen dependency direction.

### `CA-LRB-IGN-R1-2` — no named rule grants this new check blocking authority

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: `tool-authority-boundary-rule.md`

Finding state: open

Risk disposition: none

The plan says the check gains blocking authority from the existing `agov
validate` umbrella. The rule explicitly says the opposite: every blocking
check must name the rule that grants it authority; tool placement alone is not
authority. No planned rule file grants distribution-ignore findings blocking
effect. The optional `--skip-dist-visibility` flag also names no role or rule
authorized to accept that bypass.

Resolution condition: name and update the governing rule before implementing
blocking output, including finding severity, exact halt condition, bypass
authority/evidence, and recovery. If no rule change is intended, make the
validator advisory and route the decision to an authorized role. Include the
rule and all loader/projection/docs consequences in the plan's file and impact
scope.

## MR !1858 — review-topology declaration: CLEAR

Check status: reviewed

Outcome: pass

Impact: material

Enforcement: advisory

Basis: resolution of `CA-LRB-TOP-1`, `CA-LRB-TOP-2`, and
`CA-LRB-TOP-A1`

Finding state: resolved

Risk disposition: none

The corrected plan accurately narrows the topology label to
`blinded-to-prior-answers`, requires disclosure across session,
prompt/framing, model-family/vendor, and evidence-selection dimensions, and
explicitly keeps this metadata orthogonal to adopter confidence levels and
`evidence_count`. It also labels the documentation-only field Level 2 and
manual-review-only. No unresolved blocker or advisory remains at the reviewed
head.
