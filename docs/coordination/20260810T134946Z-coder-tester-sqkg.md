---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read: []
  next_agent: planner-architect
  status: ready
  task:
  - Scope a bounded child plan (v2.5.1 umbrella or standalone) fixing the verified dangling agf/MANIFEST.md reference in ADOPTING.md:1013, adding an explicit adopter-upgrade checklist (adopter-owned path preservation, loader-surface scan), and classifying agf/agents/MANIFEST.yaml as adopter-owned in upgrade guidance -- per one real-adopter learning record (harbor-workshop, single-adopter confidence).
id: 20260810T134946Z-coder-tester-sqkg
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260810T134228Z-coder-tester-k5jd.md
- 20260810T052006Z-contribution-auditor-7kjh.md
- 20260810T052005Z-contribution-auditor-h7h4.md
- 20260810T155849Z-contribution-auditor-28r5.md
requires-ack: false
timestamp: 2026-08-10T13:49:46Z
to: planner-architect
topic: real-adopter-upgrade-learnings-v250-manifest-md-gap
---

# One real external adopter learning record, citing v2.5.0 upgrade friction

**Corrected per contribution-auditor review `28r5`** (`B-DD-LEARN-1`,
`B-DD-LEARN-2`, `B-DD-LEARN-3`). The earlier draft of this filing also
included a second, unnamed submission filed under a placeholder filename
(`pending-codename-20260810a`) and framed the two together as "independent"
and "convergent." Both framings were wrong: a placeholder is not the
adopter's codename and cannot be repaired by a `supersedes` pointer across
files, and the durable evidence for "independent"/"convergent" was a
coder-tester statement that the Governor received two emails — not a
Governor attestation that the submitters are distinct external actors, and
not a distinction between submitter-reported and independently-reproduced
facts. Per `28r5`'s stated resolution path, the unnamed submission is
removed from this filing rather than published under an intentionally wrong
name; it can be refiled separately once the submitter supplies a real
codename (new file, correctly named from the start — not a same-file
`supersedes` repair of the removed placeholder).

The human Governor received a learning record via email from an adopter
project that upgraded an existing AGF adoption from 2.2.4 to 2.5.0. Filed at
`agf/learnings/harbor-workshop.yaml`, validating clean under
`agov validate learnings` and `agov validate infosec`. Held at
`single-adopter` confidence — no multi-adopter or corroboration claim is
made.

**This is real adopter evidence, not simulated analysis** — the distinction
matters against the external simulated due-diligence reports (`k5jd`, this
same batch): those are AI-authored assessments of public metadata; this is
an actual upgrade event with actual validation command output as evidence.
Command outputs listed in `evidence_refs` are submitter-reported (run by the
adopter, relayed by the Governor) except where explicitly marked otherwise
below.

# Verified technical claim

The record references an adopter-upgrade-guidance gap. I independently
verified this one directly, myself, on this repository's own history (not
submitter-reported): `agf/ADOPTING.md:1013` tells adopters uncertain about
file ownership to check `agf/MANIFEST.md`. That file does not exist anywhere
in this repository's git history — not merely absent from the packaged
release archive, never created. The actual ownership-relevant content lives
in `agf/docs/governance/manifest.md` and in `governance-file-layout-rule.md`
itself. This is a dangling reference to the wrong filename, confirmed by
`git log --all --diff-filter=A -- agf/MANIFEST.md` returning nothing.

# Recommendation (from the one filed record)

1. Fix the dangling `agf/MANIFEST.md` reference in `ADOPTING.md:1013` —
   point it at `agf/docs/governance/manifest.md` (the file that actually
   exists and actually carries the ownership table), or ship the named file.
2. Add an explicit adopter-upgrade checklist: do not blindly replace the
   whole `agf/` tree; preserve adopter-owned paths
   (`agf/agents/MANIFEST.yaml`, project-specific `agf/agents/*.yaml`,
   `agf/learnings/<codename>.yaml`, `agf/docs/adr/`, local project skills);
   overlay upstream-owned framework paths; copy
   `agf/agents/MANIFEST.example.yaml` if new/changed; update loader/version
   references separately; run the `agov validate` family.
3. Classify `agf/agents/MANIFEST.yaml` explicitly as adopter-owned during
   overlays in upgrade tooling/guidance, not just in prose.

Item 4 of the prior draft (assistant-loader surface scanning) came from the
now-removed second record and is dropped from this recommendation set; it
may resurface if that submission is later refiled under a real codename.

# Requested action

Scope a bounded child plan — folding into the v2.5.1 umbrella (`1h8t`) seems
natural given its "genuine, fact-checked findings enter only through bounded
child plans" framing, but that's a planning call, not mine to make. Items
1-2 above are concrete, narrowly scoped, and directly verified; item 3 is
real but touches upgrade tooling and may warrant its own scoping discussion.

# Open item requiring the human Governor

The second submitter (previously filed as `pending-codename-20260810a`) did
not supply a codename and their record has been removed from this filing
per `28r5`. Please ask them for their preferred pseudonym; if supplied, file
a new, correctly-named record from the start (not a `supersedes` repair of
anything already published, since nothing under that placeholder name is
being published).

# Merge hold (per `28r5`'s `B-DD-LEARN-3`)

The fresh-context due-diligence evaluator (`7kjh`/`h7h4`) is required to
determine, not assume, this project's external traction, and inspects an
anonymous clone of public `main` to do so. Publishing this record on `main`
before the evaluator's source cutoff is fixed would place it in the exact
tree the evaluator inspects. Review of this filing's content may proceed
now; merge to `main` is held until either the fresh-context evaluator
freezes its own report, or an approved diligence Gate 1 plan pins a
pre-candidate `main` SHA that excludes this MR.

# Gate status

Documentation/learnings-only fast-track filing (waiver recorded in the
commit message) — no Gate 1 plan required, but this still needs its own
binding Governor Gate 2 decision before merge, and is additionally held from
merge (not from review) by the containment condition above. The v2.5.0
forward-sync (MR !1379) this record's prior draft cited as "in-progress" has
since merged; this filing was not part of that Gate 2 approval and proceeds
on its own track.
