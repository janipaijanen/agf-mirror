<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Architecture Decision Record (ADR): agents-govern 2.0 scope and directory authorization

## Status

Proposed (Provisional)

## Date

2026-05-06

## Target framework version

agents-govern 1.1.46 (canonical) → 2.0 (Provisional candidate)

## Context

The framework's manifesto-cycle work (2026-05-04 to 2026-05-05) produced a Provisional candidate set: `MANIFESTO.md`, `docs/adr/20260505-lean-agile-test-fly.md`, `docs/adr/20260505-yaml-ld-provisional-artifact-shape.md`, `docs/process/manifesto-self-test.md`, `docs/process/manifesto-self-test-v3-result.md`, `docs/process/manifesto-provenance.md`, `docs/process/commentary-triage.md`, `learnings/linked-data-probe.yaml` (12 entries), and three YAML Ain't Markup Language (YAML)-LD experiment artifacts under `experiments/yaml-ld/` (`fact-checker`, `coder-tester`, `governor-reviewer`). Sentinel Review #6 (2026-05-05, binding with corrections) confirmed Provisional status across the candidate set and recorded the corrected post-Test-Fly-#5 evidence state.

Two Chief Technology Officer (CTO) first-arrival encounters with v1.1.46 (`README.md` line 36-40 records the first; the second is documented in the structure assessment dated 2026-05-05) have reported friction on the v1.1.46 surface. Verified frictions include:

- 8 broken README links to upstream-only paths excluded from the release archive.
- Top-level density (18 files + 10 directories at root).
- Heavy root docs (`CHANGELOG.md` 6000 lines, `framework.md` 1788, `ADOPTING.md` 1238).
- `dist.py` archive builder out of sync with new manifesto-cycle artifacts; the next v1.1.46 archive build would silently exclude them.
- `.venv/` clutter visible at root (untracked but visible).
- `adoptation.md` filename surprise (intentional but reads as typo).

The Test-Fly evidence base after 5 cycles has declining revise friction (1, 2, 0, 0, 0 rounds), demonstrating the manifesto + Test-Fly discipline operates at the framework's internal scale.

Modifying v1.1.46 in place is constrained by:

- The operating-files plan's "no copied next-generation tree" recommendation.
- The Q5 phase-1 layout refusal, conditional on N≥3 layout-blockage learning records.
- The framework's immutability and append-only conventions.

Two paths existed: (I) ADR-based revision authorizing a v2.0 directory; or (II) rescope to fit existing constraints. Jani as Framework Sentinel chose Path I on 2026-05-06.

## Decision

1. **Authorize creation of `agents-govern-2.0/` at repo root,** side-by-side with `agents-govern-1.1.46/`. The latter is untouched; existing files remain immutable.

2. **v1.1.46 remains canonical** until further notice. v2.0 is Provisional. Adopters reading the framework today land on v1.1.46.

3. **YAML-LD-compatible YAML remains the Provisional working shape inside v2.0.** It is not promoted to authoritative truth. ADR `20260505-yaml-ld-provisional-artifact-shape.md`'s Provisional disposition carries forward into v2.0.

4. **YAML-LD conversions inside v2.0 are evidence-question-driven, not momentum-driven.** Sentinel Review #6 D5's "no queue-momentum" discipline applies to v2.0 work by analogy.

5. **v2.0 design is friction-driven, not persona-driven.** Evidence-supported personas: CTO (N=2 encounters), AI architect (N=3 AI services in the manifesto consensus thread). Product Manager (PM), Research, Development, and Innovation (RDI) manager, Senior Vice President (SVP) engineering, SVP communication: N=0; content for them remains speculative and is excluded from v2.0 scope.

6. **No new agents in v2.0.** The framework's existing UX-Accessibility cross-cutting concern *may* be activated for v2.0's documentation surface; DX is not added as a new agent or concern.

7. **v2.0 in-scope:** first-arrival clarity, archive/source split, broken-link repair, this scope ADR. **v2.0 out-of-scope:** persona-targeted books, standardization positioning, marketing-style rewrite.

8. **Plan only first.** No structure creation in this ADR. Subsequent Test-Fly cycles handle each v2.0 component (manifesto placement, YAML-LD use, README, etc.).

## Supersession (explicit, narrow)

This ADR explicitly supersedes the following prior rules **for the specific case of v2.0 directory creation only**:

- The operating-files plan recommendation against a "copied next-generation tree." Other recommendations in the plan (live source / deliberation / experiment lane discipline; broad rename refusal) **remain in force.**
- The Q5 phase-1 layout-refusal amendment, **only insofar as it would otherwise forbid `agents-govern-2.0/` directory creation.** Layout cleanup *inside* v1.1.46 remains refused without N≥3 layout-blockage learning records.

This ADR does **not** supersede:

- Sentinel Review #6 D5 (no queue-momentum on YAML-LD conversions). The discipline applies to v2.0 work by analogy.
- ADR `20260505-lean-agile-test-fly.md` (Lean-Agile mapping + Test-Fly principle).
- ADR `20260505-yaml-ld-provisional-artifact-shape.md` (YAML-LD-compatible YAML as Provisional artifact shape).
- The framework's manifesto provisions.
- Tenet 8 (Promotion-by-Review with Corroboration). v2.0 ships Provisional.
- Naming convention's hard rules (immutability, no retroactive renames). v1.1.46 files are not renamed or moved.

## Consequences

**Positive:**

- v2.0 directory provides space for friction-driven first-arrival fixes without modifying v1.1.46.
- Side-by-side preserves v1.1.46 immutability.
- Friction-driven design honors Tenet 2 (Growth-by-Pain).
- Plan-only-first prevents premature commitment to design that hasn't been Test-Flown.

**Negative:**

- Two directories require dual maintenance until v2.0 promotes (or is retired).
- Risk of source-vs-archive boundary confusion compounds.
- Adopters mid-evaluation may be confused about which directory is canonical; mitigation is the v2.0 README's explicit Provisional banner.
- N=0 personas (PM, RDI manager, SVP engineering, SVP communication) explicitly receive no v2.0 content; Tenet 2 (Growth-by-Pain) is honored at the cost of those audiences.

## Non-Goals

- No persona-targeted books or marketing surface.
- No new agents (no UX agent, no DX agent).
- No standardization positioning (cross-AI-ecosystem rebrand or W3C-style spec direction).
- No tooling commitments in this ADR (validators, generators, migration tooling).
- No promotion of v2.0 above Provisional within this ADR.
- No deletion of v1.1.46 artifacts.

## Acceptance Criteria for subsequent v2.0 work

Each subsequent v2.0 work item, after this ADR lands, must:

- Cite a specific evidence question (per Sentinel Review #6 D5 spirit).
- Pass through its own Test-Fly cycle: proposal → review → implementation → outcome.
- Honor Tenet 9 (Shrinkage) — including weighing removal alongside addition.
- Avoid persona-targeted speculation per friction-driven scope.
- Avoid new agent introduction.

## Rejection / Revision Criteria

This ADR can be revised or superseded if:

- v1.1.46 friction patterns change such that v2.0 work is no longer justified.
- N≥3 layout-blockage learning records accumulate, suggesting v1.1.46 in-place restructure is more appropriate.
- Independent-adopter corroboration emerges that materially changes the design space.
- A subsequent Sentinel review concludes v2.0 work has not produced commensurate value.

## See also

- `MANIFESTO.md` — the live binding Provisional manifesto carrying into v2.0.
- `docs/adr/20260505-lean-agile-test-fly.md`.
- `docs/adr/20260505-yaml-ld-provisional-artifact-shape.md`.
- `docs/process/manifesto-self-test.md`, `manifesto-self-test-v3-result.md`, `manifesto-provenance.md`, `commentary-triage.md`.
- `learnings/linked-data-probe.yaml`.
- `GOVERNANCE.md` Sentinel Review Log entry for Review #6.
