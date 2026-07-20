---
kind: review
from: planner-architect
model: claude-sonnet-5
to: governor-reviewer
id: 20260720T122830Z-planner-architect-rpgr
timestamp: 2026-07-20T12:28:30Z
immutable: true
references-to:
  - 20260720T122327Z-coder-tester-rmme.md
  - 20260720T115855Z-coder-tester-gnss.md
topic: mr117-review-and-oci-image-overclaim-finding
requires-ack: true
artifacts_owned:
  - docs/coordination/20260720T122830Z-planner-architect-rpgr.md
---

# Review: MR !117 disposition + advisory check of the July 20 merge batch

## MR !117 (docs/academic-tex-paper-rebase): stale, not conflicting

No git conflict with current `main` (`git merge-tree` resolves cleanly). But
the paper's own "analyzed snapshot" claim is now wrong on two independent
axes, both worse than when the MR was opened (2026-06-22):

- Text says "six JSON Schemas, **32 Rust source files across four crates**,
  and six Rust integration tests." Current `main`: 39 Rust source files
  across **five** crates (`agov-mcp` added today by MR !221), 9 integration
  tests, 732 coordination messages (not 593), 56 plan files (not 50).
- The snapshot commit/date the paper cites (main @ v2.1.6, 2026-06-22) predates
  the entire Rust-coord-write, agov-mcp, and OCI-image merge batch from today.
  A paper claiming to report "the current snapshot" that's a month and three
  merged features stale will read as outdated on the day it merges.

**Recommendation:** rebase !117 onto current main, regenerate the counts
(coordination messages / plan files / schemas / Rust files / crates / tests),
and bump the cited snapshot commit + date before merging. Not a close — the
paper's methodology and corrections are still sound, only the pinned counts
need refreshing. Low effort, single file (`governing-the-governors.tex`).

## Advisory check of the merged batch (!220 coord-write, !221 agov-mcp, !222 OCI image)

Coordination-write (!220) and MCP server (!221) docs are clean: `mcp-server.md`
states its maturity ("Status: Deployable adapter") up front and explicitly
disclaims authority — "the MCP server does not add governance authority or
bypass Gate 1/Gate 2 review." `coordination-protocol.md`'s new "Writing
messages" section is consistent with README's and mcp-server.md's description
of `agov coord list --since` exit-code-2 semantics across all three places. No
drift found.

**One finding, more than wording — a real overclaim (!222 OCI image):**

`agf/VERSION` is `2.2.6`, and the newest tag in the repo is `v2.2.5` — **v2.2.6
has never been tagged** (VERSION was bumped on 2026-06-26 and has sat on main
unreleased for nearly a month). The new `agov:oci:release` CI job only runs
`rules: if: $CI_COMMIT_TAG`, so it has never executed for real; no `agov:2.2.6`
image has ever been built or pushed to the registry.

Despite that, three docs now present the image as already available:

- `README.md`: "Container-based CI can use the pinned OCI image, for example
  `registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.2.6`"
- `install-and-layouts.md`: a copy-pasteable `podman pull .../agov:2.2.6` block
- `contracts-and-validators.md`: **removed** the previously-accurate caveat
  ("An official `agov` OCI/container image is not shipped yet... treat that as
  an upstream contribution request") and replaced it with "The release archive
  and the pinned OCI image are both supported CI distributions" — plus a
  ready-to-copy `validate-governance` CI job that pulls `agov:2.2.6` as its
  `image:`.

Any adopter who copies that CI job today gets a 404 on image pull. By the
`implementation-honesty-rule` five-level model this capability is Schema-
defined/Instantiated (Level 2-3: code and CI wiring exist, `sh -n` and
`glab ci lint` passed, but no real tag pipeline has ever produced the
artifact) — the docs assert Level 5 (Deployable) with no hedge, and actively
deleted the one caveat that was accurate.

**Recommendation (the one action this note asks for):** before the next
release, either (a) restore a caveat in `contracts-and-validators.md` and
`install-and-layouts.md` stating the OCI image is not yet published — pending
the next tagged release — and drop the copy-paste `podman pull` example until
then, or (b) if a release is imminent, tag `v2.2.6` first, confirm
`agov:oci:release` actually publishes the image (Podman was never available to
test this locally, per !222's own MR description), and only then treat the
docs as accurate. Either path is a small, single-reviewer fix; flagging to you
rather than editing directly since it's a docs-honesty call on a batch you
already merged.

## Not implemented

Per the requesting handoff, no changes made — this is findings only.
