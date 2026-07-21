---
id: v231-recovery-release
status: pending-gate1
topic: v2-3-1-recovery-release
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: the recovery changes release metadata and artifact discovery without introducing secret-handling behavior; existing infosec validation remains mandatory on every changed learning and the final release candidate
  - role: contribution-auditor
    status: consulted
    reason: critical scope review is routed to the Contribution Auditor through the coordination channel and MR discussion; its response is required before governor Gate 1 disposition
---

# Plan: v2.3.1 recovery release

## Status and decision requested

**Pending Gate 1. Do not implement the release-sidecar change, bump versions,
cut a release branch, merge a release MR, or create `v2.3.1` until the human
governor approves this plan through a durable coordination decision.**

The governor is asked to approve a focused patch-release recovery for the
post-publication v2.3.0 public-truth defect. Approval fixes the inclusion and
exclusion matrix below. It does not authorize moving or replacing v2.3.0, and
it does not pull all currently open MRs into the patch.

## Incident and binding disposition

The immutable tag `v2.3.0` exists, peels to merge commit
`c3219b199b1534bc4f3fce0dc42be53c7ce93498`, and its tag pipeline published
working framework/tooling archives and OCI image. The canonical adopter-facing
`agf/CHANGELOG.md` is correct, but repository-root `CHANGELOG.md` in the tag
still ends at 2.2.5. The v2.3.0 source tree therefore contains inconsistent
release history.

Binding recovery posture for this plan:

- v2.3.0 is a real published release, not a prerelease;
- v2.3.0 must not be called fully completed after the defect was discovered;
- never delete, move, or recreate tag `v2.3.0` or OCI tag `2.3.0`;
- repair public truth on `main` and publish a new immutable patch release,
  v2.3.1;
- preserve the premature completion records as historical evidence only when
  accompanied by a later immutable retraction/supersession record.

## Frozen open-MR scope matrix

MR state is evaluated before the release branch is cut. An MR listed here is
not approved merely by appearing in this plan; it still needs its ordinary
Gate 2 decision.

### Required recovery inputs before branch cut

| MR / work | Disposition for v2.3.1 |
|---|---|
| !314, root `CHANGELOG.md` 2.3.0 entry | Required after its governance trail stops describing v2.3.0 as completed and its entry is checked against the actual v2.3.0 repository diff. |
| !310, host-side release preflight learning | Required if approved; records the checksum-path incident and host/consumer target state. |
| !313, release public-truth surface learning | Required if approved; includes the exact-tag asset audit and sidecar-discoverability follow-up. |
| !307 and !309 | Required audit trail: independent Gate 2 findings/clearance and checksum-block acknowledgment. |
| !301, !302, and !304 | Resolve before freeze as historical v2.3.0 implementation/defect audit records. They add no v2.3.1 product feature. |
| v2.3.0 failure/recovery record | Required new immutable record that supersedes the false terminal-state claims without editing history. |
| checksum/provenance Release-page discovery | Required small implementation under this plan: expose every published individual checksum and provenance sidecar through direct GitLab Release links, or expose one durable indexed manifest whose entries link to every sidecar. The chosen path must be consumer-testable. |

### Must not merge as valid completion evidence

| MR | Disposition |
|---|---|
| !311 | Close or supersede; its v2.3.0 completion claim became false when the root changelog defect was discovered. |
| !312 | Close; do not merge the premature completion record `8ag3` as the terminal release truth. Preserve the branch/MR as historical evidence. |
| !273 | Close as superseded historical v2.3.0 NO-GO review; do not treat it as current v2.3.1 status. |

### Explicitly outside v2.3.1 product scope

- !117 academic-paper work.
- !293 site-wide terminology audit.
- !294 and !295 typed coordination relationships and critique.
- !297 MCP telemetry transport proposal.
- !299 and !300 learning-focused philosophical-lenses scheduling and ack;
  the actual implementation remains a later governed change.
- !305 unified provenance contract future TODO.
- Any implementation arising from those coordination/proposal MRs.
- The full automated release truth-surface validator, generalized provenance
  schema, SBOM generation, signing, installers, third-party notices, and
  universal provenance graphs.

Coordination-only future-work MRs may be merged independently as audit records
if the governor approves them, but doing so neither places their proposed
feature implementation in v2.3.1 nor makes them a release dependency. Once the
release base is frozen, later `main` merges are not silently pulled in.

## Required pre-release repairs

Perform behavioral repairs on their own governed feature/fix branches, merge
them to `main`, require green final pipelines, and only then freeze the exact
base commit.

1. Correct repository-root `CHANGELOG.md` with an accurate v2.3.0 historical
   entry, without claiming the release was fully completed.
2. Make checksum and provenance sidecars discoverable from the GitLab Release
   surface through direct links or a durable indexed manifest. Do not rely on
   users guessing generic-package URLs.
3. Add a consumer-layout test for the selected Release-page link/index
   mechanism. It must enumerate every expected archive, checksum, and
   provenance record and fail on a missing or non-resolving entry.
4. Publish the immutable failure/recovery record and merge approved learning
   and audit-trail MRs from the matrix above.
5. Record the exact frozen `main` commit and enumerate every delta between
   `v2.3.0` and that commit. Anything not explained by the matrix blocks branch
   creation until classified.

## Release branch and permitted changes

After all prerequisites and Gate 1 approval, create `release-v2.3.1` from the
recorded frozen `main` commit. The release MR may change only:

1. `agf/VERSION`: `2.3.0` to `2.3.1`.
2. Rust workspace package versions and internal dependency constraints to
   `2.3.1`, plus mechanically regenerated `Cargo.lock` workspace entries.
3. `agf/templates/AGENTS.md` and any governed loader marker required to match
   `agf/VERSION`.
4. `agf/CHANGELOG.md`: a 2.3.1 patch entry describing the public-truth repair,
   release-sidecar discoverability, and included approved learnings without
   overstating implementation of their recommendations.
5. Repository-root `CHANGELOG.md`: a 2.3.1 entry that records the recovery and
   distinguishes the already-published v2.3.0 history from v2.3.1.
6. Literal current-version examples, release metadata, and derived inventory
   counts proven stale by the truth-surface matrix.

No unrelated product feature, rule, schema, philosophical-lens addition,
terminology sweep, relationship model, telemetry contract, or academic-paper
change belongs on the release branch. A newly discovered behavioral defect is
fixed through its own governed MR and the release branch is then recreated or
cleanly rebased onto the newly frozen `main`.

## Mandatory release truth-surface matrix

Before opening the release MR, create a review artifact in the MR description
or a committed release-review document with one row for every current-version
or release-history surface. Each row must identify:

- path or public URL;
- authority/purpose (canonical, repository-level, derived, historical, or
  consumer-discovery surface);
- expected v2.3.1 state;
- whether it changes or is explicitly waived;
- host-side verification command;
- CI or public-surface verification command where applicable;
- observed result and exact commit/pipeline identity.

At minimum enumerate:

- root and adopter-facing changelogs;
- `agf/VERSION`, Rust workspace/lockfile, and adopter template marker;
- README and documentation current-version/OCI examples;
- Pages release information and release/download links;
- GitLab Release description and every primary archive link;
- every individual checksum and provenance sidecar link or indexed entry;
- framework, Linux x86_64, Linux aarch64, and Windows x86_64 archives;
- OCI version tag, digest, labels, embedded version, and revision;
- source tag and peeled merge commit.

Search the exact prospective source archive for `2.3.0`, `2.2.6`, and
`2.2.5`. Classify every occurrence as intentionally historical, contract-level,
test fixture/example, or defective current truth. An unexplained occurrence is
a release blocker. “Canonical” never exempts another file that independently
claims to record current release history.

## Host preflight before CI

Before pushing release content or consuming a release pipeline:

1. Build an archive of the exact prospective merge tree into an isolated
   staging directory.
2. Copy only consumer-downloadable artifacts into a fresh directory.
3. Verify every individual checksum with `sha256sum -c` from that directory.
4. List and extract archives; reject `.git`, `__pycache__`, `.pyc`, local
   `tmp`, absolute paths, traversal paths, and unexpected files.
5. Inspect provenance fields and execute every host-compatible binary.
6. Exercise the Release-page asset-link/index generator with dummy artifacts
   and verify every expected public name is represented.
7. Run the truth-surface matrix checks against the prospective source archive,
   including both changelogs.

Classify remaining checks as host-conditional or genuinely CI-only. CI-only is
reserved for protected credentials, registry publication, unavailable target
execution, and runner integration—not portable path or archive semantics.

## Exact-head validation and Gate 2

Run and record on the exact release content head:

```bash
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov
cargo test --manifest-path tooling/rust/Cargo.toml
cargo clippy --manifest-path tooling/rust/Cargo.toml --workspace --all-targets -- -D warnings
agov validate
agov validate adopter-docs
agov dist validate
agov dist build
mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
```

Also require:

- `CI_COMMIT_TAG=v2.3.1 agov dist check-tag` succeeds;
- a mismatched tag fails before archive creation;
- learning, infosec, plan, and coordination validation passes for every added
  governed record;
- a green MR pipeline on the exact content head;
- downloaded preview artifacts pass the same consumer-layout, forbidden-path,
  checksum, provenance, and executable checks;
- an OCI candidate build/smoke/scan has correct 2.3.1 labels and exact commit,
  with zero fixed HIGH/CRITICAL findings;
- the final MR description is staged in `tmp/MR_DESCRIPTION.md`, synchronized,
  fetched back, and compared with the exact final state;
- the Gate 2 handoff names content head, coordination-only tip if different,
  pipelines/jobs, truth-surface matrix, accepted target-execution limitations,
  and residual risks.

Any coordination-only tip must not conceal untested content. A green pipeline
for an earlier tree is not release evidence.

## Publication sequence

Track v2.3.1 through distinct states:

```text
prepared → reviewed → merged → tagged → tag pipeline passed
→ assets published → public truth matrix verified → completed
```

1. Governor reviews and merges the release MR.
2. Resolve the resulting `main` merge commit and verify it contains the exact
   approved release diff.
3. Create annotated tag `v2.3.1` on that merge commit and push it immediately
   in the same controlled action authorized by the merged release MR.
4. Verify the remote tag object and peeled target before proceeding. Never
   delete, recreate, or move it.
5. Monitor every tag/version, framework archive, Rust target archive, OCI
   build/smoke/scan/push, package upload, and GitLab Release creation job.
6. From public surfaces, verify every row of the truth-surface matrix:
   download all primary archives and sidecars through discoverable links,
   verify checksums/provenance, execute the Linux binary, pull/run the exact
   OCI tag and record its digest, verify Pages and both changelogs, and confirm
   the source tag target.
7. Only then publish an immutable completion record containing the completed
   matrix and exact tag, merge commit, pipeline, package, Pages, checksum,
   provenance, and OCI evidence.

## Failure and recovery

- Before tag push: stop, repair through an appropriately governed MR, rerun
  affected host/CI checks, and update the Gate 2 evidence.
- After tag push: preserve the tag and OCI version tag. Record the failure and
  prepare another patch; never rewrite history.
- Do not merge or publish a completion record while any matrix row is
  unverified, waived without governor authority, stale, or contradictory.

## Acceptance criteria

Gate 2 is eligible only when:

- all required recovery inputs are merged and excluded MRs remain excluded;
- !311/!312 completion claims are closed or durably superseded;
- both changelogs accurately distinguish v2.3.0 history and v2.3.1 recovery;
- every truth-surface matrix row has exact-head evidence;
- every expected archive, checksum, and provenance item is discoverable and
  consumer-verifiable;
- repository, documentation, Rust, distribution, negative-tag, OCI,
  vulnerability, infosec, and GitLab validations pass;
- the governor can identify the precise merge commit that will be tagged; and
- no evidence depends on a stale, skipped, or unrelated pipeline.

Release completion requires the post-publication public matrix, not merely a
green tag pipeline or a correct canonical changelog.

## Advisory review questions

1. Is the open-MR matrix narrow enough for a patch while preserving the
   necessary audit trail and learnings?
2. Does the root changelog correction describe v2.3.0 historically without
   retroactively pretending its published tag contained the correction?
3. Are direct sidecar links preferable to a durable indexed manifest for the
   GitLab Release consumer experience?
4. Does any supposedly historical old-version reference independently claim
   current truth and therefore need reclassification?
5. Is any automated truth-surface validator small and proven enough for this
   patch, or should the plan retain manual matrix enforcement and defer the
   generalized validator as specified?
