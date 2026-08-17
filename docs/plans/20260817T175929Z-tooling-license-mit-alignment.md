---
id: tooling-license-mit-alignment
status: pending-gate1
topic: tooling-license-mit-alignment
advisory_consultation:
  - role: governor-reviewer
    status: consulted
    reason: >
      Licensing/legal artifacts are non-delegable per deputy-governor-rule's
      red list. The human governor made the call directly in chat, 2026-08-17
      — "Tooling is MIT" — after reviewing tmp/dd-20260817-license/, a 4-model
      (ChatGPT, Claude, Grok, Lumo) round-3 DD focused specifically on the
      licensing contradiction. Per coder-tester review
      20260817T185103Z-coder-tester-hj7n (B-LIC-1709-1), a chat instruction is
      the substantive decision but not by itself the required published Gate 1
      decision; `gate1_approved_by`/`gate1_approved_at` are correctly absent
      until a governor-reviewer coordination decision-record is published and
      cited here.
  - role: infosec-sentinel
    status: consulted
    reason: >
      agov validate infosec clean at implementation head. No credential,
      trust-boundary, schema, or validator behavior change — pure license
      metadata correction plus one new adopter-facing docs page.
  - role: contribution-auditor
    status: consulted
    reason: >
      Non-self coder-tester/contribution-auditor review
      20260817T185103Z-coder-tester-hj7n at exact head fba42f2e reviewed all
      six changed files, plan metadata, and adopter-facing licensing claims.
      Recommendation: BLOCK, 4 findings (B-LIC-1709-1 through -4), all
      accepted and being fixed at a new exact head. This corrects an earlier,
      inaccurate "waived, review requested for the exact same head" claim
      that hj7n itself flagged (B-LIC-1709-2) — the referenced request
      (pdjj/MR !1710) named head 7b9b69c1, but the branch had already moved to
      fba42f2e by the time this plan was pushed, so "same head" was false.
      A fresh exact-head re-review is required after these fixes before this
      plan can be presented for Gate 1.
---

# Tooling license: align Rust/Container metadata to MIT

- Timestamp: 2026-08-17T17:59:29Z (OS clock, `date -u`)
- Author: planner-architect (Claude)
- Substantive licensing direction: direct governor instruction, this
  session, 2026-08-17 — "Tooling is MIT" — after reviewing
  `tmp/dd-20260817-license/` (4-model round-3 DD specifically on the
  licensing question). This is a licensing/legal artifact, non-delegable
  per `deputy-governor-rule.md`'s red list; the human governor made the
  call directly, which is the only path authorized to make it. Per
  coder-tester review `20260817T185103Z-coder-tester-hj7n`
  (B-LIC-1709-1), this chat instruction is the substantive decision but
  is **not itself the required published Gate 1 authorization** — Gate 1
  remains pending until a governor-reviewer coordination decision-record
  is published and cited in this plan's frontmatter.

## Intent

Root `LICENCE` has always stated tooling is MIT. `tooling/rust/Cargo.toml`,
`Containerfile.agov`, and `Cross.toml` have instead declared
`CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial` since the Rust tooling
was introduced — a live contradiction independently confirmed by this
session's own repo checks and by four external DD models across two rounds
(ChatGPT gave the most precise "minimal correction" list; Grok's round-3
answer still states tooling-is-MIT as settled fact without checking
Cargo.toml, repeating the same blind spot named in `spgp`/2026-08-17).
The governor has now resolved the ambiguity in the direction the root
`LICENCE` already claimed: **tooling is MIT, full stop** — the Rust/
Container metadata is what's wrong and gets corrected to match, not the
reverse.

## Scope confirmed by direct repo search before writing this plan

```
grep -rn "CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial" tooling/rust/
  tooling/rust/Cargo.toml:1   (SPDX header comment)
  tooling/rust/Cargo.toml:17  (license = "...")
  tooling/rust/Containerfile.agov:1   (SPDX header comment)
  tooling/rust/Containerfile.agov:21  (org.opencontainers.image.licenses=...)
  tooling/rust/Cross.toml:1   (SPDX header comment)
  tooling/rust/deny.toml:28-29 (allow-list entries — informational, see below)
```

`agf/LICENSE-MIT` already exists (canonical tooling licence text, per
`LICENCE`'s own pointer) — no new licence text file needed. All five Rust
crates use `license.workspace = true`, so the workspace-level `Cargo.toml`
change propagates to all of them automatically; no per-crate edits needed.
`MIT` is already in `deny.toml`'s permissive allow-list, so `cargo deny
check licenses` passes without a `deny.toml` change — no dependency is
newly disallowed by this change.

## Affected files (5)

1. `tooling/rust/Cargo.toml` — SPDX header comment + `license =` field → `MIT`.
2. `tooling/rust/Containerfile.agov` — SPDX header comment + OCI
   `org.opencontainers.image.licenses` label → `MIT`.
3. `tooling/rust/Cross.toml` — SPDX header comment → `MIT`.
4. `agf/docs/site/docs/adopt/licensing.md` — new adopter-facing page
   (added after `agov validate adopter-docs` correctly flagged this
   change as adopter-relevant and requiring either downstream-facing
   docs or an explicit waiver; a real licensing page is more honest than
   waiving, and is exactly what all four DD models recommended).
5. `agf/docs/build/mkdocs.yml` — nav entry for the new page.

No `CHANGELOG.md` entry: checked the repo's actual convention before
adding one — every existing entry lives under a version heading added at
release time (`release-via-merge-request-rule.md`'s workflow), there is
no `Unreleased` section, and no other fast-track fix this session added
one. Will be captured under whichever version heading the next release
adds. (Note: `agov validate adopter-docs` checks `agf/CHANGELOG.md`
specifically for a waiver marker, not the root `CHANGELOG.md` — this
plan satisfies that check via real downstream-facing doc coverage
instead of a waiver.)

## Explicitly out of scope for this bounded fix

- **Per-`.rs`-file SPDX headers** (the 51 files with zero headers today).
  Separately tracked (Tier 1 in the DD reconciliation, `hqgb`/`2kad`) —
  now unblocked by this decision, but a distinct, larger mechanical change
  deserving its own commit/MR rather than inflating this one.
- **Bundling `agf/LICENSE-MIT` text into release archives/containers.**
  ChatGPT's DD flagged this as desirable (a consumer of only the binary
  archive should see the license without reading the repo root), but the
  actual archive-creation step lives outside `release-assets.sh` (that
  script only lists/verifies already-built `dist/` contents) and touching
  the real packaging step is release-pipeline surgery that deserves its
  own bounded review, not a rider on a licence-metadata fix.
- **`deny.toml`'s `CC-BY-SA-4.0`/`LicenseRef-AgentsGovern-Commercial`
  allow-list entries.** Technically now unused by our own crates once this
  change lands, but removing them isn't required for correctness (an
  unused allow-list entry isn't a defect) and risks being wrong if any
  other in-tree Cargo package still needs them. Leaving as-is; a cleanup
  candidate for whoever next touches `deny.toml`, not blocking this fix.
- **Root `LICENCE`'s prose.** Already correct — states tooling is MIT.
  No change needed.
- **CONTRIBUTING.md commercial-relicensing grant clarity** (ChatGPT's
  secondary finding). Real, but a distinct contribution-process question,
  not part of "what license is the tooling" — tracked separately.

## Test strategy

- `cargo metadata --manifest-path tooling/rust/Cargo.toml --format-version 1
  | jq -r '.packages[] | select(.name | startswith("agov")) | .license'`
  — confirm all five workspace crates report `MIT`.
- `cd tooling/rust && cargo deny check licenses` — confirm clean pass
  (MIT already allowed; no new denial).
- `agov validate infosec` — confirm clean (SPDX/licence field is one of
  its checks per this session's established pattern).
- `git diff --check` — no whitespace errors.
- Rebuild the Containerfile label mentally against `agov-oci-image.sh`'s
  consumption of it, if any (grep confirmed no script reads the value
  besides the Containerfile itself setting it — no further script change
  needed).

## Risk / rollback

Low risk: pure metadata correction, no code-path change, no schema change,
no migration. Rollback is a plain revert of the 3-file diff if the governor
reconsiders. Per `always/downstream-impact-rule.md`: classification is
**BREAKING** for any consumer that had already indexed the CC-BY-SA/
commercial declaration as authoritative (SBOM generators, procurement
scanners, cargo-metadata-reading tools) — but per this session's own DD
synthesis, this is correcting a metadata *defect* toward the licence the
project always intended and always stated at the root, not changing intent.
Migration note: none needed beyond the CHANGELOG entry — there are no known
external consumers of the old (contradictory) declaration to migrate, per
`deprecation-policy-rule.md`'s Phase 0 (pre-customer) framing.

## Not done by this plan

Does not open the per-`.rs`-file SPDX header work, does not touch release
packaging, does not update `deny.toml`. Each is a distinct follow-up,
named above, not silently deferred.
