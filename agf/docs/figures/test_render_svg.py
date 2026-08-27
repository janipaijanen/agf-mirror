# SPDX-License-Identifier: MIT
"""Regression tests for deterministic SVG rendering paths and metadata."""

from __future__ import annotations

import importlib.util
import os
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


SCRIPT_PATH = Path(__file__).with_name("render_svg.py")


class RenderSvgTest(unittest.TestCase):
    def test_relative_source_writes_identical_accessible_copies(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            figures = root / "agf/docs/figures"
            source_dir = figures / "src"
            site_dir = root / "agf/docs/site/reference/figures"
            source_dir.mkdir(parents=True)
            site_dir.mkdir(parents=True)
            source = source_dir / "example.dot"
            source.write_text(
                "// svg-title: Example title\n"
                "// svg-desc: Example description\n"
                "digraph example { a -> b }\n",
                encoding="utf-8",
            )

            spec = importlib.util.spec_from_file_location("render_svg", SCRIPT_PATH)
            assert spec and spec.loader
            module = importlib.util.module_from_spec(spec)

            completed = subprocess.CompletedProcess(
                args=["dot"],
                returncode=0,
                stdout="<svg><g><title>example</title></g></svg>\n",
                stderr="",
            )
            previous = Path.cwd()
            try:
                os.chdir(figures)
                with patch("subprocess.run", return_value=completed):
                    with patch("sys.argv", [str(SCRIPT_PATH), "src/example.dot"]):
                        spec.loader.exec_module(module)
            finally:
                os.chdir(previous)

            authored = figures / "example.svg"
            published = site_dir / "example.svg"
            self.assertEqual(authored.read_bytes(), published.read_bytes())
            rendered = authored.read_text(encoding="utf-8")
            self.assertIn("<title>Example title</title>", rendered)
            self.assertIn("<desc>Example description</desc>", rendered)


if __name__ == "__main__":
    unittest.main()
