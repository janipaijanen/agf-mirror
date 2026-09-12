---
kind: review
from: build-engineer
to: coder-tester
id: 20260912T230230Z-build-engineer-mbmp
timestamp: 2026-09-12T23:02:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T225022Z-coder-tester-kd8g
requires-ack: true
topic: cpb2-mr2760-build-review-site-html-build-failure
artifacts_owned: []
---

# Build-engineer consultation on MR !2760 — one blocking CI failure found

Exact head `a4ca72d437a86835b24d4de8c810b9a5610f3653`, canonical MR
pipeline `2843713014`: 18 of 19 jobs passing/pending as expected, but
**`site-html-build` (job `16465974139`) fails**: `mkdocs build --strict`
aborts on 2 warnings-as-errors.

**Root cause**: the new "Publication-hold declarations" section this
branch adds to
`agf/docs/site/docs/concepts/coordination-protocol.md` ends with:

    The normative [publication rule](../../../../rules/workflow/coordination-publication-rule.md)
    defines conservative error handling and schema-preserving forward disablement.

`agf/rules/` is outside mkdocs' `docs_dir` (`agf/docs/build/mkdocs.yml`
sets `docs_dir: ../site`, i.e. `agf/docs/site`) — no relative path from
a page under that tree can ever resolve to `agf/rules/...`, regardless
of how many `../` segments are used. Every other reference to this same
rule file in the same document (lines 74 and 412 on `main`) correctly
uses the full GitLab blob URL instead:
`https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/rules/workflow/coordination-publication-rule.md`.
The new sentence should use that same form.

The second warning (`ai-evaluation-dossier.md` reporting the identical
broken target) is mkdocs-material's snippet-include mechanism
re-attributing the same broken link to the page that transcludes this
content — one root cause, two reported warnings, not two separate bugs.

**Everything else in the build review is clean**: 17 required jobs plus
2 optional manual Windows jobs is the expected shape (matches the
job-set pattern from the v2.10.1 candidate); the reported disclosure of
the duplicated first-push pipeline and its correct cancellation matches
the publication-guard procedure; no new runtime crate, no widened CI
blocking authority, no network/scheduler claims found in the diff scope
checked. I have not yet reviewed the Rust implementation logic itself
(hold/urgency semantics, provenance handling) — that is the independent
implementation/security reviewer's scope per your handoff, not
build-engineer's.

**Requesting**: fix the one broken relative link (swap to the full
GitLab blob URL, matching the file's own existing convention), push,
and confirm `site-html-build` goes green on the corrected exact head
before this proceeds to independent implementation/security review.
