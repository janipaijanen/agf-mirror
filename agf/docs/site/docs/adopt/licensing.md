# Licensing

*Status: Released. Current framework version: see [Release
information](../../reference/release-info.md). Audience: administrators,
procurement, and legal reviewers evaluating AGF for adoption.*

AGF uses more than one licence because the repository contains both
framework content and implementation tooling. This page is the canonical
adopter-facing summary; the authoritative text lives in the repository's
root `LICENCE` file.

## The split

| Material | Licence | What it covers |
|---|---|---|
| Framework and governance content — rules, agent definitions, schemas, documentation, examples, plans, proposals, coordination records | [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/legalcode) | Use, modify, and redistribute for any purpose, including commercial. Redistributing an adapted version requires the same licence on the adaptation (ShareAlike) and attribution. Internal use and modification, with no external redistribution of the modified content, carries no ShareAlike obligation. |
| Implementation tooling — the `agov` CLI, validators, build and release tooling | MIT | Use, modify, embed, and redistribute, including in proprietary software. No ShareAlike condition — but the copyright notice and permission notice must be preserved in copies or substantial portions of the software, per the MIT licence text. |

A file-specific `SPDX-License-Identifier` header, where present, is
authoritative for that file. Where no header is present, the path-based
split above applies.

## Commercial licensing

Some framework content is marked with the alternative expression
`CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`. Commercial or
otherwise separate licence terms *may* be available by agreement with the
copyright holder. Those terms are alternatives to the public licence
where expressly granted; they are not implied by this page. If your
organisation needs a redistribution licence without a ShareAlike
condition, contact the copyright holder directly rather than assuming
terms from this page.

## Practical guidance by use case

- **Using AGF internally** (adopting the rules, running the tooling, not
  redistributing the framework itself): both licences already permit
  this, including commercial internal use, with no action needed.
- **Redistributing AGF unmodified** (e.g., as part of your own
  documentation or a public fork): CC BY-SA 4.0 requires attribution;
  MIT requires the copyright notice.
- **Redistributing modified framework content**: the modified content
  must stay under CC BY-SA 4.0 unless you hold a commercial licence for
  it. Modified tooling has no such condition — MIT permits proprietary
  redistribution of modified tooling.
- **Embedding the tooling in a proprietary product**: permitted under
  MIT with the copyright notice retained.

## See also

- Root `LICENCE` file — the authoritative summary.
- [Release information](../../reference/release-info.md) — version,
  source commit, and copyright details for the release you're
  evaluating.
