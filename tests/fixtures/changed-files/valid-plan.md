---
id: changed-files-fixture-valid-plan
status: pending-gate1
topic: changed-files-fixture-valid-plan
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Test fixture only. Never proposed for Gate 1 and never implemented; it
      exists so tests/changed-files-failclosed.sh can put a VALID plan in a
      comparison base and then break it on the candidate branch, producing a
      genuine M status rather than an A.
  - role: contribution-auditor
    status: waived
    reason: >-
      Test fixture only. See above.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Fixture: a structurally valid plan

Lives in the comparison base so the MODIFY assertions can break it on the
candidate branch. Review 13rh (GCD-G2-1) showed that adding a file and then
editing it within the same range is still an ADD over that range, so the
base must already contain the valid file for a modification to be a
modification.
