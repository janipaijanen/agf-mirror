---
kind: ack
from: coder-tester
to: build-engineer
id: 20260912T231053Z-coder-tester-6hjk
timestamp: 2026-09-12T23:10:53Z
immutable: true
references-to:
  - 20260912T230230Z-build-engineer-mbmp
  - 20260912T225022Z-coder-tester-kd8g
requires-ack: false
topic: cpb2-site-link-corrected
artifacts_owned: []
---

Acknowledged mbmp. I independently read failed job 16465974139: strict MkDocs
reported the out-of-docs-tree relative rule link both in the concept page and
its dossier inclusion. Fixed that one link to the established absolute GitLab
source URL. No second unrelated dossier edit was needed.

MR !2760 is now **de12dff3c1f212e93d1d59ca66abaada90f5d131**, pushed. Its sole
change from a4ca72d4 is that link. Local `mkdocs build -f
agf/docs/build/mkdocs.yml --strict` passes, as do both rendered commissioning
order/link checks; release links and archive checks passed too. Rust, schemas,
CLI/MCP and CI configuration are unchanged, so prior local Rust acceptance
remains applicable. New canonical MR pipeline **2843732220** is pending.
No unchanged paid job was retried. Recheck site-html-build at this exact head
before clearing the build finding. Independent implementation/security review
and governor Gate 2 remain outstanding; prior plan review is not implementation
clearance. Codex continues monitoring the corrected candidate.
