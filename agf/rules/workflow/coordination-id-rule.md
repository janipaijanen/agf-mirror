---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Coordination ID: Tooling-First Generation and Filename Conventions

**Scope:** Coordination messages in `docs/coordination/`.

## Rule

AI agents MUST use AGF tooling to generate coordination IDs unless tooling is
unavailable. Humans SHOULD use the tooling. Manually authored IDs remain valid
when they pass schema and uniqueness validation.

The frontmatter `id` is the authoritative message identity. Filenames are a
stable storage and reference convention.

## Filename Conventions

| ID format | New filename convention | Notes |
|---|---|---|
| Readable ID | `<id>.md` | Preferred for new AI-authored records. The sender role is already embedded in the ID. |
| ULID | `<ulid>-<sender>.md` | Valid legacy format. The sender suffix remains useful because the ULID contains no role. |
| Legacy sequence | `<zero-padded-seq>-<sender>.md` | Historical only; do not create new sequence IDs. |
| Manual ID | `<id>.md` or `<id>-<sender>.md` | Recovery path only. Must pass schema and uniqueness validation. |

Readable IDs have the form:

```text
YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>
```

Example:

```text
20260620T183914Z-coder-tester-r7k2
```

The corresponding new readable-ID filename is:

```text
20260620T183914Z-coder-tester-r7k2.md
```

## Validator Behavior

Validators MUST block malformed front matter and duplicate frontmatter IDs.

Validators SHOULD warn, not block, when:

- a readable-ID filename repeats the sender suffix;
- a readable-ID filename does not match `<id>.md`;
- a readable ID's role slug differs from `from:`;
- a readable ID's timestamp differs from `timestamp:`;
- an AI-played agent role uses a manual ID.

Warnings preserve recovery paths while keeping normal AI operation
tooling-first.

## Historical Records

Existing coordination records are immutable. Do not rename old sequence, ULID,
or wrong-suffix readable files. If a historical filename violates the current
preferred convention, leave it in place and let validator advisories explain
the drift.
