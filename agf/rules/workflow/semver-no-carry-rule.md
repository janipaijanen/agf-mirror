<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# SemVer: No carry on digit overflow
#
# Framework principle: Versioning discipline
# See: framework.md — Appendix: Framework Evolution (Versioning and Identifiers)

Any component of a version number (`MAJOR.MINOR.PATCH`) may grow beyond 99.
**Never** increment the digit to its left when any component reaches 100 or more.

```
# CORRECT -- patch 99 -> 100, minor stays at 3
2.3.99 -> 2.3.100

# CORRECT -- minor 99 -> 100, major stays at 1
1.99.0 -> 1.100.0

# WRONG -- do not carry; 100 is a valid patch number
2.3.99 -> 2.4.0   <- never do this just because patch hit 100
```

**Applies to:** version bumps in project manifests, changelogs, git tags, and any other place a version string appears.
