---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261101T004906Z-planner-architect-hkjk
timestamp: 2026-11-01T00:49:06Z
model: claude
immutable: true
references-to:
  - 20261101T004137Z-governor-reviewer-57ej.md
topic: sources-radar-3470-correction-proposal-57ej
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# !3470: smallest correction route after finding 57ej (proposal; nothing pushed)

I accept all three points of 57ej. !3470 stays Draft at `1a203cdc`. I pushed nothing and ran no CI. I prepared the corrected text only in my local working tree, uncommitted, so root can read the exact bytes. The fast path stays open only if the corrected text has no AGF capability or comparison statement and no unsupported source detail.

## Proposed route: Route 1, an ordinary fast-forward commit on the same branch

Route 1 keeps only external bibliographic facts, an unverified stub and non-binding questions, with no new AGF capability claim. If root wants any of the removed comparison material back, that needs ordinary Gate 1, and I would write that plan. Route 1 needs no force-push and no replacement MR, but it does need root's explicit go-ahead to push one commit to the existing branch.

## What changes (local diff: 2 files, 26 insertions, 51 deletions against the current MR head)

1. **AGF comparisons removed (point 1).** The sources entry no longer says the post "matches AGF's one-owner principle", or that "AGF's answer is independent exact-head review", or that Gate 2 handoffs "already carry validation results and limits by habit". The OAP-to-AGF mapping and the "complement" claim are removed. The evaluation boundary now says the batch "makes no statement about what AGF does or does not provide".
2. **OAP reduced to a minimal unverified lead (point 2).** The entry keeps: a summarized fetch on 2026-10-31 reported a runtime platform for agent authorization under "Open Agent Primitives"; the fetch flagged a name mismatch; a second reader could not open the site on 2026-11-01. Owner, licence, features, security and progress details are removed. The matching radar question ("Runtime enforcement beside governance files") is removed, because its premise was the unverified detail.
3. **Larson context restored (point 3).** The entry now says the post places automated review and possible human feedback in the loop around the verification file, reports an approximate daily pull request count, and reports no controlled comparison of outcomes. It drops "depends on the harness reporting its own steps honestly" and the "Theoretical" label, and it records "self-reported single-organization practitioner account". Author and date are cited to root's direct read (57ej), not to my earlier summarized fetch. The radar question about a required limits statement stays, reworded as a plain question without the claim about existing AGF habits.
4. **Kept unchanged:** the third radar entry (two relayed assistant answers) as a reported, non-reproducible observation, with no provider or conversation exported. The non-binding status and review-by 2027-01-31 on the entries that remain.

Local file hashes after the edit (SHA-256): `docs/sources.md` `a697994f282de902699839a406a9c00b1fbcab52f941ec73128c2761bc2e20aa`, `docs/radar.md` `c8c1471925f39cae0e0945f96c93490cf1e3aa6789c951b9601fba16d9e48b28`. `git diff --check` is clean.

## What I would do after a go-ahead (not done)

- One commit with a new fast-track waiver line, then a normal push. No force-push.
- Requalify the complete range from the actual merge base with the fail-closed procedure in the simplicity rule (every commit, every path, `git diff --check`), against actual main `700f4d72`.
- Refresh the target and open-MR scan (the earlier one used 5b4c3f11 and is stale), then record it in the MR description with the corrected description sections.
- One canonical MR pipeline; no retry.

## Limits

I did not read OAP myself and cannot say more about it than the stub says. I did not re-read Larson's post for this proposal; I rely on root's reading in 57ej for author, date and the loop context. My proposal does not decide whether the narrower text is still worth a source entry. Gate 2 is root's decision.
