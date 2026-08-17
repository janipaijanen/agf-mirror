Sorting the register by whether the risk attaches to *this* project or to the *class* of AI-governance frameworks. The distinction matters for remediation: the first group is fixable by you, the second can only be managed and disclosed.

## Project-specific

| ID | Risk | Why it's yours specifically |
| --- | --- | --- |
| R01 | Bus factor 1 | A solo-maintainer risk, not an AI-governance risk. Any nine-week-old single-author project has it. |
| R02 | No disclosure channel | Early-stage OSS hygiene gap; nothing about governance frameworks makes it harder. |
| R03 | No signing / SBOM | Same. You've already built more provenance machinery than most projects that *do* sign. |
| R04 | Release publication failures | Specific to this project's release-construction complexity — four failures traced to runner disk, asset-contract drift, and frozen construction lines. |
| R10 | CI runner concentration | Resourcing, not category. |
| R12 | Docs/release version drift | Your build-time generation choice; a machine-readable published-release index closes it. |
| R13 | Engineering quality unassessed | An artifact of this review's access limits, not a project property. |

## Category-wide, instantiated here

| ID | Risk | The general form |
| --- | --- | --- |
| R06 | Evidence base is self-referential; zero overkill findings | **Process frameworks have no counterfactual.** You cannot run the same change through the same team twice. Every governance framework — SAFe, ISO 42001 implementations, NIST AI RMF profiles — is validated by testimony from people already committed to it. AGF's Provisional/Recommended labelling is an unusually honest response to this, but it doesn't dissolve the problem. |
| R08 | Append-only trail vs. erasure rights; labour-surveillance exposure | **Audit value and privacy exposure are the same artifact.** Anything that makes AI-assisted work reconstructible also makes individual contribution patterns correlatable and retention-bound. Every evidence-generating governance system inherits this; runtime-monitoring platforms have it worse. |
| R09 | Review velocity vs. governor capacity | **The central structural problem of the category.** Machine-speed production against human-speed judgment. Gates degrade into ceremony under load regardless of how they're specified. Runtime enforcement doesn't escape it either — it relocates the bottleneck to policy authoring. |
| R11 | Displacement by assistant-native governance | **Third-party governance tooling sits on vendor roadmaps.** Rules files, hooks, and harness-level transition events are all candidates for native implementation. Applies to every non-vendor governance layer, not just file-based ones. |
| R05 | Content licence applied to configuration-as-code | **Governance frameworks ship prose that behaves like software.** Rules, prompts, and policy text are copied, modified, and embedded; content licences weren't designed for that, and software licences read oddly on prose. The category has no settled answer. Your choice of CC BY-SA is a specific instantiation; the ambiguity is general. |
| R07 | Fail-open tooling returning `ok: true` | Your `dir` defect is specific. The class is not: **assurance tooling that reports success on unvalidated input is worse than no tooling**, because it converts an unchecked state into recorded evidence of a check. This failure mode recurs across scanners, policy engines, and compliance automation generally. |

## Category risks not in the register

Worth naming because they bind at enterprise scale rather than at your current scale:

- **Self-reported evidence.** The agent that performs the work also authors the coordination record documenting it. The trail is reconstructible, but its *contents* are the assistant's account of its own behaviour, not an independent observation. Structural in any governance-by-record design where the governed party writes the record. Your role separation and human gates constrain this; they don't eliminate it.
- **Instruction files as an injection surface.** Rules loaded by an assistant are, mechanically, prompts. Anyone who can commit to the repository can influence assistant behaviour through them. Generic to every file-based instruction system.
- **Advisory vs. structural enforcement.** Rules that an assistant "loads as instructions" are followed at the model's discretion. Your docs are explicit that some checks rely on reviewer discipline rather than being structurally blocked — which is correct and rare. The gap itself is category-wide.
- **Governance theatre / assurance laundering.** Records accumulate and get treated downstream as evidence of compliance they don't establish. Your disclaimer and Art. 50 framing address this directly; the pressure is external and won't go away.
- **Rule drift against model behaviour.** Rules tuned to one generation of assistants may become redundant or actively counterproductive as models change. No governance framework has a good answer to this yet.

## Practical read

The project-specific risks are the ones blocking the current rating, and most are bounded work. The category risks are the ones that determine whether the *approach* holds over three years — and they're where an honest positioning statement is more valuable than a mitigation. R09 and the self-reported-evidence problem are the two I'd expect a sceptical enterprise reviewer to push hardest on, and neither currently has a documented position.