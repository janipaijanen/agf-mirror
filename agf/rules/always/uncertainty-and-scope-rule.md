---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Uncertainty and Scope
#
# Framework principle: Boundary Enforcement, Escalation Routing
# See: framework.md — Principle 5 (Escalation over Improvisation)

- If the task requires modifying more than two modules: write a plan file first. The plan must name every module that will change and why.
- If you are uncertain whether a behaviour is intentional (e.g. a method that seems to do nothing, a config flag you don't understand): read the tests for that module and the CHANGELOG before assuming it is a bug.
- If a rule conflicts with what the code already does: point out the conflict explicitly rather than silently resolving it in either direction.
- If asked to do something that a rule prohibits: refuse, state the rule name, and propose a compliant alternative.
