# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Agent communication graph generation using Graphviz.

Generates visual diagrams of agent communication networks with
asymmetry highlighting. Supports SVG, PNG, and PDF output.

Detail levels:
- minimal:  agent names + communication edges (clean overview)
- default:  + tier grouping, version, phase, maturity borders
- detailed: + message types on edges, escalation paths, legend

Graceful degradation for airgapped deployments:
- If the `dot` binary is available: renders to the requested format
- If `dot` is missing: saves the .dot source file and warns
"""

from __future__ import annotations

import shutil
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import graphviz

from agents_govern._files import repo_root
from agents_govern.manifest import AgentStatus, load_manifest
from agents_govern.validate_agents import (
    _check_communication_symmetry,
    _check_ownership_overlaps,
    _extract_comm_agents,
    _extract_coordination_agents,
)

__all__ = ["generate_graph", "has_dot_binary", "DETAIL_LEVELS", "ENGINES"]

DETAIL_LEVELS: tuple[str, ...] = ("minimal", "default", "detailed")
ENGINES: tuple[str, ...] = ("dot", "fdp", "sfdp", "neato", "circo", "twopi")

# Tier → visual style mapping
_TIER_STYLES: dict[str, dict[str, str]] = {
    "production": {"shape": "box", "style": "filled", "fillcolor": "#d4e6f1", "color": "#2980b9"},
    "governance": {"shape": "doubleoctagon", "style": "filled", "fillcolor": "#f9e79f", "color": "#f39c12"},
    "cross-cutting": {"shape": "ellipse", "style": "filled", "fillcolor": "#d5f5e3", "color": "#27ae60"},
    "meta": {"shape": "hexagon", "style": "filled", "fillcolor": "#e8daef", "color": "#8e44ad"},
    "publication-support": {"shape": "oval", "style": "filled", "fillcolor": "#fadbd8", "color": "#e74c3c"},
    "scenario-content": {"shape": "component", "style": "filled", "fillcolor": "#d6eaf8", "color": "#2e86c1"},
    "framework": {"shape": "house", "style": "filled", "fillcolor": "#fdebd0", "color": "#ca6f1e"},
}

_TIER_LABELS: dict[str, str] = {
    "production": "Production",
    "governance": "Governance",
    "cross-cutting": "Cross-Cutting",
    "meta": "Meta",
    "publication-support": "Publication Support",
    "scenario-content": "Scenario Content",
    "framework": "Framework",
}

_DEFAULT_STYLE: dict[str, str] = {"shape": "box", "style": "filled", "fillcolor": "#f2f3f4", "color": "#95a5a6"}


def has_dot_binary() -> bool:
    """Check whether the graphviz `dot` binary is available on PATH."""
    return shutil.which("dot") is not None


def _extract_message_types(
    comm: dict[str, Any], direction: str, target_agent: str,
) -> list[str]:
    """Extract message types for a specific agent from a communication section."""
    entries = comm.get(direction)
    if not isinstance(entries, list):
        return []
    for entry in entries:
        if isinstance(entry, dict) and str(entry.get("agent", "")).strip() == target_agent:
            types = entry.get("message_types", [])
            return [str(t) for t in types] if isinstance(types, list) else []
    return []


def _extract_escalation_targets(data: dict[str, Any]) -> list[tuple[str, str]]:
    """Extract escalation route_to targets from an agent's boundaries.

    Returns list of (target_agent_or_role, reason) tuples.
    """
    boundaries = data.get("boundaries", {})
    if not isinstance(boundaries, dict):
        return []
    triggers = boundaries.get("escalation_triggers", [])
    if not isinstance(triggers, list):
        return []
    results: list[tuple[str, str]] = []
    for trigger in triggers:
        if isinstance(trigger, dict):
            route_to = str(trigger.get("route_to", "")).strip()
            reason = str(trigger.get("reason", "")).strip()
            if route_to:
                results.append((route_to, reason))
    return results


def _get_version() -> str:
    """Read the framework version from VERSION file."""
    try:
        return (repo_root() / "VERSION").read_text(encoding="utf-8").strip()
    except Exception:
        return "unknown"


def _write_png_metadata(png_path: Path, metadata: dict[str, str]) -> None:
    """Write text metadata into a PNG file's tEXt chunks.

    Uses struct to write PNG chunks directly — no PIL/Pillow dependency.
    Sanitizes values: null bytes stripped (they corrupt tEXt chunk format),
    non-latin-1 characters replaced.
    """
    import struct
    import zlib

    data = png_path.read_bytes()
    if data[:8] != b'\x89PNG\r\n\x1a\n':
        return  # not a valid PNG

    # Build tEXt chunks to insert before IEND
    chunks = b""
    for key, value in metadata.items():
        # Sanitize: strip null bytes (tEXt uses null as key/value separator)
        safe_key = key.replace("\x00", "")
        safe_value = value.replace("\x00", "")
        payload = safe_key.encode("latin-1", errors="replace") + b"\x00" + safe_value.encode("latin-1", errors="replace")
        crc = zlib.crc32(b"tEXt" + payload) & 0xFFFFFFFF
        chunks += struct.pack(">I", len(payload)) + b"tEXt" + payload + struct.pack(">I", crc)

    # Insert before IEND (last 12 bytes)
    png_path.write_bytes(data[:-12] + chunks + data[-12:])


def generate_graph(
    all_agents: dict[str, dict[str, Any]],
    output_path: Path,
    fmt: str = "svg",
    title: str = "Agent Communication",
    detail: str = "default",
    engine: str = "dot",
    source_path: str = "",
) -> Path:
    """Generate an agent communication graph.

    Args:
        all_agents: dict of agent_name → agent YAML data (full parsed dict).
        output_path: path for the output file (without extension).
        fmt: output format — "svg", "png", or "pdf".
        title: graph title.
        detail: detail level — "minimal", "default", or "detailed".
        engine: graphviz layout engine — "dot", "fdp", "sfdp", "neato", "circo", "twopi".
        source_path: path to agents directory (for provenance footer).

    Returns:
        Path to the generated file. If dot binary is missing, returns .dot source path.
    """
    if detail not in DETAIL_LEVELS:
        detail = "default"
    if engine not in ENGINES:
        engine = "dot"

    show_tiers = detail in ("default", "detailed")
    show_version = detail in ("default", "detailed")
    show_maturity_border = detail in ("default", "detailed")
    show_message_types = detail == "detailed"
    show_escalations = detail == "detailed"
    show_legend = detail in ("default", "detailed")

    # Provenance
    version = _get_version()
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    source_label = source_path or "agents/"
    provenance = f"Generated with agents-govern v{version} · {timestamp} · source: {source_label} · {len(all_agents)} agents"

    dot = graphviz.Digraph(
        name="agent_communication",
        comment=title,
        format=fmt,
        engine=engine,
    )
    dot.attr(
        rankdir="LR",
        label=f"{title}\n\n{provenance}",
        labelloc="t",
        fontsize="16",
        fontname="Helvetica",
        bgcolor="white",
        pad="0.5",
    )
    dot.attr("node", fontname="Helvetica", fontsize="11")
    dot.attr("edge", fontname="Helvetica", fontsize="9")

    # Collect all declared edges
    declared_sends: set[tuple[str, str]] = set()
    declared_receives: set[tuple[str, str]] = set()

    for agent_name, data in all_agents.items():
        comm = data.get("communication", {})
        if not isinstance(comm, dict):
            continue
        for target in _extract_comm_agents(comm, "sends_to", include_legacy=False):
            if target in all_agents:
                declared_sends.add((agent_name, target))
        for source in _extract_comm_agents(comm, "receives_from", include_legacy=False):
            if source in all_agents:
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

    symmetric = declared_sends & declared_receives
    only_sends = declared_sends - declared_receives
    only_receives = declared_receives - declared_sends

    # Coordination edges not already covered by formal channels
    coord_only = coordination_edges - declared_sends - declared_receives

    # Group agents by tier
    tier_agents: dict[str, list[str]] = {}
    for agent_name, data in all_agents.items():
        agent_sec = data.get("agent", {})
        tier = str(agent_sec.get("tier", "unknown")) if isinstance(agent_sec, dict) else "unknown"
        tier_agents.setdefault(tier, []).append(agent_name)

    # Load manifest if available
    manifest: dict[str, AgentStatus] = {}
    manifest_candidates = [Path("agents/MANIFEST.yaml")]
    if source_label:
        src = Path(source_label)
        manifest_candidates.insert(0, src / "MANIFEST.yaml")
        if src.is_file():
            manifest_candidates.insert(0, src.parent / "MANIFEST.yaml")
    for candidate in manifest_candidates:
        if candidate.exists():
            manifest = load_manifest(candidate)
            break

    # Build nodes
    def _add_agent_node(graph: graphviz.Digraph, agent_name: str) -> None:
        agent_sec = all_agents[agent_name].get("agent", {})
        tier = str(agent_sec.get("tier", "unknown")) if isinstance(agent_sec, dict) else "unknown"
        base_style = dict(_TIER_STYLES.get(tier, _DEFAULT_STYLE)) if show_tiers else dict(_DEFAULT_STYLE)

        # Build label
        parts = [agent_name]
        if show_version:
            phase = agent_sec.get("phase", "?") if isinstance(agent_sec, dict) else "?"
            version = str(agent_sec.get("version", "?")) if isinstance(agent_sec, dict) else "?"
            parts.append(f"v{version} · Phase {phase}")

        # Apply manifest status styling
        agent_manifest = manifest.get(agent_name)
        if agent_manifest:
            if agent_manifest.is_suspended:
                base_style["style"] = "filled,dashed"
                base_style["fillcolor"] = "#f2f3f4"  # muted
                parts.append(f"[SUSPENDED: {agent_manifest.reason}]" if agent_manifest.reason else "[SUSPENDED]")
            elif agent_manifest.is_skipped:
                base_style["style"] = "filled,dotted"
                base_style["fillcolor"] = "#f2f3f4"  # muted
                parts.append(f"[SKIPPED: {agent_manifest.reason}]" if agent_manifest.reason else "[SKIPPED]")

        label = "\n".join(parts)

        # Maturity border
        node_kw: dict[str, str] = {}
        if show_maturity_border:
            version = str(agent_sec.get("version", "0.1.0")) if isinstance(agent_sec, dict) else "0.1.0"
            node_kw["penwidth"] = "2.5" if version > "0.1" else "1.0"

        graph.node(agent_name, label=label, **node_kw, **base_style)

    if show_tiers:
        for tier, agents in sorted(tier_agents.items()):
            style = _TIER_STYLES.get(tier, _DEFAULT_STYLE)
            with dot.subgraph(name=f"cluster_{tier}") as sub:
                sub.attr(
                    label=_TIER_LABELS.get(tier, tier.title()),
                    style="solid",
                    color=style.get("color", "#95a5a6"),
                    fontname="Helvetica",
                    fontsize="12",
                    penwidth="1.5",
                )
                for agent_name in sorted(agents):
                    _add_agent_node(sub, agent_name)
    else:
        for agent_name in sorted(all_agents.keys()):
            _add_agent_node(dot, agent_name)

    # Communication edges
    for src, tgt in sorted(symmetric):
        edge_kw: dict[str, str] = {"color": "#2c3e50", "penwidth": "1.5"}
        if show_message_types:
            comm = all_agents[src].get("communication", {})
            if isinstance(comm, dict):
                types = _extract_message_types(comm, "sends_to", tgt)
                if types:
                    edge_kw["label"] = "  " + "\n".join(types[:3])
                    if len(types) > 3:
                        edge_kw["label"] += f"\n+{len(types) - 3} more"
                    edge_kw["fontsize"] = "7"
                    edge_kw["fontcolor"] = "#7f8c8d"
        dot.edge(src, tgt, **edge_kw)

    # Asymmetric edges (always shown with labels regardless of detail)
    for src, tgt in sorted(only_sends):
        dot.edge(
            src, tgt,
            color="#e74c3c", style="dashed", penwidth="1.5",
            label="sends\n(not received)", fontcolor="#e74c3c",
        )
    for src, tgt in sorted(only_receives):
        dot.edge(
            src, tgt,
            color="#e67e22", style="dashed", penwidth="1.5",
            label="received\n(not sent)", fontcolor="#e67e22",
        )

    # Escalation paths (detailed only)
    if show_escalations:
        for agent_name, data in all_agents.items():
            for route_to, reason in _extract_escalation_targets(data):
                # Normalize route_to to an agent name if possible
                route_lower = route_to.lower()
                target_agent = None
                for candidate in all_agents:
                    if candidate in route_lower:
                        target_agent = candidate
                        break
                if target_agent and target_agent != agent_name:
                    dot.edge(
                        agent_name, target_agent,
                        color="#e67e22", style="dotted", penwidth="1.0",
                        arrowhead="diamond",
                        label="  escalation",
                        fontsize="7", fontcolor="#e67e22",
                    )

    # Coordination/consultation edges (always shown — distinct from formal channels)
    for src, tgt in sorted(coord_only):
        dot.edge(
            src, tgt,
            color="#3498db", style="dotted", penwidth="1.5",
            label="  coordinates/consults",
            fontsize="7", fontcolor="#3498db",
        )

    # Legend
    if show_legend:
        with dot.subgraph(name="cluster_legend") as legend:
            legend.attr(
                label="Legend",
                style="rounded",
                color="#bdc3c7",
                fontsize="10",
                labeljust="l",
            )

            # Tier shape examples
            for tier_name, style in _TIER_STYLES.items():
                label = _TIER_LABELS.get(tier_name, tier_name.title())
                legend.node(f"leg_tier_{tier_name}", label, fontsize="9", **style)

            tier_keys = list(_TIER_STYLES.keys())
            for j in range(len(tier_keys) - 1):
                legend.edge(
                    f"leg_tier_{tier_keys[j]}",
                    f"leg_tier_{tier_keys[j + 1]}",
                    style="invis",
                )

            # Edge style examples
            legend.node("leg_a", "", shape="point", width="0.01")
            legend.node("leg_b", "", shape="point", width="0.01")
            legend.edge(
                "leg_a", "leg_b",
                label="  Symmetric (both sides declared)",
                color="#2c3e50", penwidth="1.5", fontsize="9",
            )
            legend.node("leg_c", "", shape="point", width="0.01")
            legend.node("leg_d", "", shape="point", width="0.01")
            legend.edge(
                "leg_c", "leg_d",
                label="  Asymmetric (one side missing)",
                color="#e74c3c", style="dashed", penwidth="1.5",
                fontsize="9", fontcolor="#e74c3c",
            )

            legend.node("leg_coord1", "", shape="point", width="0.01")
            legend.node("leg_coord2", "", shape="point", width="0.01")
            legend.edge(
                "leg_coord1", "leg_coord2",
                label="  Coordination/consultation",
                color="#3498db", style="dotted", penwidth="1.5",
                fontsize="9", fontcolor="#3498db",
            )

            if show_escalations:
                legend.node("leg_e", "", shape="point", width="0.01")
                legend.node("leg_f", "", shape="point", width="0.01")
                legend.edge(
                    "leg_e", "leg_f",
                    label="  Escalation path",
                    color="#e67e22", style="dotted", penwidth="1.0",
                    arrowhead="diamond",
                    fontsize="9", fontcolor="#e67e22",
                )

            if show_maturity_border:
                legend.node(
                    "leg_mature", "Mature (v0.2+)",
                    shape="box", style="filled", fillcolor="#f2f3f4",
                    color="#95a5a6", penwidth="2.5", fontsize="9",
                )
                legend.node(
                    "leg_early", "Early (v0.1.x)",
                    shape="box", style="filled", fillcolor="#f2f3f4",
                    color="#95a5a6", penwidth="1.0", fontsize="9",
                )
                legend.edge("leg_mature", "leg_early", style="invis")

            # Manifest status indicators
            if manifest:
                legend.node(
                    "leg_suspended", "Suspended",
                    shape="box", style="filled,dashed", fillcolor="#f2f3f4",
                    color="#95a5a6", fontsize="9",
                )
                legend.node(
                    "leg_skipped", "Skipped",
                    shape="box", style="filled,dotted", fillcolor="#f2f3f4",
                    color="#95a5a6", fontsize="9",
                )
                legend.edge("leg_suspended", "leg_skipped", style="invis")

    # Render or save .dot source
    output_stem = str(output_path)
    for ext in (".svg", ".png", ".pdf", ".dot"):
        if output_stem.endswith(ext):
            output_stem = output_stem[: -len(ext)]
            break

    if has_dot_binary():
        rendered = dot.render(output_stem, cleanup=True)
        result_path = Path(rendered)

        # Write metadata into PNG files
        if fmt == "png" and result_path.exists():
            _write_png_metadata(result_path, {
                "Software": f"agents-govern v{version}",
                "Creation Time": timestamp,
                "Source": source_label,
                "Title": title,
                "Description": provenance,
                "Comment": f"{len(all_agents)} agents, detail={detail}, engine={engine}",
            })

        return result_path
    else:
        dot_path = Path(f"{output_stem}.dot")
        dot.save(str(dot_path))
        print(f"WARNING: graphviz 'dot' binary not found on PATH.")
        print(f"  Saved DOT source to: {dot_path}")
        print(f"  Install graphviz to render: dot -T{fmt} {dot_path} -o {output_stem}.{fmt}")
        return dot_path
