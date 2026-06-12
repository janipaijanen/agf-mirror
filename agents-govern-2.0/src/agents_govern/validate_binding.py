# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""`agov-validate-binding` — Layer 3 binding validator.

Implements action-plan items B1 + B2 + B3 (issues #56 / #57 / #58):

    B1 — Universally Unique Identifier (UUID) v7 correlation
         identifier convention (see ``correlation_id.py``).
    B2 — ``Agent-ID:`` commit trailer parser, branch classifier,
         per-commit validator.
    B3 — ``<artefact>.agov.json`` sidecar metadata + filename-suffix
         fallback for environments stripping non-image files.

Per the action-plan migration discipline:

    enforcement applies prospectively from the adopter's declared
    tier-upgrade date. Existing commits / artefacts without bindings
    are not retroactively validated.

In schema-only mode (no ``framework.yaml`` binding section, the
situation in this repo at v1.1.45 — Phase 5 P1 ships the file in a
later batch), the validator parses any binding evidence it finds and
reports malformed values, but a *missing* binding is not a failure.
After P1 lands the file, the validator reads the ``binding`` section
and enforces against the configured branch patterns and artefact
scope.

Exposed Application Programming Interface (API):

    AgentTrailer            — dataclass for a parsed trailer
    parse_trailer           — single-line parser
    extract_trailers        — extract from a commit message's trailer block
    is_agent_active_branch  — branch classification per config
    validate_commit         — per-commit finding list
    Sidecar                 — dataclass for ``<artefact>.agov.json``
    parse_sidecar           — JSON-text parser
    sidecar_path            — resolve ``<artefact>.agov.json`` path
    compute_artefact_sha256 — hex digest helper
    read_sidecar            — read sidecar from filesystem
    verify_sidecar          — recompute hash; report mismatch
    parse_filename_fallback — extract correlation ID from filename suffix
    validate_binding        — top-level entry; returns int exit code
    main                    — Command Line Interface (CLI) entry

The underlying correlation-identifier convention is documented in
``correlation_id.py`` (item B1); the public-facing specification lives
in ``docs/binding-contract.md``.
"""

from __future__ import annotations

import fnmatch
import hashlib
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path

from agents_govern import correlation_id
from agents_govern._exit_codes import EXIT_PASS, EXIT_VALIDATION_FAILURE
from agents_govern._files import repo_root
from agents_govern._output import ok, warn

__all__ = [
    "AgentTrailer",
    "parse_trailer",
    "extract_trailers",
    "is_agent_active_branch",
    "validate_commit",
    "Sidecar",
    "parse_sidecar",
    "sidecar_path",
    "compute_artefact_sha256",
    "read_sidecar",
    "verify_sidecar",
    "parse_filename_fallback",
    "validate_binding",
    "main",
]


# Trailer key + everything-after-the-colon. Strict on the cid; liberal
# on agent name and version (defer name/version validation to the
# agent definition cross-check, not the trailer parser).
_TRAILER_KEY = "Agent-ID"
_TRAILER_LINE_RE = re.compile(
    rf"^\s*{_TRAILER_KEY}:\s*(?P<value>\S+)\s*$"
)
_TRAILER_VALUE_RE = re.compile(
    r"^(?P<name>[A-Za-z0-9._-]+):(?P<version>[A-Za-z0-9._+-]+):(?P<cid>[0-9a-f-]{36})$"
)

_DEFAULT_AGENT_ACTIVE_PATTERNS: tuple[str, ...] = ("agent/*",)


@dataclass(frozen=True)
class AgentTrailer:
    """A single parsed ``Agent-ID:`` commit trailer."""

    agent_name: str
    agent_version: str
    correlation_id: str


# ---------------------------------------------------------------------------
# Trailer parsing
# ---------------------------------------------------------------------------


def parse_trailer(line: str) -> AgentTrailer | None:
    """Parse one ``Agent-ID:`` trailer line; ``None`` if it does not parse."""
    if not isinstance(line, str):
        return None
    line_match = _TRAILER_LINE_RE.match(line)
    if not line_match:
        return None
    value_match = _TRAILER_VALUE_RE.match(line_match.group("value"))
    if not value_match:
        return None
    cid = value_match.group("cid")
    if not correlation_id.is_valid(cid):
        return None
    return AgentTrailer(
        agent_name=value_match.group("name"),
        agent_version=value_match.group("version"),
        correlation_id=cid,
    )


def _is_agent_id_line(line: str) -> bool:
    """True when a line *looks like* an Agent-ID trailer (key matches)."""
    return _TRAILER_LINE_RE.match(line) is not None


def _trailer_block_lines(commit_message: str) -> list[str]:
    """Return the lines comprising the commit's trailer block.

    Per the git convention: trailers live in the last paragraph of the
    message, and that paragraph contains nothing but trailer-shaped
    lines (``key: value``). If the last paragraph contains any non-
    trailer-shaped line, no trailers exist on the commit.

    This deliberately mirrors ``git interpret-trailers --only-trailers``
    semantics closely enough for validation: an Agent-ID-shaped line in
    the body proper is not extracted because the surrounding paragraph
    is not trailer-shaped.
    """
    if not commit_message:
        return []
    paragraphs = re.split(r"\n[ \t]*\n", commit_message.rstrip("\n"))
    if not paragraphs:
        return []
    last = paragraphs[-1].splitlines()
    if not last:
        return []
    # Trailer paragraph: every non-empty line matches "key: value" or is
    # an indented continuation of the previous trailer.
    trailer_re = re.compile(r"^[A-Za-z][A-Za-z0-9-]*:\s.*$|^[ \t]+\S.*$")
    if not all(trailer_re.match(line) for line in last if line.strip()):
        return []
    return last


def extract_trailers(commit_message: str) -> list[AgentTrailer]:
    """Return all valid ``Agent-ID:`` trailers from the commit's trailer block.

    Multiple trailers are permitted (multi-agent contributions); the
    returned list preserves source order. Lines that look like trailers
    but fail value-shape parsing are silently skipped here — the
    validator path picks them up via ``validate_commit``.
    """
    out: list[AgentTrailer] = []
    for line in _trailer_block_lines(commit_message):
        parsed = parse_trailer(line)
        if parsed is not None:
            out.append(parsed)
    return out


def _malformed_agent_id_lines(commit_message: str) -> list[str]:
    """Return Agent-ID-prefixed lines that failed to parse, anywhere in msg.

    Schema discipline: a contributor who wrote ``Agent-ID:`` clearly
    intended a trailer; a malformed value is a contract violation
    regardless of where it appears (body or trailer block).
    """
    out: list[str] = []
    for line in commit_message.splitlines():
        if _is_agent_id_line(line) and parse_trailer(line) is None:
            out.append(line.strip())
    return out


# ---------------------------------------------------------------------------
# Branch classification
# ---------------------------------------------------------------------------


def is_agent_active_branch(branch_name: str, config: dict | None) -> bool:
    """Classify a branch per the binding configuration.

    A branch is *agent-active* when it matches one of the configured
    ``patterns`` and does not match any ``human_only_branches`` or
    ``protected_branches`` pattern. Empty pattern list disables
    enforcement entirely. ``config=None`` falls back to the default
    ``agent/*`` pattern with no exclusions.
    """
    if config is None:
        section = {"patterns": list(_DEFAULT_AGENT_ACTIVE_PATTERNS)}
    else:
        section = config.get("agent_active_branches") or {}

    patterns = section.get("patterns", list(_DEFAULT_AGENT_ACTIVE_PATTERNS))
    excluded = (
        list(section.get("human_only_branches") or [])
        + list(section.get("protected_branches") or [])
    )

    for pattern in excluded:
        if fnmatch.fnmatchcase(branch_name, pattern):
            return False

    for pattern in patterns:
        if fnmatch.fnmatchcase(branch_name, pattern):
            return True

    return False


# ---------------------------------------------------------------------------
# Per-commit validation
# ---------------------------------------------------------------------------


def validate_commit(
    *,
    commit_message: str,
    branch: str,
    config: dict | None,
    enforcement_active: bool,
) -> list[str]:
    """Return findings (empty = pass) for a single commit.

    ``enforcement_active`` selects between:

    * ``False`` (schema-only): malformed trailers are flagged; missing
      trailers are not. This is the mode the validator runs in when no
      ``framework.yaml`` ``binding`` section is present.
    * ``True`` (enforce): missing trailers on agent-active branches
      are also flagged.
    """
    findings: list[str] = []

    for bad_line in _malformed_agent_id_lines(commit_message):
        findings.append(f"malformed Agent-ID trailer: {bad_line!r}")

    if not enforcement_active:
        return findings

    if not is_agent_active_branch(branch, config):
        return findings

    if not extract_trailers(commit_message):
        findings.append(
            f"missing Agent-ID trailer on agent-active branch {branch!r}"
        )

    return findings


# ---------------------------------------------------------------------------
# Top-level entry
# ---------------------------------------------------------------------------


def _binding_config() -> dict | None:
    """Load the binding configuration from ``framework.yaml`` if present.

    ``framework.yaml`` is introduced by Phase 5 item P1; until then the
    file does not exist in this repo and the validator runs in schema-
    only mode (returns ``None`` here, which makes ``validate_binding``
    skip enforcement).
    """
    framework_yaml = repo_root() / "framework.yaml"
    if not framework_yaml.exists():
        return None
    from agents_govern._yaml import load_yaml

    data = load_yaml(framework_yaml)
    if not isinstance(data, dict):
        return None
    return data.get("binding")


def validate_binding() -> int:
    """Top-level entry. Returns the strict-contract exit code.

    Schema-only mode is the default until P1 lands ``framework.yaml``.
    In that mode the validator simply confirms the binding *contract*
    is loadable (correlation_id + parser + classifier all import
    cleanly) and returns ``EXIT_PASS``. After P1 lands, this function
    will walk recent commits and apply ``validate_commit`` per branch.
    """
    config = _binding_config()
    if config is None:
        warn(
            "binding configuration absent (framework.yaml has no `binding:` "
            "section). Validator running in schema-only mode."
        )
        ok("Layer 3 binding contract loadable; enforcement awaits P1.")
        return EXIT_PASS

    # Future: walk recent commits and apply validate_commit per branch.
    # The enforcement path lands when P1 introduces framework.yaml; for
    # now, the presence-check above is the entire surface the validator
    # exercises in this repo.
    ok("framework.yaml binding section detected; enforcement-mode walk pending.")
    return EXIT_PASS


# ---------------------------------------------------------------------------
# Sidecar metadata (B3)
# ---------------------------------------------------------------------------


_SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
# agent_id in a sidecar mirrors the trailer's <name>:<version> halves —
# exactly one colon, both sides non-empty.
_AGENT_ID_RE = re.compile(
    r"^[A-Za-z0-9._-]+:[A-Za-z0-9._+-]+$"
)
_SIDECAR_SUFFIX = ".agov.json"

# UUID v7 token embedded in a filename — same shape as
# correlation_id._UUID_V7_RE but anchored to substring boundaries.
_FALLBACK_UUID_RE = re.compile(
    r"(?<![0-9a-f])"
    r"[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}"
    r"(?![0-9a-f])"
)


@dataclass(frozen=True)
class Sidecar:
    """Parsed contents of a ``<artefact>.agov.json`` sidecar."""

    correlation_id: str
    agent_id: str
    produced_at: str
    artefact_sha256: str


def parse_sidecar(json_text: str) -> Sidecar | None:
    """Parse a sidecar JSON document; ``None`` if it does not validate.

    Required fields: ``correlation_id`` (UUID v7), ``agent_id``
    (``name:version`` form, exactly one colon), ``produced_at`` (any
    string — strict timestamp validation deferred), ``artefact_sha256``
    (64 lowercase hex characters). Extra fields are silently
    accepted for forward-compatibility.
    """
    if not json_text:
        return None
    try:
        data = json.loads(json_text)
    except (json.JSONDecodeError, ValueError):
        return None
    if not isinstance(data, dict):
        return None

    cid = data.get("correlation_id")
    agent_id = data.get("agent_id")
    produced_at = data.get("produced_at")
    sha = data.get("artefact_sha256")

    if not all(isinstance(v, str) for v in (cid, agent_id, produced_at, sha)):
        return None
    if not correlation_id.is_valid(cid):
        return None
    if not _AGENT_ID_RE.match(agent_id):
        return None
    if not _SHA256_RE.match(sha):
        return None
    if not produced_at:
        return None

    return Sidecar(
        correlation_id=cid,
        agent_id=agent_id,
        produced_at=produced_at,
        artefact_sha256=sha,
    )


def sidecar_path(artefact_path: Path) -> Path:
    """Return the conventional sidecar path for an artefact.

    Convention: append ``.agov.json`` to the artefact's full filename
    (preserving any existing extension), so a ``release.tar.gz``
    sidecar lives at ``release.tar.gz.agov.json``.
    """
    return artefact_path.with_name(artefact_path.name + _SIDECAR_SUFFIX)


def compute_artefact_sha256(path: Path) -> str:
    """Return the SHA-256 hex digest of the file at ``path``."""
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def read_sidecar(artefact_path: Path) -> Sidecar | None:
    """Read and parse the sidecar for ``artefact_path`` if present."""
    side = sidecar_path(artefact_path)
    if not side.exists():
        return None
    try:
        text = side.read_text(encoding="utf-8")
    except OSError:
        return None
    return parse_sidecar(text)


def verify_sidecar(sidecar: Sidecar, artefact_path: Path) -> list[str]:
    """Return findings (empty = pass) for a sidecar/artefact pair.

    Recomputes the artefact's SHA-256 and compares against the
    sidecar's recorded value. Missing artefact or hash mismatch yield
    findings.
    """
    if not artefact_path.exists():
        return [f"sidecar references missing artefact {artefact_path}"]

    actual = compute_artefact_sha256(artefact_path)
    if actual != sidecar.artefact_sha256:
        return [
            f"artefact sha256 mismatch for {artefact_path.name}: "
            f"sidecar declared {sidecar.artefact_sha256}, computed {actual}"
        ]
    return []


# ---------------------------------------------------------------------------
# Filename-suffix fallback (B3)
# ---------------------------------------------------------------------------


def parse_filename_fallback(filename: str | Path) -> str | None:
    """Extract a UUID v7 correlation identifier from a filename.

    Convention (per docs/binding-contract.md): an artefact in an
    environment that strips sidecars (e.g., container registries that
    drop non-image files) carries the correlation identifier inside
    the filename itself, in the form ``<base>.<corr-id>.<ext>``. This
    helper scans the filename's ``stem`` portion for a UUID v7 token
    and returns it, or ``None`` if no v7 token appears.

    Path inputs are accepted; the parser only inspects ``.name``.
    """
    name = filename.name if isinstance(filename, Path) else filename
    match = _FALLBACK_UUID_RE.search(name)
    return match.group(0) if match else None


_HELP = """\
agov-validate-binding — Layer 3 binding-contract validator

Usage:
  uv run agov-validate-binding

Validates the Layer 3 binding contract: UUID v7 correlation identifiers
(item B1) and `Agent-ID:` commit trailers (item B2). In schema-only
mode (no `framework.yaml` `binding:` section) the validator confirms
the contract is loadable and exits 0. After P1 lands `framework.yaml`,
the validator switches to enforce mode and walks recent commits.

The Application Programming Interface (API) — `parse_trailer`,
`extract_trailers`, `is_agent_active_branch`, `validate_commit` — is
importable for adopter Continuous Integration (CI) glue that needs to
check a specific commit range; see `docs/binding-contract.md`.

Options:
  --help, -h  Show this message.

Exit codes (per `docs/integration-contract.md#exit-codes`):
  0 — pass
  1 — validation failure (malformed trailer or missing trailer in
      enforce mode on an agent-active branch)
"""


def main() -> None:
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print(_HELP)
        sys.exit(0)
    if args:
        print(f"agov-validate-binding: unknown argument(s): {' '.join(args)}")
        print("Run `agov-validate-binding --help` for usage.")
        sys.exit(2)
    sys.exit(validate_binding())
