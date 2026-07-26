---
id: claude-context-cost-audit-archive
status: pending-gate1
topic: claude-context-cost-audit-archive
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >
      Documentation-only preservation of model outputs already supplied by the
      governor-reviewer. The implementation must exclude account identifiers,
      local attachment paths, credentials, and any project name other than
      "foundry"; no executable, schema, hook, or external publication changes.
  - role: contribution-auditor
    status: waived
    reason: >
      The record preserves source material and a clearly labelled advisory
      synthesis without converting model opinions into authority or independent
      evidence. Gate 2 will verify completeness, provenance labels, and the
      separation between observation, documentation, inference, and unknowns.
---

# Plan: permanently archive the Claude context-cost inquiry

## Intent

The governor-reviewer requested permanent repository storage for the context
and cost questions submitted during the 2026-07-26 investigation, the answers
received, and Codex's final cross-assessment. The source material currently
exists only in chat and local attachment storage. This plan creates one durable,
non-operational evidence record so later work can inspect the original claims
without treating session memory as authority.

## Authority and evidence boundary

- Direct governor-reviewer request in the active session on 2026-07-26.
- The record is historical/advisory evidence. AI answers are not independent
  review evidence and do not approve Gate 1, Gate 2, security, or contribution
  decisions.
- Product-behaviour conclusions must retain their evidence labels: observed,
  current primary documentation, inference, or unknown.
- The project is named only as `foundry`. Account identity, email, local user
  names, absolute attachment paths, and other machine-specific identifiers are
  excluded even when they appeared in surrounding session telemetry.

## Affected file

- Add `docs/proposals/claude-context-cost-audit-2026-07-26.md`.

The file is not placed under `.claude/rules/`, `AGENTS.md`, or another
automatically loaded instruction surface. Archival material must not become an
always-loaded prompt cost or a normative rule accidentally.

## Required contents

1. Purpose, date, scope, evidence-status legend, and anonymization statement.
2. The common prompt sent to Perplexity, Mistral, Grok, Gemini, and Lumo.
3. Each submitted first answer and follow-up answer, attributed by model and
   preserved verbatim except for mandatory anonymization. This includes the
   later Codex-specific question and the Lumo, Gemini, Mistral, and Perplexity
   answers; Grok's reported inability to answer is recorded as an absence, not
   reconstructed as model output.
4. Both empirical-audit prompts sent to Claude and both returned answers,
   likewise preserved verbatim except for mandatory anonymization.
5. The live `/context` and `/usage` figures needed to interpret the answers,
   with account identity removed and independent percentages explicitly marked
   as overlapping.
6. Codex's final synthesis, including the corrected deferred-tool arithmetic,
   ranked interventions, instruction taxonomy, controlled experiment, rollback
   thresholds, and remaining unknown measurements.
7. A source manifest containing stable labels, byte counts, and SHA-256 hashes
   for the nine supplied attachment payloads. It must not store absolute local
   attachment paths.
8. A prominent statement that cached or prior AI review is not independent
   evidence and that no recommendation weakens human authority, security,
   auditability, immutable coordination, or Gate discipline.
9. A Codex-specific empirical addendum grounded in the current official Codex
   manual and privacy-minimized local session counters. It must distinguish
   interactive `/status` and `/usage`, local token-count events, optional OTel,
   API-key billing, and ChatGPT subscription limits; it must also record that
   Codex does not natively auto-load Claude's `.claude/rules/` corpus.

## Acceptance checks

- The common and Claude-specific prompts match the submitted text.
- All nine attachment payloads are accounted for by label, byte count, and
  SHA-256 hash.
- Chat-inline follow-up answers are present and attributed correctly.
- The Codex-specific question and all four received answers are present; Grok's
  non-answer is identified without inventing content.
- Codex's synthesis includes the exact active-context reconciliation:
  `9.7 + 20.2 + 0.603 + 58.6 + 2.3 + 287.9 = 379.303k`, proving deferred
  definitions were excluded from the reported 379.3k active total.
- No email address, absolute local attachment path, credential, or non-anonymized
  project name appears.
- Codex counters are reported only as aggregate token/cache/rate-limit figures;
  raw prompts, tool outputs, and session payloads are not copied into the
  archive.
- The file is clearly non-normative and is not linked from an always-loaded
  instruction surface.
- `git diff --check` passes.

## Risks and mitigations

- **Accidental promotion of AI opinion to policy.** Keep raw answers in an
  evidence appendix and label the synthesis advisory.
- **Privacy leakage from copied telemetry.** Use a targeted forbidden-string
  scan before commit and preserve only the measurements needed for analysis.
- **Large archival document becoming prompt overhead.** Store it only under
  `docs/proposals/`; do not import it into assistant memory or rules.
- **Silent editing of submitted answers.** Preserve verbatim payloads and record
  hashes; disclose any anonymization replacement adjacent to the source
  manifest.
- **Volatile product claims aging into false facts.** Record the audit date and
  Claude Code version when available, and link current primary documentation
  rather than presenting the findings as timeless.

## Rollback

The change is one documentation file on a feature branch. Rollback is a normal
revert with no schema, runtime, release, or coordination migration.
