---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260723T211125Z-contribution-auditor-7g56
timestamp: 2026-07-23T21:11:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T205341Z-contribution-auditor-hpha
  - 20260723T205942Z-contribution-auditor-7pjs
topic: v241-mermaid-pin-and-vendoring-sequence
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Pin the v2.4.1 custom Mermaid import exactly to 11.16.0 while implementing useMaxWidth false, then complete ordinary-page and AT evidence.
    - Keep Mermaid self-hosting out of the v2.4.1 critical path unless exact-pinned CDN loading or initialization prevents reliable rendering.
  must_read:
    - docs/coordination/20260723T205341Z-contribution-auditor-hpha.md
    - docs/coordination/20260723T205942Z-contribution-auditor-7pjs.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Governor sequencing instruction: exact Mermaid 11.16.0 pin in v2.4.1; self-hosting tracked separately as issue #63

The governor instructed the release-oriented disposition in the active session:

1. **v2.4.1:** use an exact Mermaid `11.16.0` CDN import, implement the
   already-authorized responsive configuration, and complete the required tests.
2. **Immediate follow-up:** vendor Mermaid into the GitLab Pages assets and
   remove the runtime CDN dependency.
3. **Release priority:** do not delay v2.4.1 solely for vendoring unless the
   exact-pin integration reveals CDN or initialization instability that
   prevents reliable rendering.
4. Keep the automated-browser empty-container observation separate from CDN
   causation; current evidence does not prove the CDN caused it.

## v2.4.1 exact import

Use the exact version in the supported Material custom-Mermaid integration:

`https://cdn.jsdelivr.net/npm/mermaid@11.16.0/dist/mermaid.esm.min.mjs`

Do not use `@11`, `latest`, or an unversioned/moving alias. The retrieved entry
module on 2026-07-24 was 29,775 bytes with SHA-256
`b81c2c7ac0d81466022ea895c6701978ccb742e12e423dd8ae4a5a45cc259633`.
This is provenance for the observed entry file, not an integrity assertion for
the full transitive import graph.

Apply the smallest `useMaxWidth: false` configuration supported by the five
diagram types, expose the configured Mermaid object to Material as required,
and verify ordinary full loads plus instant navigation. Preserve theme and
`accTitle`/`accDescr` wiring. Complete the named screen-reader/browser/OS smoke
test or route an explicit waiver request; the pin does not clear that blocker.

## Follow-up separated

Created GitLab issue #63, **Vendor exact Mermaid distribution into GitLab Pages
assets**, with dependency-graph, license, provenance, offline-build,
initialization, accessibility, and failure-visibility acceptance boundaries.

Inspection confirms `mermaid.esm.min.mjs` imports relative files under
`./chunks/mermaid.esm.min/`; the future vendoring task must distribute and
verify the complete required browser graph, not copy only the entry module.

Resume MR !458 first. Issue #63 begins after its exact-pin correction and Gate 2
evidence unless the explicit escalation condition above occurs.
