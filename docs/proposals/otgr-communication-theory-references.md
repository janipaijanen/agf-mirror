<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Two reference points for OTGR and coordination-message design

**Status:** Proposal-intake / reading list, not a Gate 1 plan. No
implementation is proposed here. Deliberately filed for later revisit when
OTGR design work resumes. v2.10.1 has since shipped; v2.11 remains the active
release-under-work and retains priority. See
`docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md` and
the resumed RUW/v2.11 trail following
`20260912T151723Z-planner-architect-31pr`. This document exists so the
reasoning is not lost between now and the OTGR design pass.

## 1. Conway's Law, updated for AI actors

Source: Domen Kožar, "Conway's Law Is Dead, or Is It?"
(<https://www.domenkozar.com/2026/09/12/conways-law-is-dead-or-is-it/>),
published 2026-09-12; rechecked 2026-09-17.

**Core argument:** AI is now a participant in the communication
structures Conway's Law describes — "interpreting requests, writing
code, and reviewing changes" — not merely a tool operated by humans
inside those structures. The article argues project workflows should
therefore shift from **proof-of-process** gates (special marker files,
human-review-notice conventions treating agents "like students taking
tests") toward **outcome/evidence-based** standards: does the fix
address a reproducible failure, does a regression test capture the
intended behavior. It also names a real tension: evidentiary burdens
calibrated for AI-speed actors can become expensive enough to exclude
a human contributor working by hand, risking a two-tier project where
workflow design implicitly favors one kind of actor over the other.

**Relevance to AGF/OTGR:**

- AGF's own `always/gate-self-test-rule.md` already embodies the
  article's central recommendation — a gate ships only with a
  demonstrated negative test, not a procedural sign-off. This is
  independent confirmation the framework's existing direction matches
  a live 2026 external argument, not a new idea to adopt.
- The two-tier participation risk is worth testing against AGF's own
  coordination overhead empirically, not assumed away. As one concrete
  data point: the v2.10.1 customer-scope work (a comparatively small,
  well-bounded feature — three files, ~1,900 lines) generated over a
  dozen coordination records and two separate authority-attribution
  corrections before Gate 2 closed. Whether that overhead is
  "appropriately strict for AI-speed, error-prone actors" or "would
  price out a human contributor working at human speed" is exactly the
  question the article raises, and is worth an honest look when OTGR's
  own design gives AGF governance a second real testbed to compare
  against.
- OTGR was explicitly scoped as *non-authoritative* precisely to avoid
  becoming a second, competing governance channel
  (`20260904T104400Z-governor-reviewer-q7mv`). The article's framing
  sharpens why that boundary matters: an under-governed fast channel
  and an over-governed slow channel are two different answers to the
  same actor-inclusion question, and OTGR was designed to be the
  former on purpose.

## 2. Shannon's mathematical theory of communication — calibrated relevance

Primary source: Claude E. Shannon, “A Mathematical Theory of Communication,”
*Bell System Technical Journal* 27 (1948), parts 1 and 2
(<https://doi.org/10.1002/j.1538-7305.1948.tb01338.x> and
<https://doi.org/10.1002/j.1538-7305.1948.tb00917.x>), rechecked 2026-09-17.

**Core relevance, and its limit:** Shannon's theory concerns reliable
transmission of symbols over a noisy channel — encoding, channel
capacity, redundancy for error detection/correction — and explicitly
brackets out meaning: "the semantic aspects of communication are
irrelevant to the engineering problem." That limit matters here: the
concrete coordination failures surfaced during v2.10.1's own review
cycle this session (a decision-record transcribed under the wrong
role attribution; a technical evidence-applicability finding
mistaken for an extension of binding gate authority) were **meaning
and authority failures, not signal-corruption failures**. Classical
channel-capacity math does not directly diagnose or fix those; they
are resolved by role-authority rules (`AGENTS.md`'s gate discipline),
not by adding redundancy to the transmission itself.

**Where it could still genuinely help, when OTGR design resumes:**
framing OTGR (fast, low-redundancy, informal, human-and-AI-legible)
against `docs/coordination/` (slow, high-redundancy, schema-validated,
append-only) as a real **rate/reliability tradeoff** along a
Shannon-style channel-coding axis. That framing gives OTGR's eventual
design a principled way to answer questions like "how much structure
does an OTGR message need before it's reliably distinguishable from
noise/misattribution, at what cost to the low-friction speed that is
OTGR's whole reason to exist" — rather than reinventing that tradeoff
from scratch or copying `docs/coordination/`'s full redundancy by
default (which would just recreate a second governance channel and
violate OTGR's own non-authoritative design intent).

**Other candidate sources not yet reviewed**, flagged for the same
future revisit rather than pursued now: work on multi-agent
communication protocols and grounding (common ground / shared context
maintenance in dialogue systems), and CRDT/eventual-consistency
literature for append-only, multi-writer record design — both bear on
OTGR's stated goal of being reconstructable and studyable without
requiring access to full conversation history.

## Explicitly out of scope for this document

- No OTGR implementation, schema, or protocol design is proposed here.
- No change to `docs/coordination/`'s existing schema or validation
  behavior is proposed.
- No commitment to when or whether these framings get used — this is
  a reading-list/reasoning artifact, filed so it survives until OTGR's
  sequencing point arrives, not a design decision.
