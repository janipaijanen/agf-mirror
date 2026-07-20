#!/usr/bin/env python3
# SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial
# Copyright (c) 2026 Jani Päijänen
"""Render the "AGF in a Nutshell" one-pager from template.html + embedded fonts.

Usage:
    python3 build_flyer.py [--version X.Y.Z] [--date YYYY-MM-DD] [-o OUTPUT.html]

Defaults: --version reads ../../../../VERSION (agf/VERSION) relative to this
script; --date is today's UTC date; -o writes next to this script as
agf-nutshell-flyer.generated.html.

This script only produces the HTML. Rendering that HTML to a PNG is a
separate step (a headless browser screenshot) — see
agf/docs/figures/src/agf-nutshell-flyer/README.md.
"""
import argparse
import datetime
import pathlib

HERE = pathlib.Path(__file__).parent
AGF_VERSION_FILE = HERE / ".." / ".." / ".." / ".." / "VERSION"

FONT_TOKENS = {
    "__FONT_SANS__": "plex-sans",
    "__FONT_MONO400__": "plex-mono-400",
    "__FONT_MONO500__": "plex-mono-500",
    "__FONT_SERIF600__": "plex-serif-600",
    "__FONT_SERIF700__": "plex-serif-700",
}


def build(version: str, date: str) -> str:
    html = (HERE / "template.html").read_text()

    for token, font_name in FONT_TOKENS.items():
        b64 = (HERE / "fonts" / f"{font_name}.b64").read_text().strip()
        if token not in html:
            raise ValueError(f"template.html is missing expected token {token}")
        html = html.replace(token, b64)

    html = html.replace("__AGF_VERSION__", version)
    html = html.replace("__BUILD_DATE__", date)

    remaining = [tok for tok in list(FONT_TOKENS) + ["__AGF_VERSION__", "__BUILD_DATE__"] if tok in html]
    if remaining:
        raise ValueError(f"unreplaced template tokens remain: {remaining}")

    return html


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", default=None, help="AGF version (default: read agf/VERSION)")
    parser.add_argument("--date", default=None, help="Build date, YYYY-MM-DD (default: today, UTC)")
    parser.add_argument(
        "-o", "--output", default=None, help="Output HTML path (default: agf-nutshell-flyer.generated.html next to this script)"
    )
    args = parser.parse_args()

    version = args.version or AGF_VERSION_FILE.resolve().read_text().strip()
    date = args.date or datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    output = pathlib.Path(args.output) if args.output else HERE / "agf-nutshell-flyer.generated.html"

    html = build(version, date)
    output.write_text(html)
    print(f"wrote {output} ({len(html)} bytes) — version={version} date={date}")


if __name__ == "__main__":
    main()
