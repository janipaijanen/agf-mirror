# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Colored terminal output for validation scripts."""

from __future__ import annotations

import sys

__all__ = [
    "ok",
    "warn",
    "error",
    "block",
    "info",
    "print_summary",
]

# ANSI color codes — disabled when stdout is not a TTY
_RED = "\033[0;31m"
_YELLOW = "\033[0;33m"
_GREEN = "\033[0;32m"
_CYAN = "\033[0;36m"
_NC = "\033[0m"

if not sys.stdout.isatty():
    _RED = _YELLOW = _GREEN = _CYAN = _NC = ""


def ok(msg: str) -> None:
    print(f"{_GREEN}\u2713{_NC}      {msg}")


def warn(msg: str) -> None:
    print(f"{_YELLOW}WARN{_NC}   {msg}")


def error(msg: str) -> None:
    print(f"{_RED}ERROR{_NC}  {msg}")


def block(msg: str) -> None:
    print(f"{_RED}BLOCK{_NC}  {msg}")


def ack(msg: str) -> None:
    print(f"       {_GREEN}ACK{_NC} found \u2014 acknowledged by human")


def info(msg: str) -> None:
    print(f"       {_CYAN}Suggested:{_NC} {msg}")


def fail(msg: str) -> None:
    print(f"{_RED}FAIL{_NC}   {msg}")


def check_ok(msg: str) -> None:
    print(f"{_GREEN}\u2713{_NC} {msg}")


def check_fail(msg: str) -> None:
    print(f"{_RED}\u2717{_NC} {msg}")


def check_warn(msg: str) -> None:
    print(f"{_YELLOW}\u26a0{_NC} {msg}")


def print_summary(
    *,
    passed: bool,
    files_checked: int = 0,
    errors: int = 0,
    warnings: int = 0,
    blocked: int = 0,
    acked: int = 0,
    label: str = "checks",
) -> None:
    """Print a colored summary line."""
    if passed and warnings == 0:
        print(f"\n{_GREEN}All {label} passed.{_NC}")
    elif passed and warnings > 0:
        print(f"\n{_YELLOW}Passed with warnings.{_NC} Review flagged items before merge.")
    else:
        total_fail = errors + blocked
        print(f"\n{_RED}{total_fail} {label} failed.{_NC}")
