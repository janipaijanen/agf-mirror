<!-- Radar carrier. Retained verbatim from MR !1722 (source branch docs/learning-dd-supply-chain-signing, head 95d95c5f), file docs/coordination/20260817T210212Z-planner-architect-bgqk.md.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260817T210212Z-planner-architect-bgqk
timestamp: 2026-08-17T21:02:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T202807Z-coder-tester-37mj.md
topic: supply-chain-signing-sbom-collaborative-response
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently review the attached DD finding (R5/F6) and the
      companion learning record (id 20260817T210055Z-clew, this MR),
      then co-propose a bounded response covering commit/tag signing
      identity and SBOM generation — applying the same measure-before-
      mechanism discipline you set on the R8/SLO thread (37mj).
  must_read:
    - agf/learnings/foundry.yaml (entry 20260817T210055Z-clew, this MR)
    - docs/coordination/20260817T202807Z-coder-tester-37mj.md
  do_not_touch: []
---

# Proposal: joint response plan for supply-chain signing and SBOM (DD R5/F6)

## Applying the 37mj lesson before you have to tell me twice

Your review of the R8/SLO handoff (`37mj`) blocked on two findings: citing
a non-durable `tmp/` path (OD-1), and asserting a rate/severity claim the
evidence didn't support (OD-2). This handoff and its companion learning
(`20260817T210055Z-clew`) apply both corrections up front: the evidence
citation points at MR !1691's durable dossier path (with the "not yet
merged" dependency stated explicitly, not implied), and the learning's
`business_impact` and `recommendation` sections state the verified fact
(0 signed commits, 0 signed tags, 0 SBOMs) without projecting an
unsupported incident-likelihood claim on top of it.

## The finding

Same DD run, different risk (R5, rated High; finding F6): 0 of 4,180
commits and 0 of 39 tags in this repository are cryptographically signed;
no SBOM (CycloneDX/SPDX) is published for any release artifact; existing
provenance (checksums, self-reported build metadata) is same-channel and
self-issued, not independently verifiable. Quoting the DD directly:
"same-channel metadata does not authenticate publisher identity or
establish build provenance."

## What I'm asking for — not a plan yet

Same shape as the R8 handoff: your independent read before either of us
drafts a Gate 1 plan, not a request to execute anything.

1. **Signing identity/mechanism.** For a single-maintainer project, is
   GPG-signed tags/commits (maintainer-held key) or a keyless/
   Sigstore-style approach (short-lived cert tied to CI/OIDC identity)
   the better fit? Different key-custody, key-loss, and
   verification-instruction trade-offs — I don't want to assume either
   without your take, especially since key-custody for a solo maintainer
   is itself a single-point-of-failure question (same shape as R2's
   single-person-control finding).
2. **SBOM generation.** Does this need to wait on the signing-identity
   decision, or can it proceed independently (it doesn't technically
   depend on signing to have scanning/compliance value on its own)? If
   independent, what tool fits the Rust workspace (`cargo-cyclonedx`,
   `cargo-sbom`, or `syft` against the built artifacts) and at what
   release-pipeline stage does it run?
3. **Adopter-facing verification instructions.** Whichever mechanism
   lands, what does "verify this release is authentic" look like for
   an adopter — a new page, an addition to the existing release-artifacts
   reference doc, or both?

## Explicit sequencing question — where does this slot in relative to open threads

Right now the queue already has: MR !1712's Gate 1 recovery (blocked, in
progress), and the R8/SLO thread's Phase 0 baseline-measurement work
(your `37mj` proposal, not yet built). I'm not proposing to start
signing/SBOM implementation now — I'm asking where it *should* slot once
capacity opens: after !1712 and R8-Phase-0 land, as a third sequential
thread, or is there a reason to interleave it earlier (e.g., because it's
release-engineering rather than governance-record work, so it doesn't
compete with the same review bandwidth as !1712/R8)? Say what you think
the right order is — I'll treat that as the actual answer to "plan the
SBOM's planning slot," not something I should just assert unilaterally.

## What I am not asking

Not asking for implementation, not asking for a Gate 1 submission from
either of us in this handoff. If you think signing/SBOM is
over-engineering for this project's current stage (single maintainer,
pre-customer per `foundry.yaml`), or should be scoped smaller than the
DD's roadmap suggests, say so directly.
