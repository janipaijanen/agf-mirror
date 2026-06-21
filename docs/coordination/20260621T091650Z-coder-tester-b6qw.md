---
id: 20260621T091650Z-coder-tester-b6qw
kind: completion
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T09:16:50Z
immutable: true
references-to:
  - 20260621T090730Z-coder-tester-k4mj.md
topic: docs-commissioning-learning
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
completion:
  status: complete
  completed_task:
    - Documented AGF commissioning as a useful first adopter learning.
    - Updated the starter learning template to use commissioning as the initial sample learning.
    - Added warnings not to invent fixed sequential learning IDs.
  artifacts:
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/start/adopter-faq.md
    - agf/docs/site/docs/concepts/evidence-and-learnings.md
    - agf/templates/learning-record.yaml
    - agf/templates/README.md
---

# Completion: commissioning as first adopter learning

Updated the adopter docs so the first learning can be the commissioning run
itself. The docs now ask adopters to record whether the assistant understood
the commissioning prompt, kept AGF-owned files under `agf/`, created only a
small loader if needed, and left clear decisions for the governor.

The update also says upstream is interested in sanitized commissioning
learnings when they reveal adoption friction, validation failures, confusing
docs, or unusually smooth automation.

Important: the docs do not add a fixed learning ID field. They warn against
inventing fixed sequential learning IDs unless a downstream project has a
non-colliding local convention or tooling.

## Verification

- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `git diff --check` passed.
- Scan confirmed there is no fixed `id:` learning example.
