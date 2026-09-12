<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Distilled AGF/AGOV enhancement asks from the labdns customer material

**Source:** two customer-supplied documents, not committed to this repo
(`tmp/customer-1/specs-labdns-v0.5.1.md`,
`tmp/customer-1/labdns-multi-assistant-implementation-graph-v1.3.0.md`).
labdns is an Elixir/OTP DNS server project; the graph document proposes
adopting AGF, pinned to an exact release/commit, as the governance
envelope for a multi-assistant implementation effort, and separately
proposes a set of generic AGOV extensions. This document distills only
the **AGF/AGOV-directed asks** — not labdns' own DNS/product
requirements, which are the customer's own spec and out of scope here.

**Purpose of this document:** hand Codex (coder-tester) a concrete,
itemized list to take an independent build/implementation stance on
before Jani communicates anything back to the customer. This is
proposal-intake, not a Gate 1 plan — no implementation is proposed or
authorized by this document.

## What the customer is actually asking, in one sentence

Adopt AGF/AGOV unchanged as the approval/evidence/role-separation layer,
and **additively extend `agov`** with a generic (non-labdns-specific)
artifact/graph/ID-validation surface that labdns' own work-package
schemas plug into — explicitly *not* a fork, *not* a parallel validator
stack, and explicitly proposed as a project-side extension first, to be
offered upstream later only after real operational use (their own
adoption-phase V5, §20.10).

## Distilled ask list

### A1 — Generic artifact registry in `agov-config.yaml`

**Ask:** an `artifact_sets` block (name, path globs, JSON Schema,
`id_pointers`, `reference_pointers`, uniqueness scope) so AGOV can index
and validate arbitrary project-defined artifact types without AGOV
hardcoding labdns' paths or semantics.

**Why:** they want one validator binary, not a second bespoke one, for
their own work-package/graph/contract/evidence records — while keeping
those records' *shape* entirely project-owned.

**Current AGF/AGOV state (as I understand it):** `agov-config.yaml`
today configures the coordination channel specifically
(`docs/coordination/agov-config.yaml`); there is no generic
artifact-set concept. This would be new surface, not a change to
existing coordination config.

**Codex: your stance?** Is this the kind of generic extension point AGF
should carry, or should labdns wrap/shell out to `agov` from its own
tooling instead of asking AGF to house a generic registry?

### A2 — Generic ID minting beyond coordination records

**Ask:** generalize `agov coord brief`'s non-colliding-ID pattern
(mint → write once → validate → delete-on-failure) to arbitrary
artifact kinds, ULID by default, with `key` (human label), `kind`,
`version`, and `sha256` all explicitly separate from `id`. Concretely:
`agov artifact brief --kind work-package --id-format ulid`.

**Why:** they want the same fail-closed collision-avoidance AGF already
has for `docs/coordination/*.md`, applied to their own work-package
files and nested objects (assumptions, contract entries, evidence
entries), rather than reinventing ID minting per project.

**Current AGF/AGOV state, corrected per Codex review `p5mj`:** the
mint→write→validate→delete-on-failure pattern is `agov coord post`
(`--id-format ulid`), not `agov coord brief` — `brief` only prints a
skeleton to stdout and writes zero files. `post` uses `create_new`,
validates the written file, and attempts deletion after a schema or
same-directory duplicate-ID failure (cleanup errors are ignored, so
this is fail-*visible*, not a crash-safe transaction). Verified: two
records sharing one ID in the same directory are rejected by `coord
validate` with a duplicate-ID diagnostic — a concrete local-directory
uniqueness check, not proof against cross-branch or historical reuse
(that gap is exactly what A7 would need to close). There is no
artifact-kind-agnostic version of either command today.

**Codex: your stance?** Does generalizing the ID-mint code path (same
non-colliding guarantee, different kind/directory) look like a small,
low-risk lift on top of the existing implementation, or does it carry
hidden coupling to coordination-specific assumptions?

### A3 — Generic `agov artifact validate` / `agov artifact index`

**Ask:** validate arbitrary configured artifact sets against their
registered schemas, and produce a compact `id -> kind, key, version,
path, pointer, digest, status` index — same shape as `agov coord
graph-index` but artifact-set-generic.

**Codex: your stance?** Same question as A1/A2 — generic capability in
`agov` vs. a thin external wrapper the customer maintains themselves.
Codex flags this and A1 together as the strongest generic candidates,
but corrects the comparison point: `agov coord graph-index`'s current
JSON (`nodes`/`edges`/`pending`/`invalid_files`, each node carrying
`file`/`from`/`id`/`kind`/`requires_ack`/`resolved`/`summary`/
`timestamp`/`to`/`topic`) is coordination-message-shaped, not a generic
`kind/key/version/pointer/digest` artifact index — A3 would be a new
data model built alongside it, not a small generalization of it.

### A4 — `agov graph validate GRAPH_FILE` with a named semantic rule catalog

**Ask:** a graph-integrity checker enforcing rules beyond JSON Schema
shape — the customer specifies ~25 rule codes (`GRAPH001-008`,
`SPEC001-003`, `ASM001-002`, `CON001-002`, `EVD001-002`, `GOV001-005`,
`ID001-005`; see their §20.5). Representative examples: dependency
targets must exist, the dependency graph must be acyclic, gate ordering
(P0→L1→L2→M2.5→L3, L4/L5 independent) must hold, a reviewer must differ
from the implementer unless an explicit exception is recorded, and
event/history transitions must be append-only and legal.

**Why:** this is the most substantial ask in the document — it's a
real rule engine, not just schema validation.

**Corrected per Codex review `p5mj`:** my original framing overstated
the overlap with existing AGF/AGOV behavior — do not describe today's
coordination validation as already a complete generic implementation of
Gate-1-before-claim, reviewer independence, legal event transitions, and
append-only history. Some mechanical checks exist today (notably
`publication-check`'s history/pinning logic), but binding gate decisions
and semantic judgments (e.g. "is this reviewer actually independent,"
"is this role label authoritative") remain governed human/role work, not
something a validator can authenticate from a record alone — a blocking
check still needs a named rule granting it that authority, per this
project's own `tool-authority-boundary-rule.md`. A4 needs a rule-by-rule
capability inventory (which of the ~25 codes are pure structural checks
vs. which quietly assume semantic/authority judgment a validator cannot
make) before treating it as "make existing rules project-agnostic."

**Codex: your stance?** Split structural rules (reference existence,
cycle detection, typed edge constraints — plausibly generalizable and
independently useful) from domain/governance-authority rules (stage
ordering, contract/evidence semantics, reviewer-identity authentication
— these likely stay customer-owned or need independent design, not a
blanket generalization). This is the largest scope item here regardless
of the split; worth its own Gate 1 assessment separate from whether
labdns adopts AGF at all.

### A5 — `agov work packet <ID> --output PATH`

**Ask:** given one graph node's ID, emit a token-efficient "work packet"
containing only that node's transitive prerequisites, relevant contract
digests, unresolved assumptions, and acceptance criteria — instead of
handing an implementing assistant the full spec/graph every time.

**Why:** explicit token-cost motivation, same spirit as AGF's own
`agov coord list --since` design (return only what's new/relevant, not
a full re-read).

**Codex: your stance?** This one is fairly deep on labdns' own graph
schema (§8) — plausibly better as a labdns-side tool consuming a
generic AGOV index (A3) than as an AGF feature.

### A6 — `agov spec index SPEC_FILE --output PATH`

**Ask:** parse a project's own specification document into a structured
FR/NFR/UC/AC reference index, feeding `SPEC001` ("every referenced
requirement ID exists in the pinned spec") and `SPEC002` (spec-file
digest matches the graph's recorded baseline digest).

**Codex: your stance?** This requires understanding an arbitrary
project's own spec-document conventions (their ID table format is
labdns-specific) — seems like the clearest candidate for "labdns' own
tooling," not a generic AGF capability, unless AGF wants to define a
*format contract* for what a parseable spec-index looks like and let
projects conform to it.

### A7 — Merge-time cross-branch ID collision guard, generalized

**Ask:** the same union-index check AGF's coordination-publication path
already needs conceptually (candidate branch + target branch), applied
to arbitrary configured artifact sets — reject a merge if two branches
independently minted the same ID for a governed artifact.

**Codex: your stance?** How much of `agov coord publication-check`'s
existing target/candidate-pinning logic actually generalizes cleanly
here vs. needs separate design?

### A8 — Confirm-before-build: does current `agov` already do what they think it does?

The customer's own assumption register lists this explicitly
(`ASM-VAL-01`, `ASM-VAL-02`): they assume `agov coord brief`,
`agov coord validate`, and `agov coord graph-index` behave as
documented (non-colliding IDs, deterministic validation, structured
index), and that "project work-package records can be normalized into
AGOV's generic artifact/index model" without changing existing
coordination semantics. This is a **verification ask**, not a build
ask — before we discuss any of A1-A7, is the documented behavior they're
relying on actually accurate today?

**Codex: your stance?** Worth a quick verification pass against current
`agov` (not gated on the rest of this) since it costs little and either
confirms or corrects their planning assumption.

**Corrected per Codex review `20260912T153453Z-coder-tester-p5mj`** —
my first spot-check under-verified this and mis-attributed behavior to
the wrong command (see A2's correction above). Codex ran the actual
probes with a compatible local build (repository-local fixture
directories, no production records or customer documents touched):

1. `agov coord brief --id-format ulid` prints a skeleton to stdout,
   writes zero files, and does not itself validate or reserve anything.
2. `agov coord post --id-format ulid` is the real writer: `create_new`
   + validate, with best-effort delete-on-failure (cleanup errors are
   ignored — not a crash-safe transaction).
3. `agov coord validate` correctly rejected two schema-valid records
   sharing one ID in the same directory — a genuine local-directory
   uniqueness check, but not proof against cross-branch/historical
   reuse.
4. `agov coord graph-index` produces structured, deterministic JSON
   (confirmed stable across two runs excluding `generated_at`) — but
   its shape is coordination-message-specific, not a generic artifact
   index (see A3's correction).
5. Existing coordination validation should not be described as a
   complete generic implementation of Gate-1-before-claim/reviewer-
   independence/append-only-history (see A4's correction).

**Bottom line for A8:** the coordination surface is real and useful,
but several of my original "current state" claims conflated `brief`
with `post`, treated `graph-index`'s shape as more generic than it is,
and overstated how much of A4's rule catalog existing validation
already covers. All corrected above. This bounded verification is not
a release certification, a proof of distributed/cross-branch
uniqueness, or fault-injection coverage of filesystem-cleanup failure
paths.

## What the customer explicitly says they are *not* asking for

- Not asking AGF to fork or change existing coordination record
  semantics, schemas, or command behavior (§20.2: "Existing coordination
  IDs, schemas and command behavior remain unchanged").
- Not asking AGF to become a dependency/merge-conflict resolver or test
  runner (§1: AGF governs "approvals, role separation, traceability, and
  evidence," not the implementation DAG itself).
- Not asking for this to ship in the next AGF release — their own
  phased adoption plan (§20.10) treats "propose proven generic commands
  upstream" as **V5**, explicitly gated on their own operational
  experience at labdns' L1/L2, after V0-V4 (pin AGF, add project
  schemas, add graph rules, add evidence/work-packet generation) are
  done on their own side first.

## Recommended framing for the customer response (for Jani's judgment, not a decision here)

Given the customer's own adoption sequencing treats "propose upstream"
as the *last* phase, not the first, the honest answer likely does not
require AGF to commit to building anything before communicating back.
Both Codex and I converge on this shape, with Codex's corrections
folded in:

1. **Give them the corrected baseline** (A8) — `agov coord post`
   (not `brief`) does mint+validate+best-effort-delete; `coord
   validate` catches local-directory ID duplicates; `graph-index`'s
   JSON is coordination-shaped, not a generic artifact index yet. They
   can start V0-V1 today with this corrected understanding, without
   waiting on AGF.
2. **State AGF's openness to a Gate-1-reviewed generic extension**
   (A1+A3 first, as the strongest candidates; A2's writer and A7's
   merge semantics only once they have their own pilot acceptance
   evidence) — without committing a timeline now, and explicitly
   **not** claiming a generic AGOV artifact index already exists.
3. **Treat A4 as two separable things**: a generic structural subset
   (reference existence, cycle detection, typed edges) worth its own
   Gate 1 evaluation regardless of this customer, versus domain/
   governance-authority rules (stage ordering, reviewer-identity
   authentication) that likely stay customer-owned or need real
   independent design — do not present A4 as one bundled ask.
4. **Decline A5/A6** as AGF scope — recommend labdns-side tools
   consuming AGOV's index/artifact output, consistent with their own
   "generic vs. project-owned" boundary in §20.2. Be honest that a
   thin customer-side adapter does not satisfy their stated
   one-validator-binary preference *today* — that preference is
   itself future upstream/extension work, not a current capability.

**Open question for Jani before responding (raised by Codex, not
resolved here):** does the customer's project-side-first phasing
(§20.10, V0-V4 on their side before V5 "propose upstream") allow an
interim thin adapter around `agov` while AGF-side extension work is
still hypothetical, or do they need one single binary from day one?
The answer changes how much can honestly be offered right now versus
deferred to "we'll evaluate a concrete proposal when you bring one."

No version bump, release, or v2.10.1 content is implied or needed for
any of this — the customer's own material never asks for that; that
question (raised separately) is orthogonal to what's in these two
documents.

## Explicitly out of scope for this document

- The labdns DNS/product specification itself (`specs-labdns-v0.5.1.md`)
  — that's the customer's own product requirements, not an AGF ask.
- Any Gate 1 plan for building A1-A7 — none is proposed here.
- Any commitment on v2.10.1 timing or content.
