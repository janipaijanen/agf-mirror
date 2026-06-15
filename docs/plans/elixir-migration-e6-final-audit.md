---
id: elixir-migration-e6-final-audit
status: pending-gate1
topic: elixir-migration-final-audit-gates-6-7
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: audit-only plan using repository scans and existing validators; no secrets, auth, network, or isolation changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry release-gate audit; not an upstream contribution submission
---

# Plan: E6 — Final Elixir Migration Audit for Gates 6 and 7

**Date:** 2026-06-15T20:58:32Z
**Status:** PENDING GATE 1
**Depends on:** E1+E2 complete; E3 and E5 merged and Gate 2 closed.
**Proposed branch:** `audit/elixir-migration-e6-final`
**Expected implementer:** coder-tester

---

## Intent

Produce the final audit evidence needed for governor-reviewer to close:

- **Gate 6 — Tooling defaults:** `mix agov.*` commands must not have hidden
  bootstrap defaults into `agents-govern/`.
- **Gate 7 — Release-review:** governor-reviewer records that bootstrap
  authority is no longer active governance authority.

E6 is an audit and coordination slice. It should not implement new validators
or update broad documentation unless the audit finds a concrete blocker.

---

## Scope

### Evidence to collect

Run repository scans and existing validators to confirm:

- Elixir tooling defaults point at `agf/` and `main`, not bootstrap
  `agents-govern/` authority.
- `tooling/elixir/` no longer contains `agents-govern/uv.lock` or other hidden
  Python bootstrap defaults.
- The AGF v2 distribution manifest continues to forbid `agents-govern/**`.
- The root CI configuration does not invoke Python `agov-*` commands.
- Current docs no longer present Python-era validator commands as the current
  AGF-NG command surface outside historical or E4-deferred context.
- Session resume and validator tasks run cleanly from `tooling/elixir/`.

### Expected output

Add a short audit artifact under `docs/` or `docs/coordination/` that records:

- commands run;
- relevant scan results;
- any residual `agents-govern` / `agov-validate*` references and why they are
  acceptable;
- whether Gate 6 is ready for governor closure;
- whether Gate 7 is ready for governor closure.

The Gate 2 handoff should summarize the evidence and explicitly ask
governor-reviewer to close Gate 6 and Gate 7 if acceptable.

---

## Suggested Audit Commands

```bash
git diff --check
rg -n "agents-govern|uv run agov|agents-govern/uv\\.lock|src/agents_govern|pyproject\\.toml" tooling/elixir agf .gitlab-ci.yml AGENTS.md
rg -n "agov-validate|agov-pre-tag-check|agov-validate-tag-push|agov-bump|agov-dist|agov-health" agf/docs agf/framework.md
rg -n "agents-govern/\\*\\*" agf/dist/release-manifest.yaml
cd tooling/elixir
mix compile --warnings-as-errors
mix test
mix agov.validate
mix agov.session.resume --role coder-tester --base-branch main
```

The scans may find intentional historical references. The audit must classify
them rather than requiring a zero-hit result.

---

## Non-Goals

- Do not implement E4 commands.
- Do not remove `agents-govern/`; it remains the bootstrap reference.
- Do not edit historical coordination records.
- Do not perform version bumps or tags.
- Do not make broad documentation rewrites beyond a concise audit artifact.

---

## Gate 2 Acceptance Criteria

- Audit commands run and results are recorded.
- Any residual bootstrap/Python references are classified as historical,
  bootstrap-reference, E4-deferred, or blocker.
- No blocker remains for Gate 6.
- Gate 7 readiness is stated plainly for governor-reviewer.
- A Gate 2 handoff routes the closure decision to governor-reviewer.
