---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260621T200440Z-coder-tester-bs6d
timestamp: 2026-06-21T20:04:40Z
immutable: true
model: codex-gpt-5
references-to:
  - docs/plans/agents-govern-2.0-directory-structure.md
topic: disclaimer-file-and-docs-consultation
requires-ack: true
artifacts_owned:
  - docs/coordination/20260621T200440Z-coder-tester-bs6d.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critique whether AGF should add a canonical disclaimer file now.
    - If yes, propose the exact destination, docs nav location, and scope.
    - Keep the wording human-readable and avoid duplicating license text.
  must_read:
    - README.md
    - agents-govern/DISCLAIMER.md
    - agf/ADOPTING.md
    - agf/docs/build/mkdocs.yml
    - docs/plans/agents-govern-2.0-directory-structure.md
  do_not_touch:
    - license terms or copyright ownership text
    - commercial licensing language
    - unrelated Slice B documentation reordering
---

# Handoff: disclaimer file and docs inclusion

The governor asked to add a disclaimer file and include it in the docs, but
also asked us to argue against it and craft it together.

## Current evidence

There are already partial disclaimer surfaces:

- `README.md` has a short "Safety and independence" section:
  AGF is not legal, security, or compliance advice and must be adapted to the
  user's context.
- `agents-govern/DISCLAIMER.md` exists in the bootstrap reference and is much
  fuller: no warranty, limitation of liability, advisory guidance, user
  responsibility, no professional advice.
- `agf/ADOPTING.md` links to `docs/governance/disclaimer.md`, but that file is
  not present in the AGF v2 public site layout.
- `docs/plans/agents-govern-2.0-directory-structure.md` maps legacy
  `DISCLAIMER.md` to `docs/governance/disclaimer.md`.

## Argument against adding a broad new disclaimer

A standalone disclaimer can be useful, but it can also make the public docs
feel defensive and less trustworthy if it becomes a wall of legal language.
The project already has:

- license/copyright text in the MkDocs footer;
- a README safety note;
- concrete risk/exit/adoption pages;
- bootstrap disclaimer text under `agents-govern/`.

Adding another long disclaimer without pruning or linking the existing surfaces
risks duplication, drift, and a false sense that the framework has "handled"
legal/security/compliance concerns by prose alone. It could also pull the docs
voice away from practical adoption guidance into generic liability boilerplate.

## My proposed middle path

Add one canonical, adopter-facing disclaimer page, but keep it short and
specific to AGF:

- source file: `agf/docs/site/reference/disclaimer.md`
- nav location: `Reference -> Disclaimer`
- short links from `README.md` and `agf/ADOPTING.md`
- no changes to license terms;
- no claim that the page is legal advice;
- no repetition of full CC BY-SA warranty/legalcode text, only a pointer.

Recommended sections:

1. **No professional advice** — AGF is not legal, security, compliance,
   privacy, or operational-risk advice.
2. **Use on authorized systems only** — users need permission to run agents,
   validators, CI, and repository-changing procedures.
3. **No warranty / limitation pointer** — framework content is provided as-is;
   see CC BY-SA 4.0 warranty/liability sections and applicable tooling
   licenses.
4. **Your responsibility** — evaluate fit, adapt rules, back up before risky
   operations, review AI output, comply with local obligations.
5. **AI and tooling limits** — validators and agents are aids, not proof of
   correctness, security, compliance, or absence of defects.
6. **Independence** — no endorsement/dependency relationship with tool,
   model, or runtime vendors unless explicitly stated.

## Questions for planner-architect

Please challenge this before implementation:

- Should the canonical public page live under `reference/disclaimer.md`, or
  should we create `docs/governance/disclaimer.md` as the older plan suggested?
- Should it be in the public nav, footer-adjacent reference, or linked only
  from README/ADOPTING?
- Should we copy/adapt the bootstrap `agents-govern/DISCLAIMER.md`, or write a
  shorter v2 page from scratch?
- What exact wording avoids overclaiming and still protects first-time users
  from misunderstanding AGF as professional advice?

My bias: implement the middle path above. It closes the missing-file gap,
gives adopters one canonical page, and avoids turning every docs page into a
disclaimer billboard.
