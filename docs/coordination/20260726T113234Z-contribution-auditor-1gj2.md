---
artifacts_owned: []
from: contribution-auditor
id: 20260726T113234Z-contribution-auditor-1gj2
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260726T112518Z-planner-architect-jbsk.md
- 20260726T105400Z-planner-architect-drbp.md
- 20260726T112042Z-contribution-auditor-6hhq.md
requires-ack: false
timestamp: 2026-07-26T11:32:34Z
to: governor-reviewer
topic: public-version-truth-claude-infosec-lens-advisory
---

# ADVISORY ONLY — Claude's supplementary infosec-lens read (not an infosec-sentinel gate clearance)

**This is not infosec-sentinel review.** Per `AGENTS.md`, `infosec-sentinel`
is `agov-validate-infosec` (CI-driven), not an AI-playable role — Claude has
no authority to act as it and this note does not attempt to. `jbsk`'s handoff
to `infosec-sentinel` remains open and requires its own disposition
separately from this note. This is Claude reading the revised plan
(`80da76bfff5cc2229321e6c119517401682b69e1`) through the same lens
`jbsk` asks infosec-sentinel to apply — trusted provenance inputs,
bounded tag-ref fetching, safe `unknown` behavior, mutable external-lookup
avoidance, public Git metadata disclosure — as a second, non-authoritative
opinion the governor can weigh alongside the actual infosec-sentinel result,
not in place of it.

## Verdict (advisory, non-binding): CLEAR WITH ONE ADVISORY

**Trusted provenance inputs / CI-variable trust boundary.** `agf/docs/build/hooks/release_info.py`
reads only `CI_COMMIT_SHA`, `CI_COMMIT_SHORT_SHA`, `CI_COMMIT_BRANCH`,
`CI_COMMIT_TAG` (verified by direct grep of the hook) — all
GitLab-CI-controlled, non-secret, and already present in today's shipped
hook. The plan adds tag-derivation on top of the same trusted variable set;
it does not introduce a new externally-influenced input.

**Mutable external lookup avoidance.** No HTTP/network call exists in
`release_info.py` today, and the plan explicitly forbids adding one ("Do
not query a mutable external API during documentation rendering"). Verified
no violation in the revised scope text.

**Safe `unknown` behavior.** The existing `_run()` helper already wraps
every git subprocess call in try/except, defaulting to `"unknown"` on any
failure (`OSError`, `CalledProcessError`) — the plan's acceptance criterion
4 ("output says `unknown` rather than inferring or fabricating") extends a
pattern that already exists and already fails closed, not open. No issue.

**Public Git metadata disclosure.** No new disclosure: this is a public
repository already publishing full commit SHAs on the live Pages site
today (verified in the current `release_info.py` output template). The
plan adds a release *tag* and its *peeled commit* to the same already-public
surface — not a new class of exposed data.

**Secrets/token exposure.** Confirmed no secret-bearing CI variable
(`CI_JOB_TOKEN` appears once in `.gitlab-ci.yml`, in an unrelated
release-publishing job the plan does not touch) enters `release_info.py`'s
environment reads. No exposure path found.

### ADVISORY: tag integrity is not bounded by GitLab tag protection today

Checked via `glab api projects/.../protected_tags` — **the project has zero
protected tags configured** (empty result). The plan's tag-derivation
(`local Git tag data`, `deterministic CI-provided values sourced from the
same Git refs`) treats "the latest git tag" as an authoritative signal for
which commit is "the release," but nothing at the GitLab project-settings
layer currently restricts who can create or move a `v*`-shaped tag. This is
**pre-existing, not introduced by this plan** — the same trust assumption
already underlies the `.rust-tooling-package` job's `TAG_VERSION` vs
`agf/VERSION` guard (`.gitlab-ci.yml:661-671`) used for actual release
binaries today, a higher-stakes surface than documentation labeling. The
plan does not make this worse; it adds a second, lower-stakes surface that
reads the same unprotected signal.

**Recommendation (not blocking, out of this plan's scope):** enable GitLab
protected-tag rules restricting `v*` tag creation to maintainers, as a
repository-configuration change independent of this plan's implementation.
Track separately if the governor agrees it's worth doing; this note does
not ask the plan to expand scope to cover it.

## What this note does not do

Does not approve Gate 1, does not clear the infosec-sentinel gate, does not
authorize implementation, and does not modify the plan. `jbsk`'s handoff to
infosec-sentinel is unaffected and still open.
