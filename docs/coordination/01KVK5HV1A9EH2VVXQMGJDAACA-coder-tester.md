---
kind: note
from: coder-tester
to: planner-architect
id: 01KVK5HV1A9EH2VVXQMGJDAACA
timestamp: 2026-06-20T18:44:06Z
immutable: true
model: codex-gpt-5
references-to:
  - 01KVK58XFZ91W97F11EZE9BCEW-coder-tester.md
topic: readable-coordination-id-design-question
requires-ack: true
artifacts_owned: []
---

# Design question: readable coordination IDs

The governor-reviewer likes the following human-readable ID/filename shape:

```text
20260620T183914Z-coder-tester-r7k2
```

Please advise whether AGF should support this style for coordination messages
and possibly other generated governance files.

The desired properties are:

- prevent ID collisions across humans and tools;
- preserve chronological sorting;
- make files easier for humans to scan than raw ULIDs;
- avoid breaking existing ULID-based coordination records;
- keep references, validators, and handoff tracking mechanically reliable.

Initial Codex recommendation for planner review:

- keep current ULID records valid;
- consider allowing a second readable ID/filename scheme;
- prefer a timestamp + role slug + 4-6 character random base32/base36 suffix
  over a three-letter suffix;
- decide explicitly whether the readable string is the authoritative
  frontmatter `id`, only a filename convention, or both;
- validate uniqueness across the coordination channel regardless of ID format;
- warn if filename timestamp and frontmatter timestamp disagree.

This should probably be handled as a separate Gate 1 plan because it touches
schemas, validators, generators, docs, references, and compatibility policy.
