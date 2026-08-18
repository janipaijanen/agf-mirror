# Commercial licensing

*Status: Proposed draft — not released. Current framework version: see [Release
information](../../reference/release-info.md). Audience: administrators,
procurement, and legal reviewers evaluating commercial redistribution of
AGF framework content.*

## Overview

AGF is dual-licensed. Framework content — governance rules, agent
definitions, schemas, templates, skills, and related documentation — is
free under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/legalcode)
for any use, including internal commercial use, subject to attribution
and ShareAlike on distributed adaptations. Tooling remains MIT and is
never affected by anything on this page. See
[Licensing](licensing.md) for the full split.

This page covers the alternative: a commercial licence
(`LicenseRef-AgentsGovern-Commercial`) for organisations that want to
redistribute AGF framework content — original or modified — as part of
commercial products or client deliverables, without the ShareAlike
condition. If you can comply with CC BY-SA 4.0 (including keeping any
redistributed adaptation under CC BY-SA 4.0), you do not need this
licence.

## Fixed commercial corpus

The commercial grant does not automatically cover every AGF file carrying a
dual-licence header. Corpus version 1 is a closed, enumerated set of 142 files
recorded in
[`LICENSES/commercial-corpus-manifest-v1.yaml`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/LICENSES/commercial-corpus-manifest-v1.yaml).
The manifest also records 50 explicit exclusions: 49 inventory-derived paths
pending qualified-counsel clearance and the commercial licence text itself,
which is the licensing instrument rather than Framework Content.

Each included path is bound to its SHA-256 at the manifest's recorded Git
revision. New matching files do not enter the commercial corpus implicitly;
each needs an explicit included or excluded disposition and a new governed
manifest version. The current-tree check is intentionally advisory, while
manifest schema defects are blocking:

```sh
agov commercial-corpus validate
agov commercial-corpus verify-version \
  --manifest LICENSES/commercial-corpus-manifest-v1.yaml
```

Executed agreements cite the exact Git blobs for both the licence text and
the selected manifest, plus the manifest's corpus-set digest. Qualified-
counsel review remains mandatory before any real offer or agreement.

## Commercialisation begins 2027

**Commercial licence agreements are available from 1 January 2027.**
Tier pricing applies immediately from your signing date — there is no
free introductory period and no benefit to signing earlier than you
otherwise would. Until 1 January 2027, this licence isn't being offered;
CC BY-SA 4.0 is the only option for anyone who needs to redistribute AGF
framework content before then.

## Pricing from 2027

*(introductory pricing while we learn market demand)*

| Tier | Organisation size | Annual fee |
|------|-------------------|------------|
| **Starter** | Revenue < €2 million **and** < 10 employees | **EUR 100** |
| **Standard** | Revenue < €10 million **and** < 400 employees | **EUR 500** |
| **Enterprise** | Larger organisations or special requirements | Contact us |

- One fee covers the whole organisation for one year.
- No per-developer, per-product, or per-end-user counting for Starter and Standard.
- Pricing and tier definitions may change over time; an agreement's
  terms are fixed for its own stated term once signed.

## What the commercial licence gives you

- Right to redistribute AGF framework content (original or modified)
  under terms of your choosing, including proprietary terms.
- No ShareAlike obligation on those redistributions.
- Attribution is satisfied by retaining the original copyright notice
  (a stronger requirement can be agreed if desired).
- Valid for new distributions during the agreed term. Copies already
  supplied to end users may continue to be used by them after the term
  ends, even if you don't renew.

## What it does not cover

- AGF tooling code (CLI, validators, etc.) — always MIT, no commercial
  licence needed or available for it.
- Pure internal use with no redistribution to third parties — always
  free under CC BY-SA 4.0, with or without this licence.

## Eligibility — when this licence is appropriate

- You embed AGF rules, agent definitions, schemas, or templates in commercial products.
- You distribute AGF-derived content to clients or customers.
- You include AGF content in deliverables provided to third parties.
- You want to modify and redistribute AGF content without ShareAlike obligations.

## How to request an agreement

Email [jani.paijanen@protonmail.com](mailto:jani.paijanen@protonmail.com) with:

- Organisation name.
- Approximate tier (Starter / Standard / Enterprise).
- Intended use (short description).

Payment method and invoicing details are arranged as part of setting up
your specific agreement, not published here. A sample of the commercial
agreement's structure is at
[LICENSES/agf-commercial-agreement-template.md](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/LICENSES/agf-commercial-agreement-template.md)
in the repository.

The licence becomes effective only once a written agreement is signed
and payment has been received in full.

## Custom / enterprise terms

Available on request for:

- Multi-year prepayment.
- OEM / unlimited redistribution arrangements.
- Priority support or roadmap input.
- Custom attribution or branding requirements.

## FAQ

**Does this cover the tooling?**
No. Tooling is always MIT, whether or not you hold a commercial licence.

**Can I still use AGF for free without any of this?**
Yes, under CC BY-SA 4.0, if you follow its attribution and ShareAlike
conditions. Most adopters don't need this licence at all — see
[Licensing](licensing.md).

**Can I sign now, in 2026?**
No. Commercial licence agreements are available from 1 January 2027.
There's no early-signing benefit to wait for or rush toward — pricing is
the same whenever you sign from that date.

**What happens if I don't renew?**
End users who already received redistributed copies may keep using
them. New distributions require an active agreement (or compliance with
CC BY-SA 4.0 instead).

**Do I need a licence per employee or per product?**
No. Starter and Standard are a single annual fee per organisation.

**Is this pricing permanent?**
No. It's introductory while demand is assessed. An individual signed
agreement's terms don't change once signed; future signees may see
different pricing.

## See also

- [Licensing](licensing.md) — the CC BY-SA 4.0 / MIT split and general guidance.
- Root `LICENCE` file and `LICENSES/LicenseRef-AgentsGovern-Commercial.txt` — the authoritative licence text this page summarises.

---

*Last updated: 2026-08-18.*
