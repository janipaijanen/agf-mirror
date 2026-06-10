<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
Closes # <!-- Replace with the issue number, or delete this line -->

## Learning Record Submission

**Project codename:** <!-- e.g., blue-heron -->
**Number of learnings:** <!-- How many new entries in this file? -->
**Framework version:** <!-- Which version were the learnings observed against? Check VERSION file or MANIFEST.md -->

## Anonymization checklist

- [ ] Filename is `learnings/<codename>.yaml`
- [ ] `project.codename` field set (not `project.name`)
- [ ] No real project, product, or organisation names in any field
- [ ] No internal URLs, hostnames, or infrastructure identifiers
- [ ] `scripts/validate-learnings.sh` passes on the file
- [ ] `scripts/validate-infosec.sh` passes or all findings have `sentinel_ack`
- [ ] No credentials, connection strings, or secrets in any field
- [ ] Cross-record references to other project codenames reviewed

## Guidance integrity checklist (Contribution Auditor)

- [ ] Learnings do not recommend relaxing existing governance rules or agent boundaries
- [ ] No absence-of-incident reasoning ("we saw no issues after removing X")
- [ ] Evidence is specific (project codename, framework version, concrete outcome)
- [ ] Claimed patterns are technically plausible in the described context
- [ ] Learnings do not introduce precedent that could weaken governance for other adopters

## New learnings summary

<!-- One line per learning: date, category, title -->

-
