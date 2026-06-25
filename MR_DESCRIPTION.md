# fix: validate upstream learning packets

Branch: `fix/upstream-learning-validation-ci-docs`

## Summary

Addresses GitLab issues `#18` and `#19` from upstream-submittable adopter
learning records.

| Issue | Disposition |
|---|---|
| `#18` CI OCI image learning | Ingested as learning `20260625T071500Z-c1im`; documented current status that release archives are supported and an official OCI image is a proposed future distribution improvement. |
| `#19` validation prerequisites learning | Ingested as learning `20260625T072500Z-svch`; implemented explicit-file learning validation and documented it. |

## Changes

- Adds both learning records to `agf/learnings/foundry.yaml`.
- Adds `agov validate learnings <file-or-dir>...` for validating
  upstream-submittable learning packets outside the default `agf/learnings/`
  register.
- Adds a regression test for explicit learning file validation.
- Updates contracts/validators docs, adopter FAQ, and environment-variable
  reference.
- Files a Gate 2 handoff:
  `docs/coordination/20260625T072349Z-coder-tester-shad.md`.

## Validation

```
cargo test --manifest-path tooling/rust/Cargo.toml
cargo test --manifest-path tooling/rust/Cargo.toml -p agov --test validate
./tooling/agov validate
./tooling/agov validate infosec
./tooling/agov validate learnings agf/learnings/foundry.yaml
git diff --check
```

Known warnings:

- Existing bare timestamp learning IDs in historical records.
- Existing .claude/rules assistant drift warning.

## Review Checklist

- [x] Gate 2 decision: 20260625T074918Z-governor-reviewer-qq17.md — APPROVED.
- [ ] agov validate learnings <file> behavior is acceptable for issue #19.
- [ ] OCI-image wording for issue #18 is clearly framed as not shipped yet.
- [ ] Learning records preserve anonymization and evidence discipline.
