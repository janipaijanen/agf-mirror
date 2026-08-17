# External simulated due-diligence assessments: ChatGPT, Claude, Grok, Lumo (2026-08-17)

## Provenance

The human Governor separately prompted four web-based AI assistants —
ChatGPT, Claude (claude.ai web, a separate session from any AGF planner-
architect session), Grok, and Lumo — with the "Open-Source Project Due
Diligence Assessment Protocol, Version 2.1" template (`raw/10-DD-template-
question.md`), naming the real public project (`agents-govern` / AGF), its
real GitLab repository, and its real public Pages site. Each system's raw
response is preserved verbatim in `raw/11-<Model>-Answer.md`. A second-round
follow-up question (`raw/20-Risk-question.md`, asking each system to
classify its own first-round findings as AGF-specific vs. broader
AI-governance-industry risk) was then put to the same four systems; those
responses are preserved verbatim in `raw/21-<Model>-Answer.md`.

This follows the same pattern as, and is explicitly a reassessment relative
to, the prior filing at `docs/reviews/20260810T134228Z-external-ai-
diligence-grok-copilot/` (Grok and Copilot, simulated DD of v2.5.0,
2026-08-10). Both ChatGPT's and Claude's round-1 answers cite that prior
filing's date and scope directly, confirming they were told about it as
context, not independently unaware of it.

**Naming the real project to a public AI system is not an anonymization
violation here.** `agents-govern`/AGF is itself a public, openly-licensed
project — the prior 2026-08-10 filing already established this same
practice without objection. `always/external-anonymization-rule.md`
protects a private/internal project's real identity from leaking; it does
not require anonymizing a project whose whole purpose is public distribution
and whose GitLab remote, Pages site, and documentation are already public.

**What is and is not known about this packet**, same limitation class as
the 2026-08-10 filing: the prompt text is known and reproducible (preserved
in `raw/10-DD-template-question.md` and `raw/20-Risk-question.md`). Not
preserved or available: invocation timestamps for each individual model
call (file mtimes on disk are 2026-08-17 but reflect when the Governor
saved the file, not necessarily the exact model-response time), which
product mode/model variant answered (e.g. which ChatGPT/Grok/Claude
model tier, whether browsing/deep-research was enabled), or an attestation
that no system was shown another system's answer before generating its
own. This is **separately prompted, cross-vendor reports relayed by the
Governor; session/model/blinding metadata unavailable** — the same honest
framing the 2026-08-10 filing settled on after its own initial overclaim
was corrected (`rp7m`'s `B-DD-FILE-1`/`B-DD-FILE-2` findings).

**`raw/11-LUMO-Answer.md` is an empty file (0 bytes).** Lumo's first-round
answer was not captured or was not produced; only its round-2 answer
(`raw/21-Lumo-Answer.md`) exists. Filed as-is, not backfilled or omitted,
so the gap is visible rather than silently absent.

## Relationship to the separate "fresh-context evaluator" effort

This project has a distinct, much heavier apparatus for a *genuinely*
blinded due-diligence evaluation: a pinned-baseline evidence package
submitted to a fresh Gemini conversation, gated through multiple Gate 1
review rounds (`docs/plans/20260811T110333Z-diligence-gate1-baseline-
pinned-plan.md`, governor decisions `mh8p`/`514h`, contribution-auditor
rounds through `ga1r`). That effort's last visible activity in this
channel is 2026-08-12 (`pbe5`, a scope-bloat learning correction); I found
no evidence it was formally closed, abandoned, or completed as of this
filing. **This filing is not that effort and does not feed it.** The two
are independent: this is four public-facing chat assistants answering a
disclosed, non-blinded prompt; the other effort's entire point is a single
blinded evaluator with a pinned, allowlisted, non-coordination evidence
universe. Storing this filing does not add content to that other effort's
evidence package. Flagging the open question rather than asserting
certainty: if that effort is still live, someone with visibility into its
current state should confirm whether any containment boundary needs to
apply here too — I don't have enough information in this channel to rule
it out with confidence, only to say I found no direct evidence of overlap.

## Known specific claims requiring independent verification before treating as fact

Per this project's own `always/verifiable-claims-rule.md` and the
2026-08-10 filing's own corrected overclaims, the following notable claims
across these eight non-empty responses are flagged as **not yet checked**
against the actual repository — listed here so review doesn't have to
re-derive them, not asserted as either confirmed or false:

- **Grok (round 1)** states as settled fact that "tooling code MIT" is
  the current license. The 2026-08-10 filing already found the equivalent
  Grok claim false against `tooling/rust/Cargo.toml`'s actual
  `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial` declaration at that
  time — worth checking whether this is the same recurring error or
  whether the license actually changed since.
- **ChatGPT (round 1)** claims a licensing *contradiction* rather than a
  settled MIT state: "Root documentation says Rust tooling is MIT, while
  Cargo and OCI metadata say CC BY-SA/commercial." This is a different,
  more specific claim than Grok's and deserves its own check — is there
  actually a doc page currently claiming MIT for the tooling?
- **ChatGPT (round 1)** claims "333 coordination-path files and 12 plans
  failed current validation." A specific, checkable count — not verified
  here.
- Various maintainer-concentration / commit-percentage claims (e.g. Lumo's
  "sole maintainer," ChatGPT's "~96% of commits are attributable to the
  owner") are plausible directionally but the exact figures are not
  independently re-derived in this filing.

## Disposition

Filed for reading and cross-model debate between planner-architect and
coder-tester per the Governor's direct request. Not merged to `main`
without further review — this MR requests review, not immediate merge, in
keeping with the 2026-08-10 filing's own practice of holding merge
decisions separate from content review for this class of material.

This filing does not authorize any Tier 1-N roadmap action by itself, does
not treat any external response as confirmed fact, and does not claim
independence stronger than what is actually attested above.
