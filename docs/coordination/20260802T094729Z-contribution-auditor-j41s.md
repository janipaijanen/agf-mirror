---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T094729Z-contribution-auditor-j41s
timestamp: 2026-08-02T09:47:29Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260802T091106Z-governor-reviewer-7pdr
  - 20260802T091308Z-coder-tester-bgkb
topic: ci-rust-cache-policy-slice-a-implementation
requires-ack: false
artifacts_owned: []
---

# Live pipeline review: MR !930 is blocked at exact head f69dcb43

Scope: MR !930 at exact head
`f69dcb438ee902db18131f6f3b4d3e4c52452bdd`, pipeline `2724972771`  
Coverage: INCOMPLETE — the exact-head pipeline is still running, but a
reproduced blocking failure already prevents Gate 2  
Recommendation: BLOCK  
Blocking findings: 1  
Advisory findings: 0  
Open findings with accepted risk: 0  
Authority: recommendation only; the human governor-reviewer decides Gate 2

## Finding CI-CACHE-A-001

Check: resolved `before_script` composition for all 12 shared-cache consumers  
Check status: reviewed  
Outcome: fail  
Requirement strength: mandatory  
Finding: CI-CACHE-A-001  
Impact: material  
Enforcement: blocking  
Basis: approved plan Gate 2 items 6–8 require every shared-cache consumer to
run `assert-current` without losing its existing job behavior, and the normal
exact-head pipeline must pass  
Finding state: open  
Risk disposition: none  
Resolution condition: renew the MR head with correct `before_script`
composition, obtain a passing exact-head pipeline, and verify the resolved
sequence for all three `.rust-tooling-package` consumers plus
`rust:tooling:build:windows:preview`

The renewed implementation added
`!reference [.rust-base, before_script]` to the overriding `before_script`
arrays. That referenced sequence already executes `cd tooling/rust`. The
existing local sequence then executes `cd tooling/rust` again.

The failure is directly reproduced in job `15663970805`
(`rust:tooling:preview`):

```text
$ cd tooling/rust
/usr/bin/bash: line 224: cd: tooling/rust: No such file or directory
ERROR: Job failed: exit code 1
```

This is not isolated to one job. The same `.rust-tooling-package` override is
inherited by `rust:tooling:preview`,
`rust:tooling:preview:windows-package`, and `rust:tooling:release`; the manual
`rust:tooling:build:windows:preview` override repeats the same pattern.

A small safe repair is available: either call only the policy assertion in
each overriding array while preserving its former working-directory sequence,
or retain the base reference while removing/reordering the duplicated
directory change. The coder-tester should choose and test the exact resolved
sequence. This review does not modify MR !930 and makes no Gate 2 decision.
