# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""YAML loading and free-text extraction utilities."""

from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml

__all__ = [
    "load_yaml",
    "extract_freetext",
]


def load_yaml(path: Path) -> dict[str, Any]:
    """Load a YAML file and return its contents as a dict.

    Returns an empty dict if the file is missing or cannot be parsed.
    """
    if not path.exists():
        return {}
    try:
        with path.open(encoding="utf-8") as f:
            data = yaml.safe_load(f)
    except yaml.YAMLError:
        return {}
    return data if isinstance(data, dict) else {}


def extract_freetext(data: dict[str, Any]) -> tuple[str, ...]:
    """Extract free-text values from specific YAML fields.

    Mirrors the bash awk-based extraction which only targets:
    description, title, recommendation, notes, customization_notes, conclusion.

    Returns an immutable tuple of all free-text strings found.
    """
    _TARGET_KEYS: frozenset[str] = frozenset({
        "description",
        "title",
        "recommendation",
        "notes",
        "customization_notes",
        "conclusion",
    })
    texts: list[str] = []
    _collect_targeted_strings(data, _TARGET_KEYS, texts)
    return tuple(texts)


def _collect_targeted_strings(
    obj: Any,
    target_keys: frozenset[str],
    out: list[str],
    current_key: str = "",
) -> None:
    """Recursively collect string values only from targeted field names."""
    match obj:
        case dict():
            for k, v in obj.items():
                key_str = str(k)
                if key_str in target_keys:
                    # Collect all string content under this key
                    _collect_all_strings(v, out)
                else:
                    # Recurse to find nested target keys
                    _collect_targeted_strings(v, target_keys, out, current_key=key_str)
        case list():
            for item in obj:
                _collect_targeted_strings(item, target_keys, out, current_key=current_key)
        case _:
            pass


def _collect_all_strings(obj: Any, out: list[str]) -> None:
    """Collect all string values from a YAML subtree."""
    match obj:
        case str():
            stripped = obj.strip()
            if stripped and len(stripped) > 2:
                out.append(stripped)
        case dict():
            for v in obj.values():
                _collect_all_strings(v, out)
        case list():
            for item in obj:
                _collect_all_strings(item, out)
        case _:
            pass
