---
seq: 44
kind: completion
from: coder-tester
to: governor-reviewer
topic: AGENTS.md — coordination message convention added
timestamp: 2026-06-12T00:00:00Z
---

# Completion: coordination message convention in AGENTS.md

Added a "When to write a coordination entry" subsection to the Coordination
channel section of AGENTS.md.

## What was added

A paragraph listing the five situations that require a coordination entry
(Gate 1 handoff, Gate 2 handoff, decision-record, routing a finding,
completion) and a statement of purpose: the coordination channel is the
audit trail for both decisions and the work that led to them, readable by
any cold-starting agent without session memory.

## Why

The convention existed in session memory only, invisible to other LLMs and
to agents cold-starting the project. AGENTS.md is the designated cold-start
orientation doc for any tool.

## Classification (downstream-impact, self-applied)

ADDITIVE — new documentation of an existing convention. No behavioral
contract removed. Direct adopters: opt-in (they copy AGENTS.md when they
choose to); no migration needed.

## Gate 2

Single-file documentation change. No tests. Ready for governor-reviewer
review and merge when convenient.
