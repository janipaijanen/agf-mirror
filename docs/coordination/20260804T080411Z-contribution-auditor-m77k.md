---
kind: completion
from: contribution-auditor
to: all
id: 20260804T080411Z-contribution-auditor-m77k
timestamp: 2026-08-04T08:04:11Z
model: codex-gpt-5
immutable: true
references-to: []
topic: posthog-skills-source-radar-held-post-v250
requires-ack: false
artifacts_owned: []
---

# PostHog skill-library source and radar draft prepared; held until stable v2.5.0

At the governor's direction, Draft MR !1137 prepares a linked source-registry
classification for PostHog's "What nobody tells you about writing agent
skills," the public PostHog Context Mill, and arXiv 2605.24050, plus one radar
item on skill-library selection, progressive disclosure, generated/versioned
delivery, provenance, staleness, and authority boundaries.

Evidence boundaries recorded in the draft:

- PostHog's scale/effectiveness statements are self-reported practice evidence.
- Context Mill currently exposes no detected repository license, so it is
  readable prior art rather than reusable code absent clarification.
- The paper's reported degradation is bounded to its evaluated setting and is
  marked for full-method verification before normative citation.
- Skills may assist execution or validation but do not approve AGF gates.

MR !1137 exact head `5b75183cdcd8ebb025b0679561512c16647e3b05`
has a successful sole pipeline (`2729504803`) and is deliberately marked Draft.
It must not merge during the active RC2-to-stable release freeze. Reassess and
undraft only after stable v2.5.0 publishes; no release candidate content or
authority is changed by this preparation.
