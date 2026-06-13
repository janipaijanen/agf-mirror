<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Architecture Decision Record (ADR): License change from CC BY-NC-SA 4.0 to CC BY-SA 4.0

## Status

Proposed

## Context

agents-govern is currently licensed under CC BY-NC-SA 4.0, which prohibits commercial use. The first adopter project (iron-gate — a cybersecurity exercise scenario definition language) targets an audience that includes commercial cybersecurity training providers, exercise platform vendors, and enterprise security teams. These users need to use the iron-gate framework — which itself incorporates agents-govern governance patterns — in commercial operations.

The NonCommercial restriction creates a licensing incompatibility: iron-gate cannot be adopted by commercial users if the upstream governance framework prohibits commercial use. Since agents-govern is a governance framework (not a product), its value increases with adoption breadth. Restricting commercial use limits that adoption without providing meaningful protection.

## Decision

Change the license from CC BY-NC-SA 4.0 to CC BY-SA 4.0.

- **Attribution (BY):** Retained. Users must credit the original author.
- **ShareAlike (SA):** Retained. Derivative works must use the same or a compatible license.
- **NonCommercial (NC):** Removed. Commercial use is now permitted.

All Software Package Data Exchange (SPDX) headers across the repository were updated from `CC-BY-NC-SA-4.0` to `CC-BY-SA-4.0`.

## Alternatives considered

**A. CC BY-SA 4.0 (chosen).**
- Pro: Allows commercial adoption while preserving attribution and sharealike. ShareAlike ensures that improvements to the framework remain open. Well-understood license with strong legal backing from Creative Commons.
- Con: No revenue protection if someone commercialises the framework itself (mitigated by sharealike — they must share their modifications).

**B. Apache 2.0.**
- Pro: Very permissive, includes patent grant, widely understood in software.
- Con: No sharealike — derivative works could be made proprietary. Better suited for software libraries than governance frameworks.

**C. Massachusetts Institute of Technology (MIT).**
- Pro: Maximally permissive, simplest possible license.
- Con: No sharealike, no patent grant. Derivative governance frameworks could be closed-source.

**D. Keep CC BY-NC-SA 4.0.**
- Pro: Maintains control over commercial use.
- Con: Blocks the primary adoption path (commercial cybersecurity exercise platforms). A governance framework with no commercial adopters has limited real-world validation.

## Counter-arguments sought

- **iron-gate project (adopter):** Confirmed that commercial cybersecurity training providers are a primary target audience. The NC restriction would prevent adoption by organisations like the SysAdmin, Audit, Network, and Security (SANS) Institute, Offensive Security, or enterprise security teams running internal exercises.
- **Open-source licensing perspective:** CC BY-SA 4.0 is the license used by Wikipedia and OpenStreetMap — proven at scale for collaborative knowledge works. The sharealike provision is the key protection: anyone who modifies the framework must share their modifications under the same terms.

## Strongest objection

Removing the NonCommercial restriction means a company could take agents-govern, repackage it as part of a paid governance consulting offering, and profit without contributing back beyond the sharealike requirement.

**Rebuttal:** The ShareAlike clause ensures that any modified version of the framework must be released under CC BY-SA 4.0. A company can *use* the framework commercially (which is the goal — real-world adoption), but cannot create a proprietary fork. The attribution requirement ensures the original author is credited. This is the same protection model that Wikipedia and OpenStreetMap rely on successfully.

## Consequences

**Positive:**
- Removes the primary barrier to commercial adoption of agents-govern and its downstream projects (iron-gate).
- Increases the potential adopter pool significantly (enterprise, training providers, consultancies).
- More adopters means more learnings feeding back into the framework.
- Aligns with the framework's purpose: governance knowledge should be widely adopted, not restricted.

**Negative:**
- Loss of theoretical revenue protection from the NC clause (practically, no revenue was being generated or planned under NC).
- Cannot be reversed for existing versions already released under CC BY-SA 4.0 (though new versions could change license — not recommended).

## Confidence and reevaluation triggers

**Confidence: High.** The NC restriction actively harms the framework's adoption goals with no offsetting benefit. CC BY-SA 4.0 is a well-tested license for collaborative works.

**Reevaluation triggers:**
- Evidence that a commercial entity is extracting value without contributing back in ways that harm the framework ecosystem (unlikely given sharealike).
- A future need for dual licensing (e.g., offering a commercial support tier) — would require contributor agreement infrastructure first.
