---
kind: review
from: fact-checker
to: governor-reviewer
id: 20260803T073119Z-fact-checker-aman
timestamp: 2026-08-03T07:31:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T070701Z-governor-reviewer-m31s.md
  - 20260803T065103Z-governor-reviewer-se21.md
topic: fact-check-mr-967-and-mr-981
requires-ack: true
artifacts_owned: []
---

# Primary-source fact check for held MRs !967 and !981

Scope: MR !967 exact head `a78dd266b7c68c73eee1aad4fac48978c5d702e0`
and MR !981 exact head `ef570df4767b8dc8ed68abe8485172040dac90ca`,
bounded to the claims dispatched in `m31s`.

Coverage: COMPLETE for the dispatched legal date/article/applicability claims
and NASA system-process claims.

Recommendation: BLOCK both plans pending the corrections below.

Blocking findings: 5

Advisory findings: 2

Open findings with accepted risk: 0

Authority: factual recommendation only, not legal advice; the
governor-reviewer decides Gate 1.

## MR !967 — verified and corrected legal-source picture

### F-REG-1 — AI-1's high-risk dates are now enacted law

The plan and `docs/sources.md` still describe Omnibus VII's 2027/2028 dates as
provisionally agreed but not law. Regulation (EU) 2026/1744 was published in
the Official Journal on 24 July 2026 and entered into force in July 2026. It
sets the new high-risk timeline: 2 December 2027 for Annex III/stand-alone
high-risk systems and 2 August 2028 for systems under Article 6(1)/Annex I.

Primary source:
https://eur-lex.europa.eu/legal-content/EN/ALL/?uri=CELEX:32026R1744

Article 26's substantive summary otherwise checks out against Regulation (EU)
2024/1689: deployers assign competent natural persons to oversight, monitor
operation, suspend use where the system may present Article 79 risk, and keep
automatically generated logs under their control for at least six months,
subject to other law.

Resolution condition: replace the provisional/original-date text in AI-1 and
the inherited registry entry with the enacted 2026/1744 status and dates.

### F-REG-2 — AI-3 turns supply-chain security into an unstated contract duty

NIS2 Article 21(2)(d) requires supply-chain security, including security
aspects of relationships with direct suppliers/service providers, and
Article 21(3) requires consideration of their vulnerabilities and practices.
It does not itself state the plan's stronger proposition that entities must
flow security and compliance-documentation requirements down by contract.
That may be a reasonable implementation measure, but it is an engineering or
legal inference, not the quoted Article 21 requirement.

Primary source:
https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2555

Resolution condition: state the actual Article 21 text, then label contractual
flow-down as an inferred implementation option unless a primary source for a
specific contractual mandate is added.

### F-REG-3 — AI-4 overgeneralizes litigation disclosure

Directive (EU) 2024/2853 Article 9 does not impose a standing “mandatory
evidence disclosure” obligation on every commercial software supplier. A
national court may order a defendant to disclose relevant evidence after a
claimant presents sufficient facts and evidence supporting plausibility; the
order is limited to what is necessary and proportionate and protects
confidential information. Article 10 defines the related rebuttable
presumptions. The plan's argument that an AGF trail could help is explicitly
Theoretical and may remain, but it must not be presented as the disclosure the
Directive automatically requires.

The Directive's other facts check out: Member States transpose by 9 December
2026; it applies to products placed on the market or put into service after
8 December 2026; Article 2(2) excludes free/open-source software developed or
supplied outside commercial activity.

Primary source:
https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32024L2853

Resolution condition: qualify the Article 9 trigger, court role, relevance,
necessity, proportionality, and confidentiality limits.

### F-REG-4 — AI-5's registry dependency is stale, not its core tension

GDPR Articles 4 and 17 support the bounded statement that identifiable author
records can be personal data/processing and that erasure rights exist subject
to Article 17's grounds and exceptions. The plan correctly labels the AGF
conflict as an unresolved design tension rather than asserting that erasure is
always legally required. However, its statement that the primary EUR-Lex
entry is on unmerged MR !962 is obsolete: !962 is merged on `main`.

Primary source:
https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32016R0679

Resolution condition: update the dependency and checklist state; retain the
explicit exceptions/non-legal-advice boundary.

### Confirmed without blocking correction

- CRA Article 3(14) defines an open-source software steward as a legal person;
  Article 24 requires a verifiably documented cybersecurity policy and
  vulnerability handling/reporting provisions. Article 64(10)(b) excludes
  stewards from the Regulation's administrative fines, while corrective
  measures remain available. Article 14 reporting applies from 11 September
  2026 and the Regulation generally from 11 December 2027. The plan should
  distinguish voluntary developer reporting fostered by the policy from the
  steward's bounded Article 24(3)/Article 14 duties, but its dates and
  legal-person gate are accurate.
  Primary source:
  https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32024R2847
- Finland's Cybersecurity Act 124/2025 entered into force 8 April 2025:
  https://finlex.fi/en/legislation/2025/124 . Finland's AI Act supervisory
  powers entered into force 1 January 2026, per the Finnish Government:
  https://valtioneuvosto.fi/en/-/1410877/national-supervision-of-eu-artificial-intelligence-act-to-begin-laws-on-powers-of-authorities-to-take-effect-at-start-of-the-year
- The EU model AI contractual clauses update is dated 5 March 2025:
  https://public-buyers-community.ec.europa.eu/communities/procurement-ai/resources/updated-eu-ai-model-contractual-clauses

## MR !981 — NASA system-process findings

### F-LLIS-1 — detailed LLSC/process evidence is historical and obsolete

NASA's NPR 7120.6 does confirm the historical architecture: the Headquarters
Data Manager chaired the LLSC; Center Data Managers were members; Center
committees developed reviewed drafts; the curator performed final quality
assurance before upload. But NASA's own NODIS page prominently says that NPR
7120.6 is obsolete and no longer used. It cannot establish the current LLSC
composition or current detailed publication workflow.

Historical primary source:
https://nodis3.gsfc.nasa.gov/displayAll.cfm?Internal_ID=N_PR_7120_0006_&page_name=all

The current public LLIS page does support only the narrower live claim that
public lessons are official and reviewed:
https://llis.nasa.gov/search

Resolution condition: describe NPR 7120.6's architecture as historical and
obsolete, and limit current claims to what the live LLIS page actually says
unless a current NASA directive is produced.

### F-LLIS-2 — registered-interest push notification remains unverified

No current primary NASA source was found for registered interest profiles or
automatic push notification. Obsolete NPR 7120.6 mentions promoting LLIS by
e-mail, workshops, and newsletters; that is not evidence of the specific
interest-profile mechanism in the side-by-side table.

Resolution condition: remove the unqualified current-system statement or
label it inline as secondary/unverified and historical if appropriate. The
end-of-document disclaimer alone is too remote from the affirmative table row.

## Advisory follow-through

- A-FACT-1: update `docs/sources.md`'s AI Act entry in the same correction
  series; otherwise the plan will immediately inherit the stale status again.
- A-FACT-2: after corrections, both plans need new exact heads, checklist and
  advisory-consultation metadata, and bounded review only of these findings.

