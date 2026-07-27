---
kind: decision-record
from: governor-reviewer
to: all
id: 20260727T010904Z-governor-reviewer-6qpr
timestamp: 2026-07-27T01:09:04Z
model: human-governor
immutable: true
references-to: []
topic: whole-installation-compatibility-provenance-verification
requires-ack: false
artifacts_owned: []
---

# Adopt whole-installation compatibility and provenance verification

Commissioning or updating AGF, `agov`, or another required installed component
MUST verify the compatibility and provenance of the whole installed component
set before the operation reports completion.

The invariant is not simple version equality. Independently versioned
components may pass when published compatibility metadata proves that the
combination is supported. Verification must inspect the actual installation,
not only the assumptions bundled with the component being installed.

At minimum, the governed contract must account for the installed AGF version
and integrity, the `agov` version and build provenance, the supported AGF range,
the schema source used for validation, supported loaders and installed skills,
documentation provenance, and intentionally absent optional components.

A known incompatibility, mixed major lineage, failed integrity check, or
unresolved provenance mismatch is blocking. A required companion that cannot be
inspected is incomplete verification and cannot be reported as success without
an explicit governor waiver. Compatible non-identical versions pass with the
compatibility basis recorded. Optional absent components pass only when their
absence is explicit.

The supported design should permit offline verification from trustworthy local
release metadata. Network availability must not become an accidental condition
for establishing installation coherence.

This decision authorizes Gate 1 planning. It does not authorize implementation,
release preparation, versioning, tagging, or mutation of an adopter
installation.
