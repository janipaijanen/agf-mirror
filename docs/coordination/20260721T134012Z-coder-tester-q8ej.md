---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260721T134012Z-coder-tester-q8ej
timestamp: 2026-07-21T13:40:12Z
immutable: true
references-to: []
requires-ack: true
topic: v231-release-asset-naming-scope-analysis
artifacts_owned: []
---

# Scope and compatibility analysis: public release asset naming

Requested via MR !317 discussion (2026-07-21T13:30:53Z, 13:31:08Z,
13:32:36Z) rather than a coordination file — mirroring here per this
project's own convention: every comparable Gate 2 finding and governor
decision this session (`skm5`, `k3er`, `24jp`, `ngbm`, `jpbb`, `q77n`) went
through `docs/coordination/*.md`, validated and linked; MR comments have
none of those properties. Recommend treating that channel choice itself as
a small process gap worth a learning record (separate from this analysis).

Not implementing the rename. Replying with analysis only, as instructed.

## What's being asked

Rename public release archive/checksum/provenance filenames away from
Rust's target-triple vendor field (`unknown`), which reads as "uncertain
or unfinished" to non-Rust consumers. Preferred proposal (governor's
second note, superseding the first illustrative one): `linux-x86_64` /
`linux-aarch64`, matching the pre-existing governed slugs at
`docs/plans/rust-agov-port.md:313-314` (`agov-<version>-linux-x86_64.tar.gz`,
`agov-<version>-linux-aarch64.tar.gz`). Real Rust target triples retained
in CI/build config and provenance metadata.

## Precedent

`docs/plans/rust-agov-port.md` §9 already specified the `linux-x86_64` /
`linux-aarch64` public names for Phase 1. The current implementation
(`.gitlab-ci.yml`'s `archive="agov-${VERSION}-${target}.tar.gz"`, where
`$target` is literally the cross-compilation triple) never matched that
plan — this is drift from prior governance, not only a fresh preference.
Windows was not in that original table (deferred to Phase 3 there); the
current `x86_64-pc-windows-gnu` naming was added independently later
(`agf/CHANGELOG.md` `[2.1.5]`). The governor's notes did not propose a
Windows public name — open question below.

## Everywhere the current names appear

- `.gitlab-ci.yml` — archive filename is built directly from the loop
  variable `target`/hardcoded `x86_64-pc-windows-gnu`, in both the
  `rust:tooling:preview`/`rust:tooling:release` shared script and the
  Windows conditional block. The `--target` flag passed to `cross build`
  must keep the real triple; only the `archive="agov-${VERSION}-..."`
  construction would change, via a target→public-name mapping.
- `tooling/rust/scripts/release-assets.sh` (this session's new script,
  MR !317) — `list()`'s glob patterns (`agov-*.tar.gz` etc.) are already
  name-agnostic, no change needed. `expected()`'s 14 hardcoded filenames
  and `selftest`'s fixture/negative-test names would need updating to the
  new scheme.
- `tooling/rust/scripts/agov-oci-image.sh` — `AGOV_TARGET` selects the
  Rust cross-compilation target for the OCI image's staged binary; it is
  unrelated to public archive filenames (OCI images aren't distributed as
  `agov-<version>-<target>.tar.gz`). No change needed.
- `tooling/rust/.cargo/config.toml` — real Rust `[target.*]` sections,
  internal build config, correctly unaffected by a public-name rename.
- `agf/docs/site/reference/release-artifacts.md` — the exact-filename
  reference table (lines 58-66) would need updating.
- `agf/docs/site/docs/adopt/install-and-layouts.md` — my sidecar-
  discoverability doc note (MR !317, just added to satisfy
  `agov validate adopter-docs`) doesn't hardcode filenames; unaffected.
- `README.md`, `agf/docs/site/docs/adopt/*.md` (other than the above) —
  no hardcoded target-triple strings found; no install command depends on
  the exact archive name pattern.
- Historical `docs/coordination/*.md`, `docs/plans/*.md`,
  `agf/CHANGELOG.md` entries — old names are historical evidence, must
  not be rewritten (per `rggs`'s finding, already binding this session).

## Compatibility

Per `agf/rules/workflow/deprecation-policy-rule.md`: this project is
Phase 0 (`customer_state.paying_customers: false`, no confirmed external
dependent on this specific naming). A silent-but-disclosed hard rename is
permitted under that rule without a deprecation cycle, provided it's
stated plainly in `agf/CHANGELOG.md` under a `Changed` (breaking) heading
with one sentence on why no deprecation cycle was used — not folded
silently into `Added`, not described with softening language. v2.3.0's
already-published assets keep their old names permanently (tags are
immutable); only new releases from v2.3.1+ would use the new scheme, so
there is no retroactive breakage of what's already public.

On old-name aliasing (governor's third note, offered as optional): I'd
recommend against it for this specific change — no confirmed external
dependent exists yet, and every asset changes filename on every release
anyway (the version segment), so a consumer with pinned automation is
already updating a URL per release; doubling upload/link work for an
alias with no known consumer seems like unjustified complexity per
`simplicity-rule.md`. Flagging as a recommendation, not a decision — this
is the governor's call, consistent with the note that asked for it.

## Scope recommendation

This does not fit v2.3.1's own frozen-scope character. The plan's two
already-approved "Required pre-release repairs" (root changelog, sidecar
discoverability) are *repairs of something broken* — the current asset
names function correctly; renaming them is an *improvement*, not a repair,
closer to a new product decision than a patch-release fix. The v2.3.1
plan itself says "No unrelated product feature... belongs on the release
branch," and expanding its frozen open-MR scope matrix requires an
explicit new governor decision, which I'm not making here.

**Recommend:** land as its own small, separately-governed change (a
dedicated plan/MR) either immediately after v2.3.1 ships or as v2.3.1's
own follow-up patch, rather than reopening v2.3.1's already Gate 1-approved
scope. If the governor judges the naming clarity important enough to hold
the recovery release for, that's a legitimate call to make explicitly —
just not one I'm defaulting into by implementing it inside !317.

## Open question for the governor

No public name was specified for the Windows target
(`x86_64-pc-windows-gnu`). If this proceeds, the analogous public name
would presumably be `windows-x86_64` following the same pattern, but that
wasn't in the original governed plan (Windows was deferred there) — worth
an explicit decision rather than my inferring it.
