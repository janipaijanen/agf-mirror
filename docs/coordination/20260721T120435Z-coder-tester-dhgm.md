---
kind: completion
from: coder-tester
to: all
id: 20260721T120435Z-coder-tester-dhgm
timestamp: 2026-07-21T12:04:35Z
immutable: true
references-to:
  - 20260721T080952Z-planner-architect-sdsk
  - 20260721T112008Z-coder-tester-mmq5
  - 20260721T120118Z-coder-tester-1jqq
topic: v2-3-0-release-complete
requires-ack: false
artifacts_owned: []
---

# v2.3.0 released and publicly verified

Completes the handoff chain starting at `sdsk`. Per the plan's state model
(`prepared -> reviewed -> merged -> tagged -> tag pipeline passed -> assets
published -> public artifacts verified -> completed`), all stages are now
done, verified from public surfaces rather than assumed from CI's own
report.

## Merge and tag

MR !306 merged to `main` at `c3219b199b1534bc4f3fce0dc42be53c7ce93498`
(parents `545f386` + release branch tip `291d16d`; diff confirmed to match
exactly the reviewed release-only content). Tag `v2.3.0` points at that
exact commit (`git ls-remote --tags origin` confirms
`d9f41a0f6ab78ab65db5bd69fdd904b00f184821` as the tag object,
dereferencing to `c3219b19...`).

## Tag pipeline

Pipeline `2693582488`: initially 9/11 jobs green, `agov:oci:release`
failed (`missing required environment variable: CI_REGISTRY_IMAGE` —
Container Registry was disabled at the project level), which also skipped
`release:create` via its `needs:` dependency. Root-caused via the job
trace and `GET /projects/:id` (see `1jqq`). Enabled Container Registry via
the GitLab API with the human governor's direction, retried the OCI job
(`15452727104`, success), and `release:create` then ran automatically and
succeeded. Pipeline `2693582488` is now fully green, all 11 jobs.

## Public-surface verification (this session, not CI's self-report)

- **GitLab Release**: `agents-govern v2.3.0`, `released_at:
  2026-07-21T12:02:10Z`, 5 asset links, all resolving.
- **Package registry**: downloaded all 5 archives + all `.sha256` +
  all 4 `-provenance.json` files directly from
  `.../packages/generic/agf/v2.3.0/...`. Every `sha256sum -c` reports
  `OK` (framework tar.gz/zip, Rust x86_64/aarch64 musl tar.gz, Windows
  zip). Every archive checked for forbidden/local paths — clean. Every
  provenance JSON carries `source_commit: c3219b199b1534bc4f3fce0dc42be53c7ce93498`,
  `tag: v2.3.0`, and the real `pipeline_id`/`job_id` for
  `2693582488`.
- **Executed the downloaded Linux binary**: `agov version --format json`
  -> `{"version":"2.3.0","commit":"c3219b199b1534bc4f3fce0dc42be53c7ce93498","dirty":false}`.
- **OCI image**: registry repository `agov` has exactly one tag, `2.3.0`
  (no floating `latest`). Pulled anonymously (public project/registry) via
  `podman pull registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.3.0`;
  ran it -> same version/commit/`dirty:false`; inspected labels
  (`org.opencontainers.image.version/revision/title/source` all correct),
  non-root `User: agov:agov`, `Entrypoint: [agov]`.
- **GitLab Pages**: `https://janipaijanen.gitlab.io/agents-govern-framework/`
  and its `reference/release-info/` page both report `v2.3.0` /
  `c3219b19` live.

## Residual/deferred items (not release-blocking, tracked separately)

- `learn/release-ci-host-preflight` (`k9p2`) — proposed learning on
  host-preflight discipline for release CI, still `status: proposed`,
  pending governor review.
- `2jtj` — non-blocking `agov:governance:validate` branch-pipeline
  force-push gap, documented, not fixed.
- `1jqq`/this record's Container Registry enablement — infra change made
  with explicit governor direction; worth a retrospective note on why the
  v2.3.0 CHANGELOG's OCI-release claim had never been exercised
  end-to-end against a real tag before now.
- Windows/`aarch64` execution verification remains an accepted limitation
  (no matching host architecture available); checksums, provenance, and
  archive contents were verified instead.

v2.3.0 is released, tagged, published, and independently verified from
public surfaces. This closes the release-preparation task from `sdsk`.
