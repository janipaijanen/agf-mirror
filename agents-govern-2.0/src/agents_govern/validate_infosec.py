# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""InfoSec Sentinel: screen learning records for information leaks."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from agents_govern._files import find_learnings, repo_root
from agents_govern._output import ack as print_ack
from agents_govern._output import block as print_block
from agents_govern._output import info, warn
from agents_govern._yaml import extract_freetext, load_yaml

__all__ = ["validate_infosec", "main"]

# ---- Compiled regex patterns (immutable) ----

_VCS_TOKEN_RE = re.compile(
    r"(glpat-|ghp_|gho_|github_pat_|glcbt-|glrt-|xoxb-|xoxp-|xoxr-)[^\s\"]{4,}"
)
_DB_CONN_RE = re.compile(
    r"(postgres|postgresql|mysql|redis|mongodb|amqp|rabbitmq)://[^\s\"]*[:@]"
)
_API_KEY_RE = re.compile(
    r"(api[_-]?key|secret[_-]?key|SECRET_KEY|password|passwd|pwd)\s*[:=]\s*[^\s\"]{8,}"
)
_PRIVATE_KEY_RE = re.compile(r"BEGIN\s+(RSA |EC |DSA |OPENSSH )?PRIVATE KEY")
_USER_PASS_RE = re.compile(
    r"(user|username|login)\s*[:=].*\b(pass|password|pwd)\s*[:=]"
)
_DDL_RE = re.compile(
    r"\b(CREATE|ALTER|DROP)\s+(TABLE|INDEX|VIEW|DATABASE)\b", re.IGNORECASE
)
_FILE_PATH_RE = re.compile(
    r"/(home|var|opt|srv|etc)/[a-zA-Z0-9_/.-]+|[A-Z]:\\[a-zA-Z0-9_\\.-]+"
)
_RFC1918_RE = re.compile(
    r"\b(10\.[0-9.]+|172\.(1[6-9]|2[0-9]|3[01])\.[0-9.]+|192\.168\.[0-9.]+)(/[0-9]+)?\b"
)
_HEADCOUNT_RE = re.compile(
    r"\b([0-9]+) (employees|developers|engineers|staff|people|team members)\b"
)
_REGISTRY_RE = re.compile(
    r"[a-zA-Z0-9.-]+\.(azurecr\.io|gcr\.io|ecr\.[a-z]+|registry\.[a-z]+)[^\s\"]*"
)
_LOCATION_RE = re.compile(
    r"\b(?i:office|building|floor|campus|headquarters|HQ)\s+[A-Z0-9]"
)
_CUSTOMER_RE = re.compile(r"\b(client|customer|partner|vendor)\s+[A-Z][a-z]+")
_UUID_RE = re.compile(
    r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"
)
_LINKING_RE = re.compile(
    r"(parent|sister|sibling|upstream|downstream|forked from|based on|predecessor|successor)"
    r"\s+(project|repo|repository|codebase)",
    re.IGNORECASE,
)
_ARCH_RE = re.compile(
    r"\b(database|cache|queue|broker|gateway|proxy|load.balancer|CDN|scheduler"
    r"|worker|cron|webhook|API|microservice|monolith)\b",
    re.IGNORECASE,
)
_INTEGRATION_RE = re.compile(
    r"\b(OAuth|OIDC|SAML|LDAP|SMTP|webhook|SSO|S3|SQS|SNS|pub.?sub"
    r"|gRPC|GraphQL|REST\s+API)\b",
    re.IGNORECASE,
)
_BIZ_PROCESS_RE = re.compile(
    r"\b(onboarding|checkout|billing|invoice|subscription|approval|workflow"
    r"|pipeline|notification|escalation|audit.trail|compliance.check|risk.assessment)\b",
    re.IGNORECASE,
)
_NICHE_DOMAIN_RE = re.compile(r"^[a-z]+-[a-z]+-[a-z]+")
_NICHE_LANG_RE = re.compile(
    r"\b(CUE|SHACL|Turtle|Elm|Haskell|OCaml|Erlang|Prolog|Fortran|COBOL|Ada|Zig|Nim)\b",
    re.IGNORECASE,
)
_STANDARD_LIFECYCLE: frozenset[str] = frozenset({
    "greenfield",
    "early-development",
    "scaling",
    "legacy-migration",
    "maintenance",
})


@dataclass(slots=True)
class _ScreeningState:
    """Mutable accumulator for a single screening run."""

    blocks: int = 0
    warns: int = 0
    acked: int = 0
    file_finding_num: int = 0
    current_file: Path = field(default_factory=lambda: Path("."))
    ack_entries: dict[str, str] = field(default_factory=dict)

    def _next_id(self, category: str) -> str:
        self.file_finding_num += 1
        return f"ISEC-{category}-{self.file_finding_num:03d}"

    def block(self, category: str, name: str, msg: str) -> str:
        fid = self._next_id(category)
        print_block(f"{fid}  {name}: {msg}")
        if fid in self.ack_entries:
            print_ack("")
            self.acked += 1
        else:
            self.blocks += 1
            print(f"       Add sentinel_ack with finding_id: \"{fid}\" to acknowledge")
        return fid

    def flag(self, category: str, name: str, msg: str) -> str:
        fid = self._next_id(category)
        warn(f"{fid}  {name}: {msg}")
        self.warns += 1
        return fid


def _load_ack_entries(data: dict[str, Any]) -> dict[str, str]:
    """Extract sentinel_ack entries as {finding_id: decision} mapping."""
    acks = data.get("sentinel_ack")
    if not isinstance(acks, list):
        return {}
    result: dict[str, str] = {}
    for entry in acks:
        if isinstance(entry, dict):
            fid = entry.get("finding_id", "")
            decision = entry.get("decision", "")
            if fid:
                result[str(fid)] = str(decision)
    return result


def _load_sidecar_acks(path: Path) -> dict[str, str]:
    """Load ack entries from a .infosec-ack.yaml sidecar file.

    The sidecar file sits next to the screened file and contains
    sentinel_ack entries keyed by filename, e.g.:

        LESSONS-LEARNED.md:
          - finding_id: "ISEC-CROSSRECORD-001"
            decision: "false_positive"
            ...

    This enables human acknowledgment for non-YAML files (markdown,
    reviews) that cannot embed sentinel_ack entries themselves.
    """
    sidecar = path.parent / ".infosec-ack.yaml"
    if not sidecar.exists():
        return {}
    data = load_yaml(sidecar)
    file_acks = data.get(path.name)
    if not isinstance(file_acks, list):
        return {}
    result: dict[str, str] = {}
    for entry in file_acks:
        if isinstance(entry, dict):
            fid = entry.get("finding_id", "")
            decision = entry.get("decision", "")
            if fid:
                result[str(fid)] = str(decision)
    return result


def _strip_comments(text: str) -> str:
    """Remove comment lines from text."""
    return "\n".join(
        line for line in text.splitlines()
        if not line.strip().startswith("#")
    )


def _screen_credentials(state: _ScreeningState, name: str, content: str) -> None:
    """Category A: Credential & secret detection."""
    if _VCS_TOKEN_RE.search(content):
        m = _VCS_TOKEN_RE.search(content)
        state.block("CRED", name, f"VCS/service token detected: {m.group()[:20]}...")  # type: ignore[union-attr]
        info("[token redacted]")

    if _DB_CONN_RE.search(content):
        m = _DB_CONN_RE.search(content)
        state.block("CRED", name, f"Database connection string: {m.group()[:40]}...")  # type: ignore[union-attr]
        info("[database connection string redacted]")

    if _API_KEY_RE.search(content):
        m = _API_KEY_RE.search(content)
        state.block("CRED", name, f"Credential assignment: {m.group()[:40]}...")  # type: ignore[union-attr]
        info("[credential redacted]")

    if _PRIVATE_KEY_RE.search(content):
        state.block("CRED", name, "Private key material detected")
        info("[private key redacted]")

    if _USER_PASS_RE.search(content):
        state.block("CRED", name, "Username + password pair detected")
        info("[credentials redacted]")


def _screen_details(state: _ScreeningState, name: str, content: str) -> None:
    """Category B: Sensitive internal detail detection."""
    if _DDL_RE.search(content):
        m = _DDL_RE.search(content)
        state.block("DETAIL", name, f"Database DDL statement: {m.group()[:60]}")  # type: ignore[union-attr]
        info("[database schema detail redacted]")

    m = _FILE_PATH_RE.search(content)
    if m:
        state.block("DETAIL", name, f"Absolute file path: {m.group()}")
        info("[file path redacted]")

    m = _RFC1918_RE.search(content)
    if m:
        state.block("DETAIL", name, f"Internal IP address: {m.group()}")
        info("[internal IP redacted]")

    m = _HEADCOUNT_RE.search(content)
    if m:
        num = int(m.group(1))
        lo = (num // 10) * 10
        hi = lo + 10
        state.block("DETAIL", name, f"Exact headcount: {m.group()}")
        info(f"Replace with range: {lo}-{hi}")

    m = _REGISTRY_RE.search(content)
    if m:
        state.block("DETAIL", name, f"Container registry URL: {m.group()}")
        info("[container registry redacted]")


def _screen_crossfield(
    state: _ScreeningState, name: str, data: dict[str, Any]
) -> None:
    """Category C: Cross-field re-identification scoring."""
    specificity = 0
    details: list[str] = []

    # Niche business domain
    domain = str(data.get("business_domain", {}).get("primary", "") or "")
    if _NICHE_DOMAIN_RE.match(domain):
        specificity += 1
        details.append(f"  niche domain: {domain}")

    # Multiple secondary domains
    secondary = data.get("business_domain", {}).get("secondary", [])
    if isinstance(secondary, list) and len(secondary) >= 2:
        specificity += 1
        details.append(f"  {len(secondary)} secondary domains")

    # Unusual language combination
    codebase = data.get("codebase", {})
    languages = codebase.get("languages", []) if isinstance(codebase, dict) else []
    if isinstance(languages, list):
        niche_count = sum(
            1 for lang in languages if _NICHE_LANG_RE.search(str(lang))
        )
        if len(languages) >= 4 or niche_count >= 1:
            specificity += 1
            details.append(
                f"  unusual languages ({len(languages)} languages, {niche_count} niche)"
            )

    # Regulatory frameworks beyond GDPR
    reg = data.get("regulatory_context", {})
    frameworks = reg.get("frameworks", []) if isinstance(reg, dict) else []
    if isinstance(frameworks, list):
        non_gdpr = [f for f in frameworks if str(f).upper() != "GDPR"]
        if len(non_gdpr) >= 1:
            specificity += 1
            details.append(f"  {len(non_gdpr)} regulatory frameworks beyond GDPR")

    # Solo developer
    team = data.get("team", {})
    team_size = team.get("size") if isinstance(team, dict) else None
    if str(team_size) == "1":
        specificity += 1
        details.append("  solo developer (team size 1)")

    # Non-standard lifecycle
    lifecycle = data.get("lifecycle_state", {})
    current = str(lifecycle.get("current", "") or "") if isinstance(lifecycle, dict) else ""
    if current and current not in _STANDARD_LIFECYCLE:
        specificity += 1
        details.append(f"  non-standard lifecycle: {current}")

    if specificity >= 6:
        state.block(
            "CROSSFIELD", name,
            f"High re-identification risk (specificity score: {specificity}/6+)",
        )
        for d in details:
            print(d)
        info("Generalize niche fields or remove the most identifying combination")
    elif specificity >= 4:
        state.flag(
            "CROSSFIELD", name,
            f"Moderate re-identification risk (specificity score: {specificity})",
        )
        for d in details:
            print(d)


def _screen_cumulative(
    state: _ScreeningState, name: str, data: dict[str, Any]
) -> None:
    """Category D: Cumulative cross-entry analysis."""
    texts = extract_freetext(data)
    if not texts:
        return

    combined = " ".join(texts)

    arch = len(set(m.lower() for m in _ARCH_RE.findall(combined)))
    integ = len(set(m.lower() for m in _INTEGRATION_RE.findall(combined)))
    biz = len(set(m.lower() for m in _BIZ_PROCESS_RE.findall(combined)))
    cumulative = arch + integ + biz

    if cumulative >= 8:
        state.block(
            "CUMULATIVE", name,
            f"High cumulative detail: {arch} architecture, {integ} integrations, "
            f"{biz} business process terms across all entries",
        )
        info("Review whether the aggregate reveals reconstructable system design")
    elif cumulative >= 5:
        state.flag(
            "CUMULATIVE", name,
            f"Moderate cumulative detail: {arch} architecture, {integ} integrations, "
            f"{biz} business process terms",
        )


def _screen_crossrecord(
    state: _ScreeningState,
    name: str,
    text: str,
    current_codename: str,
    all_codenames: frozenset[str],
) -> None:
    """Category E: Cross-record linkage."""
    for cn in sorted(all_codenames):
        if cn == current_codename:
            continue
        if re.search(rf"\b{re.escape(cn)}\b", text, re.IGNORECASE):
            state.block("CROSSRECORD", name, f"References another project codename: {cn}")
            info("Remove cross-project reference or use generic description")

    # Linking phrases
    content = _strip_comments(text)
    m = _LINKING_RE.search(content)
    if m:
        state.block("CROSSRECORD", name, f"Cross-project linking phrase: {m.group()}")
        info("Replace with generic reference or remove the link")


def _screen_semantic(state: _ScreeningState, name: str, content: str) -> None:
    """Category F: Semantic leak in free-text."""
    m = _LOCATION_RE.search(content)
    if m:
        # Get the full match context
        full = re.search(
            r"\b(office|building|floor|campus|headquarters|HQ)\s+\S+",
            content, re.IGNORECASE,
        )
        state.block("SEMANTIC", name, f"Location identifier: {full.group() if full else m.group()}")
        info("[location redacted]")

    m = _CUSTOMER_RE.search(content)
    if m:
        state.block("SEMANTIC", name, f"Named customer/partner: {m.group()}")
        info("Replace with generic: 'a customer', 'an integration partner'")

    # UUIDs outside framework_commit
    no_commit = "\n".join(
        line for line in content.splitlines()
        if "framework_commit" not in line
    )
    if _UUID_RE.search(no_commit):
        state.block("SEMANTIC", name, "UUID detected (may identify specific records or systems)")
        info("[identifier redacted]")


def validate_infosec(
    files: tuple[Path, ...] | None = None,
    *,
    include_md: bool = False,
) -> int:
    """Screen files for information leaks. Returns exit code."""
    root = repo_root()

    if files is None:
        file_list: list[Path] = list(find_learnings(root))
        if include_md:
            ll = root / "LESSONS-LEARNED.md"
            if ll.exists():
                file_list.append(ll)
            reviews_dir = root / "reviews"
            if reviews_dir.is_dir():
                file_list.extend(sorted(reviews_dir.glob("*.md")))
        files = tuple(file_list)

    if not files:
        print("No files to screen.")
        return 0

    # Collect all codenames for cross-record checks
    all_codenames: frozenset[str] = frozenset(
        p.stem for p in find_learnings(root)
    )

    print(f"InfoSec Sentinel: screening {len(files)} file(s)...")
    print()

    state = _ScreeningState()

    for path in files:
        name = path.name
        state.current_file = path
        state.file_finding_num = 0

        print(f"--- Screening: {name} ---")

        if not path.exists():
            print(f"  (file not found, skipping)")
            print()
            continue

        text = path.read_text(encoding="utf-8")
        content = _strip_comments(text)

        # Load ack entries: from sentinel_ack in YAML files,
        # or from .infosec-ack.yaml sidecar for non-YAML files
        if path.suffix in (".yaml", ".yml"):
            data = load_yaml(path)
            state.ack_entries = _load_ack_entries(data)
        else:
            data = {}
            state.ack_entries = _load_sidecar_acks(path)

        # A. Credentials
        _screen_credentials(state, name, content)

        # B. Sensitive details
        _screen_details(state, name, content)

        # C. Cross-field re-identification (YAML only)
        if path.suffix in (".yaml", ".yml"):
            _screen_crossfield(state, name, data)

        # D. Cumulative analysis (YAML only)
        if path.suffix in (".yaml", ".yml"):
            _screen_cumulative(state, name, data)

        # E. Cross-record linkage
        if path.suffix in (".yaml", ".yml"):
            current_codename = path.stem
            _screen_crossrecord(state, name, text, current_codename, all_codenames)
        else:
            # For MD files, still check linking phrases
            m = _LINKING_RE.search(content)
            if m:
                state.block("CROSSRECORD", name, f"Cross-project linking phrase: {m.group()}")
                info("Replace with generic reference or remove the link")

        # F. Semantic leaks
        _screen_semantic(state, name, content)

        print()

    # Summary
    total_findings = state.blocks + state.warns + state.acked
    print("--- InfoSec Sentinel Summary ---")
    print(f"Files screened: {len(files)}")
    print(f"Findings:     {total_findings}")
    print(f"  Blocked:    {state.blocks} (unacknowledged)")
    print(f"  Warned:     {state.warns}")
    print(f"  Acked:      {state.acked} (human confirmed)")

    if state.blocks > 0:
        print(f"\n\033[0;31mScreening blocked.\033[0m {state.blocks} finding(s) require human acknowledgment.")
        print("Add sentinel_ack entries to the YAML file for each ISEC-* finding.")
        print("See learnings/_template.yaml for the sentinel_ack format.")
        return 1
    elif state.warns > 0:
        print(f"\n\033[0;33mPassed with warnings.\033[0m Review flagged items before merge.")
        return 0
    else:
        print(f"\n\033[0;32mAll clear.\033[0m")
        return 0


_HELP = """\
validate-infosec — Screen learning records for information leaks

Usage:
  uv run agov-validate-infosec [OPTIONS] [FILES...]

Screens YAML learning records for 6 categories of information leaks:
  A. Credentials (tokens, connection strings, API keys, private keys)
  B. Sensitive details (DDL, file paths, internal IPs, headcounts)
  C. Cross-field re-identification (niche domain + solo dev + rare lang)
  D. Cumulative detail (architecture/integration/process terms)
  E. Cross-record linkage (references to other project codenames)
  F. Semantic leaks (locations, customer names, UUIDs)

Options:
  --include-md    Also screen LESSONS-LEARNED.md and reviews/*.md
  --help, -h      Show this help message

If no FILES are given, screens all files in learnings/ (excluding template).

Findings can be acknowledged with sentinel_ack entries in the YAML file
(for YAML files) or .infosec-ack.yaml sidecar files (for markdown).

Exit codes: 0 = pass (or warnings only), 1 = unacknowledged findings.

Full documentation: LEARNINGS.md and agents/infosec-sentinel.yaml.
"""


def main() -> None:
    include_md = False
    file_args: list[str] = []

    for arg in sys.argv[1:]:
        if arg in ("--help", "-h"):
            print(_HELP)
            return
        elif arg == "--include-md":
            include_md = True
        else:
            file_args.append(arg)

    files: tuple[Path, ...] | None = None
    if file_args:
        files = tuple(Path(a) for a in file_args)

    sys.exit(validate_infosec(files, include_md=include_md))


if __name__ == "__main__":
    main()
