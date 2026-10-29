<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.13 release-content-freeze documentation-estate sweep

**Sweep class:** release-content-freeze (blocking)

**Steward:** Codex, temporarily allocated as documentation-estate-steward by
governor decision `20261029T205351Z-governor-reviewer-mbsk`

**Combined implementation baseline:**
`32e384e74959d456acf85435fcd7d7c7194d8cad`

**Reviewed documentation candidate before this report is committed:**
`2dd77780f0624c3e47e6cf1378d5cfbb63b8a422` (includes factual correction
`e1c0e102` and current main `6042bc8b`)

**Previous full sweep baseline:**
`e03b3261ccb8a4205c45801ea1d034971f437a2d`

**Status:** **COMPLETE — documentation-estate assessment clean.** All four
content findings are fixed and revalidated. This closes the estate-sweep
precondition; it does not clear the separate APK-risk disposition, final
exact-head CI/Gate 2, or release-publication gates. The final docs SHA will be
the later commit that adds this report, not the implementation baseline or the
pre-report candidate above.

## Estate scope and completion criterion

The unit of review is the complete adopter-facing documentation estate, not
only pages changed after v2.12. The sweep completes only when every class below
has been inspected or explicitly excluded, all six steward dimensions have a
rating, every finding has a disposition, the post-v2.12 first-parent inventory
has a changelog/no-shipped-byte disposition, and the final corrected candidate
passes the applicable current-tree validations. An open, unknown, referred, or
unvalidated finding does not clear the freeze.

### Complete classified touchpoint inventory

The following recursive classes are exhaustive for the stated baseline. A glob
classification covers every matching file; counts are included so additions
cannot disappear behind a category label.

| Class | Complete surface set | Count | Classification |
|---|---|---:|---|
| Root adopter and loader surfaces | `README.md`, `AGENTS.md`, `.claude/CLAUDE.md` | 3 | Included. Root `ADOPTING.md` and `GOVERNANCE.md` are absent by design; their canonical product equivalents are under `agf/`. |
| Root project projections | `docs/roadmap.md`, all eight Markdown files directly under `docs/reference/` | 9 | Included where they project current product, command, release, or operator claims. |
| Product entry surfaces | `agf/ADOPTING.md`, `agf/framework.md`, `agf/VERSION`, `agf/CHANGELOG.md` | 4 | Included. The changelog is assessed, not owned by the steward. |
| Pages prose | Every `*.md` recursively under `agf/docs/site/` | 49 | Included. This is the complete public-site prose set, including both roadmaps/reference pages and operating/adoption pages. |
| Non-site product prose | Every `*.md` recursively under `agf/docs/` excluding `agf/docs/site/` | 25 | Included for currency, claims and pointers: adoption, ADR, contract, governance, reference, build and figure-index prose. |
| Normative/customize owners | Every `*.md` recursively under `agf/rules/` | 53 | Included for owner identity, normative claim, CUSTOMIZE instruction and pointer consistency; no normative edits authorized. |
| Adopter templates | Every file recursively under `agf/templates/` | 10 | Included: three loaders/readmes, five coordination/plan templates, the learning template and the private-codename example. |
| Adopter skills | Every file recursively under `agf/skills/` | 10 | Included as shipped operator guidance even though the steward role names them indirectly through projection/distribution surfaces. |
| Agent and role metadata | Every `*.yaml` directly under `agf/agents/` | 18 | Included for live role/status/framework-version and documentation-owner claims; role definitions are read-only. |
| Learning current-version cells | Every `*.yaml` directly under `agf/learnings/` | 2 | Included. `foundry.yaml` is the live adopter record at 2.12.0; `harbor-workshop.yaml` is explicitly a submitted historical 2.5.0 snapshot, not a current-version cell. |
| Projection manifests | Every `*.yaml` directly under `docs/reviews/rule-projection/` | 41 | Included for six-surface ownership, pointer existence and post-v2.12 normative coverage. The 41st is the approved urgency-currency correction manifest. |
| Foundry assistant projections | All 56 entries under `.claude/rules/` (49 symlinks, 7 regular local projections) | 56 | Included for loader/path projection. Symlinks are classified by byte identity with their canonical `agf/rules/` targets and are not counted as independent normative owners. |
| Site non-Markdown assets | All 19 non-Markdown files under `agf/docs/site/`: `llms.txt`, two HTML overrides, `robots.txt`, `stylesheets/agf.css`, 13 SVGs and one PNG | 19 | Text discovery/pointer files are checked for currency and existence. Visual rendering, CSS, theme, responsive layout and accessibility are excluded to web-ux-reviewer; image bytes are checked only as referenced-path targets. |
| Documentation build and figure implementation | `agf/docs/build/mkdocs.yml`; 11 Python hook/test files; 13 mirrored SVGs, one PNG, 13 DOT sources, renderer/tests, flyer HTML/build source and five embedded font files under `agf/docs/figures/` | 48 | Excluded from prose/visual review. Read only as validation and pointer evidence. Build behavior belongs to its implementation owner; visual quality belongs to web-ux-reviewer. |
| Project governance history | `docs/plans/**`, `docs/coordination/**`, other `docs/reviews/**` | n/a | Excluded as adopter-facing estate. The approved release scope/plan, prior freeze report and relevant projection records are read as evidence, not treated as public product prose. |
| Product implementation | Schemas, Rust tooling, CI, fixtures and tests | n/a | Excluded from documentation authorship. Read only to test whether capability/path claims match the implemented and released surface. |
| Bootstrap reference | `agents-govern/**` | n/a | Explicitly excluded: historical bootstrap reference, not the v2 product. |

The inventory was freshly enumerated at the combined baseline. Prior sweep
evidence was reused only for unchanged bytes after `git diff --name-only
e03b3261..32e384e7` and never as a substitute for the fresh inventory or
current-tree checks. Changed estate surfaces were inspected directly. Those
changes were root `AGENTS.md`; `agf/ADOPTING.md`; all live agent YAML due to the
2.12 version/status update; governance/manifest pages; MCP and coordination
pages; both roadmaps; `foundry.yaml`; the coordination-publication owner;
two templates; and the three new projection manifests. A byte-identical page
contained finding F1 below, demonstrating why unchanged-byte reuse alone was
insufficient. The approved correction was then inspected at `e1c0e102`, the
inventory was refreshed to 41 manifests, and the corrected candidate was
revalidated after merging current main.

## Six-dimension assessment

| Dimension | Rating | Key finding |
|---|---|---|
| 1. Framework-version currency | Clean | `agf/VERSION`, current/latest-release cells, live agent definitions, loaders, template, governance manifest, roadmaps and live foundry learning record agree on 2.12.0. The 2.5.0 harbor file is an explicitly historical submitted snapshot. |
| 2. Capability and maturity currency | Clean | F1 is corrected to the released v2.11 Level 5 fact. F3 and F4 now state capability maturity separately from evidence status and do not overclaim production-write or broad-adopter validation. |
| 3. Cross-touchpoint claim consistency | Clean | The polling guide, canonical coordination protocol and Unreleased correction now agree on v2.11 shipment, Level 5 with compatible artifacts, and urgency's advisory/no-permission boundary. F2 cites the actual !3332 administrative merge. |
| 4. Pointer and path integrity | Clean | All paths declared by the 41 projection manifests exist; canonical loader/rule links and changed-page pointers resolve. Strict site build, rendered structural checks, relative/external links and anonymous v2.12 archive verification pass. |
| 5. Projection-manifest completeness | Clean | Bounded record-only publication termination and the factual urgency correction each have coherent six-surface manifests. Strict change-bounded validation of the new correction manifest passes. No post-v2.12 normative adopter change lacks projection evidence. |
| 6. Estate coverage | Clean | All adopter prose, loaders, templates, skills, normative/customize rules, live metadata/learnings, projections and non-prose exclusions are classified above. No new unclassified adopter-facing surface was found. |

Overall rating is **Clean after correction**. The initial material drift remains
recorded as F1 rather than being erased from the audit trail.

## Findings and dispositions

| ID | Surface / dimension | Finding | Recommended disposition | Actual disposition | Authority |
|---|---|---|---|---|---|
| F1 | `agf/docs/site/docs/operate/poll-and-coordinate.md:250-255`; dimensions 2 and 3 | The heading said “v2.11 development CLI” and the body called urgency ordering Level 4 and pending release, conflicting with the canonical protocol. | Correct only the heading and stale maturity/shipment sentence, preserve advisory/no-authority semantics, and add Unreleased/projection evidence. | **Fixed and verified.** Governor amendment `20261029T211008Z-governor-reviewer-gpp8` approved the two-path expansion after independent review. Commit `e1c0e102` makes exactly the approved page and adopter-changelog correction; the manifest is strict-valid and the site/search checks find no stale phrase. | gpp8 amended Gate 1; release owner implemented; steward verified. |
| F2 | Root `CHANGELOG.md`; dimension 3 | The initial draft attributed v2.12 administrative status synchronization to MR !3339. The merge tree shows `34789b4c` is MR !3332; !3339 is an unrelated coordination carrier. | Cite !3332 and retain the administrative/no-new-capability qualifier. | **Fixed by release owner** in the current draft before freeze submission. | Existing changelog ownership in the approved freeze plan. |
| F3 | `agf/CHANGELOG.md`; dimension 2 | The initial record-only entry stated evidence status without a distinct capability maturity, allowing maturity and evidentiary strength to blur. | State Level 4 in current main, Level 5 only after the supported v2.13 release, and keep Empirically informed as a separate evidence status. | **Fixed by release owner** in the current draft before freeze submission. | Existing changelog ownership in the approved freeze plan. |
| F4 | Root `CHANGELOG.md`; dimension 2 | The initial !3358 CI/publisher entry described a new capability but stated neither capability maturity nor evidence status. | Label it Level 4 Validated in current main and Empirically informed; do not claim Level 5 or production-write validation. | **Fixed and verified.** The committed entry now names Level 4, Empirically informed tests/read-only-probe evidence, and the absence of production-write validation. | Existing changelog ownership in the approved freeze plan. |

No finding is silently accepted. F1-F4 are fixed with the authority and evidence
recorded above. There is no open documentation-content finding.

## Post-v2.12 first-parent merge crosswalk

Range: peeled v2.12 commit
`e045d20b073ce31341cd0d460469fac60cdbab5a` (exclusive) through combined
baseline `32e384e74959d456acf85435fcd7d7c7194d8cad` (inclusive), plus the two
subsequent first-parent administrative merges on current main through
`6042bc8b201f74f4c9b5a0f3b0dab06457dd1cef`. All 48 are classified below.
Plans and coordination carriers are not advertised as features.

| Merge | Disposition |
|---|---|
| `2eb83f66` — v2.12 release completion coordination | No shipped product bytes; coordination evidence only. |
| `6e927570` — v2.12 status-sync scope coordination | No shipped product bytes; coordination evidence only. |
| `34789b4c` — v2.12 publication status sync | Root changelog administrative release-truth entry; not a v2.13 capability. |
| `c677c31f` — status-sync Gate 2 coordination | No shipped product bytes; coordination evidence only. |
| `9cf7cd3c` — status-sync completion coordination | No shipped product bytes; coordination evidence only. |
| `19f362e0` — stale handoff acknowledgment | No shipped product bytes; coordination evidence only. |
| `c6cd5a7b` — third-party coexistence acknowledgment | No shipped product bytes; coordination evidence only. |
| `15add7eb` — MR !3066 collision review | No shipped product bytes; coordination evidence only. |
| `ccb52317` — post-v2.12 audit intake | No shipped product bytes; coordination evidence only. |
| `b61ff556` — refreshed intake acknowledgment | No shipped product bytes; coordination evidence only. |
| `cc2976a3` — post-v2.12 plan-collision coordination | No shipped product bytes; coordination evidence only. |
| `6ba049ba` — refreshed handoff acknowledgment | No shipped product bytes; coordination evidence only. |
| `ae18caf9` — MR !3340 advisory review | No shipped product bytes; coordination evidence only. |
| `2b90b87e` — MR !3272 Gate 1 coordination | No shipped product bytes; coordination evidence only. |
| `fc76e31c` — record-recursion plan | Plan only; no shipped product bytes. |
| `27cbfaf5` — APK residual-risk acknowledgment | No shipped product bytes; coordination evidence only. |
| `27f6b8c6` — MR !3340 rereview | No shipped product bytes; coordination evidence only. |
| `4e594eae` — MR !3340 final advisory clear | No shipped product bytes; coordination evidence only. |
| `10afc5b8` — MR !3340 exact-head acknowledgment | No shipped product bytes; coordination evidence only. |
| `050d5ac8` — supersede philosophical-lenses plan | Administrative plan/TODO state only; no shipped feature bytes. |
| `e901e4b0` — v2.13 planning Gate 1 coordination | No shipped product bytes; coordination evidence only. |
| `8d0ab371` — record-only slice claim | No shipped product bytes; coordination evidence only. |
| `a040e99f` — distribution profiling parent plan | Plan only; no shipped product bytes. |
| `f0d54a4d` — CI reproducibility pins parent plan | Plan only; no shipped product bytes. |
| `6b9389ae` — v2.13 scope Gate 1 coordination | No shipped product bytes; coordination evidence only. |
| `78184c8c` — v2.13 release scope plan | Plan only; no shipped product bytes. |
| `b985c984` — record-only Gate 2 coordination | No shipped product bytes; coordination evidence only. |
| `ee2a768a` — bounded record-only publication termination | Adopter-facing `agf/CHANGELOG.md` entry; rule, protocol and template bytes ship in the next distribution. |
| `81941b9d` — CI-pins handoff | No shipped product bytes; coordination evidence only. |
| `26d26c09` — CI-pins acceptance | No shipped product bytes; coordination evidence only. |
| `40e929f2` — CI-pins gate-scope block | No shipped product bytes; coordination evidence only. |
| `15d2ba43` — CI-pins test amendment Gate 1 | No shipped product bytes; coordination evidence only. |
| `e477626f` — CI-pins test amendment plan | Plan only; no shipped product bytes. |
| `22381ade` — CI-pins test coverage completion | No shipped product bytes; coordination evidence only. |
| `2b76eb6f` — CI-pins evidence handoff | No shipped product bytes; coordination evidence only. |
| `4a9d0130` — profiling gate-test amendment plan | Plan only; no shipped product bytes. |
| `bd639b50` — profiling test Gate 1 coordination | No shipped product bytes; coordination evidence only. |
| `49d82cfe` — profiling ROI amendment plan | Plan only; no shipped product bytes. |
| `5ec22b7c` — profiling ROI Gate 1 coordination | No shipped product bytes; coordination evidence only. |
| `112f8a5c` — publisher API amendment plan | Plan only; no shipped product bytes. |
| `67772d36` — publisher Gate 1 coordination | No shipped product bytes; coordination evidence only. |
| `5bcffccb` — profiling baseline-acceptance amendment | Plan/scope correction only; no shipped product bytes. |
| `e68d162e` — opt-in distribution profiling | Root changelog entry. Foundry internal diagnostic/report; absent from default release binaries and not adopter documentation. |
| `2a99009d` — profiling Gate 2 coordination | No shipped product bytes; coordination evidence only. |
| `c7ea3cfb` — CI pins and create-only release publisher | Root changelog entry. Foundry CI/release operation only; no adopter CI claim. |
| `32e384e` — v2.13 freeze-preparation plan | Plan only; no shipped product bytes. |
| `bf73d6ee` — CI/publisher Gate 2 coordination (!3375) | Audit/authority evidence only; no shipped implementation bytes and no changelog feature. |
| `6042bc8b` — urgency-currency plan/projection amendment (!3376) | Plan and projection audit evidence only; no shipped implementation bytes. The separately governed page correction remains `e1c0e102` on the freeze branch. |

## Validation and evidence

Completed against baseline `32e384e7`, correction `e1c0e102`, and the reviewed
pre-report candidate `2dd77780` as applicable:

- Full Rust workspace, release/all-features: 852 passed, 0 failed, 4 ignored
  across 39 summaries (`/tmp/agf-v213-workspace-all-features.log`). The ignores
  are the two real platform/OCI composed checks, the real disposable-registry
  network check, and the manual profiling-baseline recorder; their names and
  explicit run paths remain in the log. Source code is unchanged from baseline
  `32e384e7`.
- Distribution validation/build passed. The final no-version-bump archive build
  retained 2.12.0 metadata and 262 manifest files, producing tar/zip plus
  provenance (`/tmp/agf-v213-dist-validate.log`,
  `/tmp/agf-v213-dist-final-build.log`). Both archive suites passed 28 shipped
  tests each; self-sufficiency passed and re-runs shipped coverage, so it is not
  counted as a third independent suite (`/tmp/agf-v213-shipped-tests.log`,
  `/tmp/agf-v213-self-sufficiency.log`).
- Documentation source hooks: 27 passed; cross-surface tooling tests: 3 passed
  (`/tmp/agf-v213-doc-hooks.log`, `/tmp/agf-v213-doc-tooling.log`).
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed in 4.53 seconds.
  Rendered first-screen and commissioning-link checks passed; both external
  endpoints returned HTTP 200 and the linked v2.12 AGF/agov archives, checksums
  and provenance verified anonymously (`/tmp/agf-v213-site.log`,
  `/tmp/agf-v213-firstscreen.log`, `/tmp/agf-v213-links.log`). The documented
  non-nav reference pages remain informational MkDocs output, not errors.
- `agov validate adopter-docs`, `agov validate infosec` and the supported
  `agov validate agents` invocation passed. An attempted unsupported
  `agents --strict` flag was rejected and was not represented as evidence.
  `python3 agf/docs/build/hooks/check_stale_version_labels.py` passed.
- Fresh enumeration covers all 41 projection manifests: each has all six rows
  and every declared current-tree path exists. Strict change-bounded
  `projection-check` for the new freeze/currency manifest passed
  (`/tmp/agf-v213-final-projection.log`).
- The supported whole-tree command `agov plan projection-check-all --base-ref
  origin/main` passed all 41 manifests (`/tmp/agf-v213-projection-all.log`). An
  earlier attempt to pass one release-wide base to all historical per-change
  manifests with `projection-check` exited 1 because that invocation had the
  wrong comparison semantics. It was a method error, was excluded from evidence,
  and is not represented as a corpus failure; the corrected supported command
  is the whole-tree mechanical result.
- The final changed-plan collision check reports 0 unacknowledged and 6
  acknowledged overlaps; historical no-`affected_paths` coverage remains
  explicit (`/tmp/agf-v213-final-collisions.log`). `git diff --check` passes.
- Environment recorded for the local evidence: Linux 6.8.0-147-generic x86_64,
  rustc/cargo 1.99.1, Python 3.14.5, MkDocs 1.6.1 and Material 9.7.7
  (`/tmp/agf-v213-docs-env.log`).

The documentation-estate assessment is closed. The following are external
release-gate conditions, not open estate findings:

1. Commit this report, run the implementation MR's exact-head CI, obtain
   independent final report/changelog review and binding governor Gate 2.
2. Obtain the governor's explicit disposition of the separately recorded
   mutable-APK residual risk; this report neither accepts nor waives it.
3. Merge the freeze-preparation change and recheck any intervening main-only
   audit carriers in the release owner's final crosswalk.
4. Use a separate approved release-publication plan for version, provenance,
   tag, package, OCI, GitLab Release and deployment actions.

## Escalations to governor

F1 was the sole source-path escalation discovered by this sweep. Governor
decision `20261029T211008Z-governor-reviewer-gpp8` answered the bounded question:
it authorized the polling-page correction and projection evidence while
preserving urgency semantics, tool authority, timer behavior and runtime
behavior. The correction and validation are complete; no estate escalation is
open.

No visual/UX, fact-verification, normative redesign or structural-manifest
question was silently absorbed by the steward.

## Comparison with the previous sweep

- **New and resolved:** F1, an unchanged-byte stale Level 4/pending-release
  statement missed by the v2.12 sweep, was corrected under amended Gate 1. F2,
  F3 and F4 were draft accuracy/maturity omissions and were fixed by their
  author before final assessment.
- **Resolved since the previous sweep:** the v2.12 release-status and
  coordination shipment corrections recorded by the previous report remain
  consistent in current main. The record-only addition has coherent owner,
  protocol, loader and projection surfaces.
- **Still open:** no previous-sweep or current-sweep content finding remains
  open. APK-risk disposition, exact-head CI/Gate 2 and release publication are
  separate release prerequisites, not documentation-estate findings.

Final review also removed five Markdown trailing-space hard breaks from this
report's metadata header and re-ran whitespace validation; this formatting fix
changes no finding, disposition or evidence claim.

This report grants no release, tag, deployment or Gate 2 authority.
