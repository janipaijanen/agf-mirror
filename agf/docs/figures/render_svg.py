#!/usr/bin/env python3
"""Render governed DOT figures and add reproducible SVG title/description."""

from __future__ import annotations

import html
import re
import subprocess
import sys
from pathlib import Path


def metadata(source: str, key: str) -> str:
    match = re.search(rf"^// svg-{key}:\s*(.+)$", source, re.MULTILINE)
    if not match:
        raise SystemExit(f"missing // svg-{key}: metadata")
    return match.group(1).strip()


def render(source_path: Path) -> None:
    source_path = source_path.resolve()
    source = source_path.read_text(encoding="utf-8")
    title = metadata(source, "title")
    description = metadata(source, "desc")
    process = subprocess.run(
        ["dot", "-Tsvg", str(source_path)], capture_output=True, text=True
    )
    if process.returncode:
        raise SystemExit(process.stderr.strip())
    rendered = process.stdout
    rendered, count = re.subn(
        r"<title>.*?</title>",
        f"<title>{html.escape(title)}</title>\n"
        f"<desc>{html.escape(description)}</desc>",
        rendered,
        count=1,
    )
    if count != 1:
        raise SystemExit(f"expected one graph title in {source_path}")

    figures_dir = source_path.parent.parent
    destinations = (
        figures_dir / f"{source_path.stem}.svg",
        figures_dir.parent / "site" / "reference" / "figures" / f"{source_path.stem}.svg",
    )
    for destination in destinations:
        destination.write_text(rendered, encoding="utf-8")


if len(sys.argv) < 2:
    raise SystemExit("usage: render_svg.py src/figure.dot [src/figure.dot ...]")
for argument in sys.argv[1:]:
    render(Path(argument))
