---
kind: decision-record
from: governor-reviewer
to: all
id: 20260917T055825Z-governor-reviewer-h2jk
timestamp: 2026-09-17T05:58:25Z
model: human-governor-via-codex
immutable: true
references-to:
  - 20260917T050745Z-capability-developer-2jk2
  - 20260916T191501Z-contribution-auditor-dbrg
topic: learning-contract-versioning-location-migration-and-otgr-envelope
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Decision: versioned learning contracts, legacy preservation, and relaxed OTGR payload

The governor approves the learning-record contract direction proposed in the
2026-09-17 development session, including use of the AGF GitLab Pages site as
the canonical URI namespace. This record preserves that decision; it is not
Gate 1 approval for implementation.

## Learning contract identity and distribution

Learning-record schemas, lifecycle definitions, and semantic profiles shall
have explicit, independently meaningful contract identifiers and versions.
They shall be stored as immutable approved artifacts in the AGF repository,
shipped in AGF release archives, and registered with status, digest, authority
reference, compatibility, and successor metadata.

The canonical URI namespace shall be rooted at:

```text
https://janipaijanen.gitlab.io/agents-govern-framework/contracts/
```

Canonical URIs identify contracts; they shall not create a network dependency.
Normal validation must resolve approved contracts from bundled local copies and
verify their registered digest. Mutable `main` or raw-branch URLs are not
canonical contract identifiers.

AGF product versions and learning-contract versions are independent. An AGF
release manifest shall state which learning-record schema, lifecycle, and
semantic-profile versions it bundles, supports, and uses by default for new
records. A contract version may be shipped unchanged by several AGF releases.

Approval is separate from numbering. A version identifier alone does not mean
approved. The registry shall distinguish at least draft, approved, deprecated,
and retired states and name the governing authority record.

## Record and lifecycle fields

New-contract learning records shall identify their record-schema version and
lifecycle version. Each append-only lifecycle stamp shall identify the
lifecycle version under which its transition was made. A derived semantic
representation shall identify its semantic-profile/context version.

The admitted learning body remains immutable. Lifecycle stamps contain only
closed-vocabulary transition metadata and evidence/authority references;
substantive later remarks remain annexes or separately linked learning records.

## Existing adopter records

Framework upgrades shall not silently rewrite adopter-owned learning records.
Records without a per-record schema version shall be recognized explicitly as
legacy rather than inferred to satisfy the new contract. A corpus may contain
both legacy and current records after an adopter records its cutover and the
default contract for newly admitted records.

Historical records shall not be assigned invented approvals, lifecycle states,
or timestamps. They shall not be moved to an archive merely because they use an
older schema. Archive/storage placement is a view, not semantic lifecycle
state. Any later migration facility must begin with a dry-run proposal/report,
retain source provenance, distinguish classification from transformation, and
avoid destructive rewriting by default.

## Generic versioning principle

Every proposed change to a persisted or independently consumed contract must
include an explicit compatibility and versioning disposition: no contract
change, compatible extension, incompatible new version, or replacement of an
unapproved draft. This does not require a new version number for every Git
change. Git provides exact change identity for implementation and editorial
changes that do not alter a consumed contract.

## OTGR envelope boundary

OTGR shall retain a small, strict, versioned infrastructure envelope for
identity, ordering, authenticated attribution, delivery, limits, and the
non-authoritative trust marker. Conversation bodies remain free-form;
experimental metadata remains optional and extensible. Semantic extensions are
separately versioned and must not make delivery depend on RDF storage, SHACL,
reasoning, or remote context dereferencing.

The replacement OTGR plan shall explicitly dispose the earlier draft's
`protocol_version: "0.1"` design and decide whether protocol and envelope
schema remain lockstep or receive separate identifiers. It shall preserve the
charter's relaxed communication model rather than allowing a strict semantic
document to prescribe participant behaviour.

## Next governed work

The next learning-contract plan shall cover the versioned artifact layout,
registry and release-manifest binding, local URI resolution, legacy dispatch,
mixed-corpus validation, adopter cutover, lifecycle replay, semantic export,
and negative/compatibility fixtures. Implementation requires its own approved
Gate 1 plan.

The OTGR replacement plan remains separate. It may reuse the generic contract
versioning discipline, but learning-contract implementation shall not depend on
the chat service and AGF operation shall remain complete when OTGR is absent.

## Provenance

This decision was stated and approved directly by the human governor in the
active development session. Codex transcribed the approved substance into the
coordination schema without adding implementation authority.
