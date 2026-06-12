# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""`agov --capabilities` capability report.

Action-plan item A6 (issue #55). Exposes partial-adoption status as a
machine-readable JavaScript Object Notation (JSON) report so adopters
branching Continuous Integration (CI) on a declared compliance tier
(per `docs/adopt/compliance-profile.md`) can see *what is currently
shipped versus what is missing for the Full tier*.

The capability schema and `missing_for_full_compliance` semantics are
part of the strict-contract surface (per
`docs/contract-stability.md` — additive in minor releases, never
changed in patch releases).

Capability map at this version:

    artifact_validation       — Layer 1; validators ship.
    precedence_enforcement    — Shipped v1.1.36 via `_exit_codes`.
    confidence_tracking       — Shipped v1.1.33 via L2 (learning record
                                `confidence:` field validation).
    binding                   — Layer 3; closed in v1.1.45 with B3
                                (sidecar metadata + filename fallback)
                                joining B1 (UUID v7 correlation_id,
                                v1.1.44) and B2 (Agent-ID commit
                                trailer, v1.1.44).
    tension_recording         — Closed in v1.1.47 with P2: documented
                                discipline that cross-layer conflict
                                resolutions land as `category: tension`
                                learning entries.
    tension_recording         — Pending issue P2.

`missing_for_full_compliance` is the actionable surface — it lists
exactly the capabilities currently `False`, in the same order as the
capability map's declaration. Adopters targeting Full read this list
and know what to wait for.
"""

from __future__ import annotations

import json
from collections import OrderedDict

from agents_govern._files import repo_root

__all__ = [
    "compute_capabilities",
    "to_json",
]


def _read_version() -> str:
    """Return the framework version from the repo's VERSION file."""
    version_file = repo_root() / "VERSION"
    return version_file.read_text(encoding="utf-8").strip()


def _capability_map() -> "OrderedDict[str, bool]":
    """The capability flags this framework version exposes.

    Order matters: `missing_for_full_compliance` preserves it so the
    list is deterministic across runs.
    """
    return OrderedDict(
        [
            # Layer 1 — artefact validation. Validators ship; always true.
            ("artifact_validation", True),
            # v1.1.36 shipped the exit-code precedence helper.
            ("precedence_enforcement", True),
            # v1.1.33 shipped L2 (confidence: ordinal + evidence_count).
            ("confidence_tracking", True),
            # Layer 3 — binding via UUID v7 + commit trailer + sidecar.
            # All three legs present as of v1.1.45: B1 (correlation_id),
            # B2 (validate_binding trailer parser + branch classifier),
            # B3 (sidecar parser + hash verifier + filename fallback).
            ("binding", True),
            # Mandatory tension recording on resolved cross-layer
            # conflicts. Documented discipline shipped via P2 (issue
            # #70) in v1.1.47: cross-layer conflict resolutions are
            # recorded as `category: tension` learning entries per
            # learnings/_template.yaml header and the canonical
            # docs/concept/principles.md#tension-recording section.
            ("tension_recording", True),
        ]
    )


def compute_capabilities() -> dict:
    """Return the capability report as a plain dict."""
    version = _read_version()
    caps = _capability_map()
    missing = [name for name, present in caps.items() if not present]

    return {
        "framework_version": version,
        # Per A5 stability promise: contract version = framework version.
        "contract_version": version,
        "capabilities": dict(caps),
        "missing_for_full_compliance": missing,
    }


def to_json() -> str:
    """Serialise the capability report to the strict-contract JSON shape."""
    return json.dumps(compute_capabilities(), indent=2)
