# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Validate agent definition YAML files for structural consistency."""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Any

from agents_govern._files import find_agents, repo_root
from agents_govern._output import fail, ok, warn
from agents_govern._yaml import load_yaml

__all__ = ["validate_agents", "main", "_extract_coordination_agents"]

VALID_TIERS: frozenset[str] = frozenset({
    "production",
    "governance",
    "cross-cutting",
    "publication-support",
    "meta",
})


def _get_nested(data: dict[str, Any], *keys: str) -> Any:
    """Safely traverse nested dict keys."""
    current: Any = data
    for key in keys:
        if not isinstance(current, dict):
            return None
        current = current.get(key)
    return current


def _extract_comm_agents(
    comm: dict[str, Any], direction: str,
    *,
    include_legacy: bool = True,
) -> set[str]:
    """Extract agent names from a communication section (receives_from or sends_to).

    Handles three formats:
    1. Structured: list of dicts with 'agent' key and 'message_types'
    2. Simple: list of strings like "agent-name (description)"
    3. Legacy: 'reports_to', 'coordinates_with', and 'consults' fields
       (mapped to sends_to and bidirectional respectively)

    Set include_legacy=False to extract only explicit sends_to/receives_from.
    """
    names: set[str] = set()

    # Standard format: receives_from / sends_to
    entries = comm.get(direction)
    if isinstance(entries, list):
        for entry in entries:
            if isinstance(entry, dict) and "agent" in entry:
                name = str(entry["agent"]).strip()
                if name and name != "human":
                    names.add(name)
            elif isinstance(entry, str):
                name = entry.split("(")[0].split("—")[0].strip()
                if name and name.lower() != "human" and not name.startswith("human"):
                    names.add(name)

    if not include_legacy:
        return names

    # Legacy format: reports_to (→ treat as sends_to target)
    if direction == "sends_to":
        reports_to = comm.get("reports_to")
        if isinstance(reports_to, str):
            name = reports_to.strip()
            if name and name.lower() != "human":
                names.add(name)

    # Legacy format: coordinates_with (→ treat as bidirectional)
    coordinates = comm.get("coordinates_with")
    if isinstance(coordinates, list):
        for entry in coordinates:
            if isinstance(entry, dict) and "agent" in entry:
                name = str(entry["agent"]).strip()
            elif isinstance(entry, str):
                name = entry.split("(")[0].split("—")[0].strip()
            else:
                continue
            if name and name.lower() != "human" and not name.startswith("human"):
                names.add(name)

    # Legacy format: consults (→ treat as sends_to target, advisory channel)
    if direction == "sends_to":
        consults = comm.get("consults")
        if isinstance(consults, list):
            for entry in consults:
                if isinstance(entry, dict) and "agent" in entry:
                    name = str(entry["agent"]).strip()
                elif isinstance(entry, str):
                    name = entry.split("(")[0].split("—")[0].strip()
                else:
                    continue
                if name and name.lower() != "human" and not name.startswith("human"):
                    names.add(name)

    return names


def _extract_coordination_agents(
    comm: dict[str, Any],
) -> set[str]:
    """Extract agent names from coordinates_with, consults, and reports_to.

    These represent informal/advisory communication channels, distinct
    from the formal sends_to/receives_from pairs. Used to render a
    third edge style in communication graphs.
    """
    names: set[str] = set()

    def _add_entries(entries: Any) -> None:
        if isinstance(entries, list):
            for entry in entries:
                if isinstance(entry, dict) and "agent" in entry:
                    name = str(entry["agent"]).strip()
                elif isinstance(entry, str):
                    name = entry.split("(")[0].split("—")[0].strip()
                else:
                    continue
                if name and name.lower() != "human" and not name.startswith("human"):
                    names.add(name)
        elif isinstance(entries, str):
            name = entries.strip()
            if name and name.lower() != "human":
                names.add(name)

    _add_entries(comm.get("coordinates_with"))
    _add_entries(comm.get("consults"))
    _add_entries(comm.get("reports_to"))
    return names


def _check_communication_symmetry(
    all_agents: dict[str, dict[str, Any]],
) -> list[str]:
    """Check that sends_to/receives_from are symmetric across agents.

    For every agent A that sends_to agent B, verify that B lists
    receives_from agent A (and vice versa).

    Returns a list of warning messages.
    """
    warnings: list[str] = []

    for agent_name, data in all_agents.items():
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            continue

        sends_to = _extract_comm_agents(comm, "sends_to")
        receives_from = _extract_comm_agents(comm, "receives_from")

        for target in sends_to:
            if target not in all_agents:
                continue  # target agent not in this set (e.g., sentinel, auditor)
            target_comm = all_agents[target].get("communication", {})
            if not isinstance(target_comm, dict):
                continue
            target_receives = _extract_comm_agents(target_comm, "receives_from")
            if agent_name not in target_receives:
                warnings.append(
                    f"{agent_name} sends_to {target}, "
                    f"but {target} does not list {agent_name} in receives_from"
                )

        for source in receives_from:
            if source not in all_agents:
                continue
            source_comm = all_agents[source].get("communication", {})
            if not isinstance(source_comm, dict):
                continue
            source_sends = _extract_comm_agents(source_comm, "sends_to")
            if agent_name not in source_sends:
                warnings.append(
                    f"{agent_name} receives_from {source}, "
                    f"but {source} does not list {agent_name} in sends_to"
                )

    # Deduplicate (A→B missing and B←A missing produce the same issue)
    return sorted(set(warnings))


def _check_communication_completeness(
    all_agents: dict[str, dict[str, Any]],
) -> list[str]:
    """Check for agents that only send or only receive.

    An agent with receives_from but no sends_to is structurally silent —
    its findings never reach the governance pipeline. An agent with
    sends_to but no receives_from never gets tasked.

    Agents without a communication section are ignored (may be skipped).
    Agents with only coordination fields (coordinates_with, consults,
    reports_to) are not flagged — they have an informal channel.

    Returns a list of warning messages.
    """
    warnings: list[str] = []

    for agent_name, data in all_agents.items():
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            continue
        # Skip agents with no communication section at all
        if not comm:
            continue

        sends = _extract_comm_agents(comm, "sends_to", include_legacy=False)
        receives = _extract_comm_agents(comm, "receives_from", include_legacy=False)
        coords = _extract_coordination_agents(comm)

        has_sends = len(sends) > 0
        has_receives = len(receives) > 0
        has_coords = len(coords) > 0

        if has_receives and not has_sends and not has_coords:
            warnings.append(
                f"{agent_name}: receive-only (no sends_to) — "
                f"findings are structurally silent"
            )
        elif has_sends and not has_receives and not has_coords:
            warnings.append(
                f"{agent_name}: send-only (no receives_from) — "
                f"never tasked by other agents"
            )

    return sorted(warnings)


def _check_ownership_overlaps(
    all_agents: dict[str, dict[str, Any]],
) -> list[str]:
    """Check for artifact ownership overlaps across agents.

    Compares the 'owns' lists in each agent's jurisdiction. If two agents
    own artifacts with overlapping keywords, flag it as a warning.

    Uses keyword extraction: each owns entry is split into lowercase words
    and compared for significant overlap with other agents' entries.

    Returns a list of warning messages.
    """
    # Extract normalized ownership tokens per agent
    agent_owns: dict[str, list[str]] = {}
    for agent_name, data in all_agents.items():
        identity = data.get("identity", {})
        jurisdiction = identity.get("jurisdiction", {}) if isinstance(identity, dict) else {}
        owns = jurisdiction.get("owns", []) if isinstance(jurisdiction, dict) else []
        if isinstance(owns, list):
            agent_owns[agent_name] = [str(o).lower() for o in owns]

    # Noise words that appear in many ownership descriptions
    noise = frozenset({
        "within", "scope", "of", "approved", "tasks", "and", "the", "a",
        "for", "from", "to", "in", "with", "by", "on", "or", "as", "an",
        "all", "any", "that", "this", "which", "if", "not", "are", "is",
        "files", "owned", "e.g.", "etc.", "etc", "e.g",
    })

    def _keywords(text: str) -> set[str]:
        """Extract meaningful keywords from an ownership description."""
        # Remove parenthetical notes and punctuation
        clean = text.replace("(", " ").replace(")", " ").replace(",", " ")
        words = {w.strip(".-_") for w in clean.split() if len(w) > 2}
        return words - noise

    warnings: list[str] = []
    agents = sorted(agent_owns.keys())
    for i, a1 in enumerate(agents):
        for a2 in agents[i + 1:]:
            for desc1 in agent_owns[a1]:
                kw1 = _keywords(desc1)
                if len(kw1) < 2:
                    continue
                for desc2 in agent_owns[a2]:
                    kw2 = _keywords(desc2)
                    if len(kw2) < 2:
                        continue
                    overlap = kw1 & kw2
                    # Require 3+ shared keywords to flag (reduces false positives)
                    if len(overlap) >= 3:
                        warnings.append(
                            f"Ownership overlap between {a1} and {a2}: "
                            f"shared keywords [{', '.join(sorted(overlap))}] "
                            f"— review jurisdictions"
                        )
    return sorted(set(warnings))


def validate_agents(files: tuple[Path, ...] | None = None, *, strict: bool = False) -> int:
    """Validate agent definition files. Returns exit code.

    With strict=True, communication asymmetries and completeness
    warnings are treated as errors (exit code 1).
    """
    root = repo_root()
    if files is None:
        files = find_agents(root)

    if not files:
        print("No agent files found.")
        return 0

    print(f"Agent definition validation: {len(files)} file(s)")
    print()

    total_errors = 0
    total_warnings = 0
    all_agents: dict[str, dict[str, Any]] = {}

    for path in files:
        name = path.name
        file_errors = 0

        if not path.exists():
            fail(f"{name}: file not found")
            total_errors += 1
            continue

        # 0. SPDX license header check
        raw = path.read_text(encoding="utf-8")
        if "SPDX-License-Identifier:" not in raw:
            warn(f"{name}: missing SPDX-License-Identifier header")
            total_warnings += 1
        if "Copyright" not in raw:
            warn(f"{name}: missing Copyright header")
            total_warnings += 1

        data = load_yaml(path)
        if not data:
            fail(f"{name}: failed to parse YAML")
            total_errors += 1
            continue

        # 1. Required top-level sections
        for section in ("agent", "identity", "boundaries"):
            if section not in data:
                fail(f"{name}: missing top-level section: {section}")
                file_errors += 1

        if file_errors > 0:
            total_errors += file_errors
            continue

        agent = data.get("agent", {})
        if not isinstance(agent, dict):
            fail(f"{name}: 'agent' is not a mapping")
            total_errors += 1
            continue

        # 2. Required agent fields
        required_fields = ("name", "id", "version", "framework_version", "tier", "phase")
        for field in required_fields:
            if field not in agent or agent[field] is None:
                fail(f"{name}: missing agent.{field}")
                file_errors += 1

        if file_errors > 0:
            total_errors += file_errors
            continue

        agent_name = str(agent["name"])
        agent_id = str(agent["id"])
        agent_version = str(agent["version"])
        agent_tier = str(agent["tier"])

        # 3. ID format: agents-govern:<name>:<version>
        expected_id = f"agents-govern:{agent_name}:{agent_version}"
        if agent_id != expected_id:
            fail(f"{name}: id '{agent_id}' does not match expected '{expected_id}'")
            file_errors += 1

        # 4. Tier validation
        if agent_tier not in VALID_TIERS:
            fail(f"{name}: unknown tier '{agent_tier}' (expected: {', '.join(sorted(VALID_TIERS))})")
            file_errors += 1

        # 5. Required identity fields (role_mapping optional for single-role agents)
        identity = data.get("identity", {})
        if not isinstance(identity, dict) or "purpose" not in identity:
            fail(f"{name}: missing identity.purpose")
            file_errors += 1

        if file_errors == 0:
            ok(name)
            all_agents[agent_name] = data
        else:
            total_errors += file_errors

    # Cross-agent checks (only if per-file validation passed for all)
    if total_errors == 0 and len(all_agents) >= 2:
        print()
        print("Cross-agent consistency checks:")
        print()

        # Communication symmetry
        comm_warnings = _check_communication_symmetry(all_agents)
        if comm_warnings:
            report = fail if strict else warn
            for w in comm_warnings:
                report(f"Communication asymmetry: {w}")
            if strict:
                total_errors += len(comm_warnings)
            else:
                total_warnings += len(comm_warnings)
        else:
            ok("Communication symmetry: all sends_to/receives_from pairs are symmetric")

        # Communication completeness (send-only / receive-only)
        completeness_warnings = _check_communication_completeness(all_agents)
        if completeness_warnings:
            report = fail if strict else warn
            for w in completeness_warnings:
                report(f"Communication: {w}")
            if strict:
                total_errors += len(completeness_warnings)
            else:
                total_warnings += len(completeness_warnings)
        else:
            ok("Communication completeness: all agents have both sends_to and receives_from")

        # Artifact ownership overlaps
        ownership_warnings = _check_ownership_overlaps(all_agents)
        if ownership_warnings:
            for w in ownership_warnings:
                warn(f"Ownership: {w}")
            total_warnings += len(ownership_warnings)
        else:
            ok("Artifact ownership: no overlapping jurisdictions detected")

        # Manifest check (if present)
        from agents_govern.manifest import validate_manifest
        manifest_errors, manifest_warnings = validate_manifest(root)
        total_errors += manifest_errors
        total_warnings += manifest_warnings

        # Phase distribution summary (for human review against framework.md)
        phase_map: dict[int, list[str]] = {}
        for agent_name, data in sorted(all_agents.items()):
            agent_sec = data.get("agent", {})
            phase = agent_sec.get("phase", "?") if isinstance(agent_sec, dict) else "?"
            phase_int = int(phase) if isinstance(phase, (int, float)) else -1
            phase_map.setdefault(phase_int, []).append(agent_name)
        print()
        print("Phase distribution (verify against framework.md Section 7):")
        for phase_num in sorted(phase_map.keys()):
            agents_in_phase = phase_map[phase_num]
            print(f"  Phase {phase_num}: {len(agents_in_phase)} agent(s) — {', '.join(agents_in_phase)}")

    # Summary
    print()
    if total_errors > 0:
        print(f"\033[0;31m{total_errors} error(s) found.\033[0m")
        return 1
    if total_warnings > 0:
        print(f"\033[0;33mAll {len(files)} agent definitions valid, {total_warnings} warning(s).\033[0m")
        return 0

    print(f"\033[0;32mAll {len(files)} agent definitions valid.\033[0m")
    return 0


def _generate_comm_graph(
    all_agents: dict[str, dict[str, Any]],
) -> str:
    """Generate a Mermaid graph of agent communication with asymmetry highlights.

    Symmetric links are solid arrows. Asymmetric links are dashed red.
    """
    lines: list[str] = ["graph LR"]

    # Collect explicit sends_to/receives_from edges (exclude legacy fields)
    declared_sends: set[tuple[str, str]] = set()  # (from, to)
    declared_receives: set[tuple[str, str]] = set()  # (to, from) stored as (from, to)

    for agent_name, data in all_agents.items():
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            continue
        for target in _extract_comm_agents(comm, "sends_to", include_legacy=False):
            declared_sends.add((agent_name, target))
        for source in _extract_comm_agents(comm, "receives_from", include_legacy=False):
            declared_receives.add((source, agent_name))

    # Coordination edges (coordinates_with, consults, reports_to)
    coordination_edges: set[tuple[str, str]] = set()
    for agent_name, data in all_agents.items():
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            continue
        for target in _extract_coordination_agents(comm):
            if target in all_agents:
                coordination_edges.add((agent_name, target))

    # All unique edges (union of both directions)
    all_edges = declared_sends | declared_receives

    # Classify
    symmetric = declared_sends & declared_receives
    only_sends = declared_sends - declared_receives  # A says sends_to B, B doesn't list receives_from A
    only_receives = declared_receives - declared_sends  # B says receives_from A, A doesn't list sends_to B

    # Coordination edges that aren't already covered by formal channels
    coord_only = coordination_edges - declared_sends - declared_receives

    # Node styling by tier
    tier_map: dict[str, str] = {}
    for agent_name, data in all_agents.items():
        agent_sec = data.get("agent", {})
        tier_map[agent_name] = str(agent_sec.get("tier", ""))

    tier_shapes = {
        "production": ("[", "]"),
        "governance": ("[[", "]]"),
        "cross-cutting": ("([", "])"),
        "meta": ("{{", "}}"),
        "publication-support": ("(", ")"),
    }

    # Declare nodes
    for agent_name in sorted(all_agents.keys()):
        tier = tier_map.get(agent_name, "")
        left, right = tier_shapes.get(tier, ("[", "]"))
        short = agent_name.replace("-", "\\n")
        lines.append(f"    {agent_name}{left}\"{short}\"{right}")

    lines.append("")

    # Symmetric edges (solid)
    for src, tgt in sorted(symmetric):
        lines.append(f"    {src} --> {tgt}")

    # Asymmetric edges (dashed, with note)
    for src, tgt in sorted(only_sends):
        lines.append(f"    {src} -.->|sends but not received| {tgt}")
    for src, tgt in sorted(only_receives):
        lines.append(f"    {src} -.->|received but not sent| {tgt}")

    # Coordination edges (dotted, blue)
    for src, tgt in sorted(coord_only):
        lines.append(f"    {src} -.->|coordinates/consults| {tgt}")

    # Style asymmetric edges
    if only_sends or only_receives:
        lines.append("")
        lines.append("    %% Red dashed = asymmetric (one side missing)")
        lines.append("    linkStyle default stroke:#333")

    if coord_only:
        lines.append("    %% Blue dotted = coordination/consultation (informal channel)")

    return "\n".join(lines)


def generate_versioned_markdown(
    all_agents: dict[str, dict[str, Any]],
    version: str,
) -> str:
    """Generate a versioned markdown document with agent index and Mermaid graph.

    Returns the markdown content as a string. The caller writes to file.
    """
    lines: list[str] = []
    lines.append(f"# Agent Communication Graph — v{version}")
    lines.append("")
    lines.append("Generated by `validate-agents --graph`.")
    lines.append("")

    # Agent index table
    lines.append("## Agent Index")
    lines.append("")
    lines.append("| Agent | Tier | Phase | Sends To | Receives From | Coordination |")
    lines.append("|-------|------|-------|----------|---------------|--------------|")

    for name in sorted(all_agents.keys()):
        data = all_agents[name]
        agent_sec = data.get("agent", {})
        tier = str(agent_sec.get("tier", "")) if isinstance(agent_sec, dict) else ""
        phase = str(agent_sec.get("phase", "")) if isinstance(agent_sec, dict) else ""
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            comm = {}
        sends = _extract_comm_agents(comm, "sends_to", include_legacy=False)
        receives = _extract_comm_agents(comm, "receives_from", include_legacy=False)
        coords = _extract_coordination_agents(comm)
        lines.append(
            f"| {name} | {tier} | {phase} "
            f"| {len(sends)} | {len(receives)} | {len(coords)} |"
        )

    lines.append("")

    # Summary statistics
    all_sends: set[tuple[str, str]] = set()
    all_receives: set[tuple[str, str]] = set()
    all_coords: set[tuple[str, str]] = set()
    for agent_name, data in all_agents.items():
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            continue
        for target in _extract_comm_agents(comm, "sends_to", include_legacy=False):
            if target in all_agents:
                all_sends.add((agent_name, target))
        for source in _extract_comm_agents(comm, "receives_from", include_legacy=False):
            if source in all_agents:
                all_receives.add((source, agent_name))
        for target in _extract_coordination_agents(comm):
            if target in all_agents:
                all_coords.add((agent_name, target))

    symmetric = all_sends & all_receives
    only_sends = all_sends - all_receives
    only_receives = all_receives - all_sends
    coord_only = all_coords - all_sends - all_receives

    lines.append("## Summary")
    lines.append("")
    lines.append(f"- **Agents:** {len(all_agents)}")
    lines.append(f"- **Symmetric pairs:** {len(symmetric)}")
    lines.append(f"- **Asymmetric (sends only):** {len(only_sends)}")
    lines.append(f"- **Asymmetric (receives only):** {len(only_receives)}")
    lines.append(f"- **Coordination edges:** {len(coord_only)}")
    lines.append("")

    # Mermaid graph
    lines.append("## Communication Graph")
    lines.append("")
    lines.append("```mermaid")
    lines.append(_generate_comm_graph(all_agents))
    lines.append("```")
    lines.append("")

    # Asymmetry warnings
    comm_warnings = _check_communication_symmetry(all_agents)
    if comm_warnings:
        lines.append("## Communication Asymmetries")
        lines.append("")
        for w in comm_warnings:
            lines.append(f"- {w}")
        lines.append("")

    return "\n".join(lines)


_HELP = """\
validate-agents — Validate agent definition YAML files

Usage:
  uv run agov-validate-agents [OPTIONS] [FILES...]

Validation mode (default):
  Checks each agent YAML for structural correctness (required fields,
  ID format, tier membership). Then runs cross-agent checks:
  - Communication symmetry (sends_to/receives_from pairs)
  - Artifact ownership overlaps
  - Phase distribution summary

Graph mode:
  --graph                  Generate communication diagram (default: SVG)
  --format FORMAT          Output format: svg, png, pdf (default: svg)
  --output PATH            Output file or directory path. If directory, writes
                           versioned file: agent-communication-{version}.md
  --detail LEVEL           Detail level: minimal, default, detailed
  --filter-tier TIER       Show only agents in this tier (e.g., production)
  --filter-phase PHASE     Show only agents in this phase (e.g., 1)
  --engine ENGINE          Layout engine: dot (default), fdp, sfdp, neato, circo, twopi
                           Use fdp/sfdp for large graphs (20+ agents)
  --mermaid                Output Mermaid text instead of rendered graph
  --check                  Check if existing versioned markdown is stale (exit 1 if so)
  --strict                 Treat communication asymmetries and completeness
                           warnings as errors (exit 1). For zero-asymmetry enforcement.

Detail levels:
  minimal   — agent names + communication edges (clean overview)
  default   — + tier grouping, version, phase, maturity borders, legend
  detailed  — + message types on edges, escalation paths

Other:
  --help                   Show this help message

Requires: graphviz system binary for rendered output (SVG/PNG/PDF).
          If missing, saves .dot source file for offline rendering.

Full documentation: ADOPTING.md Step 1.3 and framework.md Section 6.
"""


def main() -> None:
    graph_mode = False
    graph_format = "svg"
    graph_output: str | None = None
    graph_detail = "default"
    graph_engine = "dot"
    graph_check = False
    strict_mode = False
    filter_tier: str | None = None
    filter_phase: int | None = None
    file_args: list[str] = []

    args = sys.argv[1:]
    i = 0
    while i < len(args):
        if args[i] in ("--help", "-h"):
            print(_HELP)
            return
        elif args[i] == "--graph":
            graph_mode = True
        elif args[i] == "--format" and i + 1 < len(args):
            i += 1
            graph_format = args[i]
        elif args[i] == "--output" and i + 1 < len(args):
            i += 1
            graph_output = args[i]
        elif args[i] == "--detail" and i + 1 < len(args):
            i += 1
            graph_detail = args[i]
        elif args[i] == "--engine" and i + 1 < len(args):
            i += 1
            graph_engine = args[i]
        elif args[i] == "--filter-tier" and i + 1 < len(args):
            i += 1
            filter_tier = args[i]
        elif args[i] == "--filter-phase" and i + 1 < len(args):
            i += 1
            try:
                filter_phase = int(args[i])
            except ValueError:
                print(f"Invalid phase number: {args[i]}")
                sys.exit(1)
        elif args[i] == "--mermaid":
            graph_mode = True
            graph_format = "mermaid"
        elif args[i] == "--check":
            graph_check = True
            graph_mode = True
        elif args[i] == "--strict":
            strict_mode = True
        else:
            file_args.append(args[i])
        i += 1

    files: tuple[Path, ...] | None = None
    if file_args:
        files = tuple(Path(a) for a in file_args)

    if graph_mode:
        root = repo_root()
        if files is None:
            files = find_agents(root)
        all_agents: dict[str, dict[str, Any]] = {}
        for path in files:
            data = load_yaml(path)
            agent = data.get("agent", {})
            if isinstance(agent, dict) and "name" in agent:
                all_agents[str(agent["name"])] = data

        # Apply filters
        if filter_tier:
            all_agents = {
                name: data for name, data in all_agents.items()
                if str(data.get("agent", {}).get("tier", "")) == filter_tier
            }
        if filter_phase is not None:
            all_agents = {
                name: data for name, data in all_agents.items()
                if data.get("agent", {}).get("phase") == filter_phase
            }

        if not all_agents:
            print("No agents match the filter criteria.")
            sys.exit(1)

        # Read version for versioned output
        version_file = root / "VERSION"
        version = version_file.read_text(encoding="utf-8").strip() if version_file.exists() else "dev"

        # --check mode: compare generated markdown against existing file
        if graph_check:
            output_dir = Path(graph_output) if graph_output else root / "docs"
            if output_dir.is_file():
                md_path = output_dir
            else:
                md_path = output_dir / f"agent-communication-{version}.md"

            expected = generate_versioned_markdown(all_agents, version)
            if md_path.exists():
                existing = md_path.read_text(encoding="utf-8")
                if existing == expected:
                    print(f"Up to date: {md_path}")
                    sys.exit(0)
                else:
                    print(f"Stale: {md_path}")
                    print("  Run: uv run agov-validate-agents --graph --output docs/ to regenerate")
                    sys.exit(1)
            else:
                print(f"Missing: {md_path}")
                print("  Run: uv run agov-validate-agents --graph --output docs/ to generate")
                sys.exit(1)

        # Versioned markdown output (when --output points to a directory)
        if graph_output and Path(graph_output).is_dir():
            output_dir = Path(graph_output)
            md_path = output_dir / f"agent-communication-{version}.md"
            content = generate_versioned_markdown(all_agents, version)
            md_path.write_text(content, encoding="utf-8")
            print(f"Versioned graph written to: {md_path}")

            # Also generate SVG/PNG if graphviz available
            from agents_govern.graph import generate_graph, has_dot_binary
            graph_stem = output_dir / f"agent-communication-{version}"
            svg_result = generate_graph(
                all_agents, graph_stem, fmt="svg", detail=graph_detail,
                engine=graph_engine, source_path="agents/",
            )
            print(f"SVG written to: {svg_result}")
            if has_dot_binary():
                png_result = generate_graph(
                    all_agents, graph_stem, fmt="png", detail=graph_detail,
                    engine=graph_engine, source_path="agents/",
                )
                print(f"PNG written to: {png_result}")
            return

        if graph_format == "mermaid":
            print(_generate_comm_graph(all_agents))
        else:
            from agents_govern.graph import generate_graph

            if graph_output is None:
                graph_output = f"agent-communication.{graph_format}"
            source = file_args[0] if file_args else "agents/"
            result = generate_graph(
                all_agents, Path(graph_output),
                fmt=graph_format, detail=graph_detail,
                engine=graph_engine,
                source_path=source,
            )
            print(f"Graph written to: {result}")
        return

    sys.exit(validate_agents(files, strict=strict_mode))


if __name__ == "__main__":
    main()
