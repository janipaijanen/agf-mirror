# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""File discovery utilities for the agents-govern framework."""

from __future__ import annotations

from pathlib import Path

__all__ = [
    "find_learnings",
    "find_agents",
    "find_rules",
    "find_skills",
    "repo_root",
]


def repo_root() -> Path:
    """Find the repository root by walking up from this file's location."""
    current = Path(__file__).resolve().parent
    while current != current.parent:
        if (current / "VERSION").exists() and (current / "framework.md").exists():
            return current
        current = current.parent
    # Fallback: assume cwd
    return Path.cwd()


def find_learnings(root: Path | None = None) -> tuple[Path, ...]:
    """Find all learning YAML files, excluding the template."""
    root = root or repo_root()
    learnings_dir = root / "learnings"
    if not learnings_dir.is_dir():
        return ()
    return tuple(
        sorted(
            p
            for p in learnings_dir.iterdir()
            if p.suffix in (".yaml", ".yml")
            and p.stem != "_template"
        )
    )


def find_agents(root: Path | None = None) -> tuple[Path, ...]:
    """Find all agent definition YAML files."""
    root = root or repo_root()
    agents_dir = root / "agents"
    if not agents_dir.is_dir():
        return ()
    return tuple(
        sorted(
            p
            for p in agents_dir.iterdir()
            if p.suffix in (".yaml", ".yml")
            and p.name != "MANIFEST.yaml"
        )
    )


def find_rules(root: Path | None = None) -> tuple[Path, ...]:
    """Find all rule template markdown files, excluding READMEs."""
    root = root or repo_root()
    rules_dir = root / "rules-template"
    if not rules_dir.is_dir():
        return ()
    return tuple(
        sorted(
            p
            for p in rules_dir.rglob("*.md")
            if p.name != "README.md"
        )
    )


def find_skills(root: Path | None = None) -> tuple[Path, ...]:
    """Find all skill directories (those containing SKILL.md)."""
    root = root or repo_root()
    skills_dir = root / "skills"
    if not skills_dir.is_dir():
        return ()
    return tuple(
        sorted(
            d
            for d in skills_dir.iterdir()
            if d.is_dir() and (d / "SKILL.md").exists()
        )
    )
