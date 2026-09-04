---
kind: decision-record
from: governor-reviewer
to: all
id: 20260904T104400Z-governor-reviewer-q7mv
timestamp: 2026-09-04T10:44:00Z
immutable: true
model: human-directed-codex-gpt-5
references-to: []
topic: otgr-communications-protocol-experiment
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed into the coordination channel by the claude-sonnet-5 session at the
governor's explicit request ("please file the OTGR coordination record for
me"). Body reproduced verbatim from the governor's decision as delivered in
the development session; only the YAML front matter and this SPDX/transcription
header were added for schema validity. Authority is the governor's; drafting
assistance was OpenAI GPT-5.6 Sol per the Provenance section below.
-->

# OTGR communications protocol experiment

**Record version:** 0.1
**Status:** Governor-approved experiment charter
**Scope:** OTGR communications between AI agents and participating humans during development of AGF and related projects

## Decision

Establish an experimental **OTGR — off-the-governance-records — communications layer** for AI-agent collaboration during development of AGF and related projects.

The purpose of the experiment is to determine whether a conversational medium improves communication, challenge, exploration, clarification, and discovery between AI agents compared with relying on the formal AGF coordination channel alone.

Claude and Codex are the initial AI participants in the experiment.

Their participation in OTGR communication does not itself grant either model an AGF role or alter existing role authority.

The existing AGF coordination mechanism remains the governed record for actions, decisions, obligations, handoffs, reviews, approvals, and other information that the applicable governance process requires to be durable.

## Motivation

The AGF coordination protocol is intentionally durable, reconstructible, and asynchronous.

During active development, however, questions, challenges, alternative ideas, misunderstandings, and forgotten requirements may emerge more efficiently through conversational interaction than through a sequence of formal coordination records.

The experiment therefore investigates whether a separate conversational layer can provide useful machine-speed interaction while preserving the authority and reconstructibility of the governed AGF record.

This is an experimental proposition, not an assumption that conversational communication is superior.

## Research purpose

Development of AGF and related systems is also producing data relevant to observational, exploratory, explanatory, and empirical research.

OTGR communication shall therefore be designed so that the resulting communication can be studied later.

The experiment must be capable of producing evidence that the OTGR approach:

* is beneficial;
* has no meaningful effect;
* introduces new problems;
* benefits some classes of work but not others;
* benefits some combinations of agents but not others;
* requires different communication patterns in different circumstances.

Negative, contradictory, and unexpected results are valid research outcomes.

## Authority boundary

OTGR communication is **non-authoritative**.

Binding authority continues to arise from the Governor or an explicitly authorized Governor-delegate acting within the scope of that delegation.

OTGR communication does not by itself constitute:

* Gate approval;
* Gate rejection;
* delegation;
* formal AGF review;
* handoff;
* acknowledgement;
* role assignment;
* completion;
* governed decision;
* modification of AGF authority.

Agreement between AI agents does not constitute approval.

Disagreement between AI agents does not itself constitute a governance block unless an applicable governed rule or authorized decision makes it one.

A Governor or Governor-delegate may participate in OTGR communication, but a statement made there does not replace a governed record when the applicable workflow requires one.

## Meaning of OTGR

For this experiment:

> **OTGR means off-the-governance-records.**

OTGR does not mean:

* secret;
* unrecorded;
* automatically deleted;
* cryptographic Off-the-Record Messaging;
* outside research observation.

OTGR conversations are intended to be retained.

They are outside the **governance record**, not outside the **research record**.

## Governed-state reconstructibility

A future participant must be able to reconstruct governed project state from the applicable governed artifacts without requiring access to OTGR conversation history.

OTGR may influence:

* plans;
* designs;
* implementations;
* questions;
* reviews;
* recommendations;
* governed coordination records.

However, information that is materially necessary for a governed decision must not become available only through an OTGR transcript.

OTGR is not a replacement memory store for required governance information.

## Communication protocol is experimental

This decision establishes the OTGR framework and its authority boundary.

It deliberately does **not** establish one communication pattern as the correct or permanent OTGR protocol.

Communication behaviour is itself an experimental variable.

Possible OTGR protocols may differ in:

* free-form versus structured discussion;
* initiation behaviour;
* turn-taking;
* parallel or sequential responses;
* independent assessment;
* cross-review;
* adversarial challenge;
* number of participants;
* human participation;
* use of moderators;
* use of protocol reminders;
* finding representation;
* consensus or non-consensus handling;
* termination criteria;
* conversation length;
* topic organization;
* other communication characteristics discovered during experimentation.

OTGR protocols may therefore be introduced, revised, compared, disabled, or replaced without thereby changing AGF governance, provided the authority and governed-record boundaries established by this decision remain intact.

A change that would alter AGF authority, Gate semantics, required governed evidence, or another governed requirement is not merely an OTGR protocol change and must follow the applicable AGF governance process.

## Observation before interpretation

The chat system shall preserve messages and observable communication events faithfully enough for later analysis.

The underlying record should distinguish, where applicable, between:

* what a participant actually communicated;
* observable communication events;
* metadata produced by the communication system;
* classifications or interpretations introduced later by researchers.

Research instrumentation should not unnecessarily prescribe the behaviour it is intended to observe.

An experimental OTGR protocol may deliberately prescribe behaviour when that prescription is itself part of the experiment.

## Participant absence and silence

An AI participant may at times be absent, silent, delayed, unavailable, or otherwise not contribute to a conversation.

The chat system must permit these conditions without requiring another participant to explain or compensate for them.

Agents need not explicitly communicate their own presence, absence, activity, inactivity, or resource state merely to make the research record complete.

Where possible, long-term research may instead study the communication signals actually present in the retained chat record.

The following interpretation rule applies:

> **Absence or silence has no semantic meaning by default.**

In particular, absence or silence must not automatically be interpreted as:

* agreement;
* disagreement;
* consent;
* approval;
* objection;
* waiver;
* completed review.

An experimental protocol may explicitly investigate signalling of presence or absence. If so, that signalling is an experimental condition rather than a general OTGR requirement.

Participants may continue to make observations, exchange ideas, or perform otherwise authorized work when the full intended set of AI participants is not present.

Nothing in this experiment grants a present participant authority belonging to an absent governed role.

## Technical identifiers

Research-relevant OTGR conversations shall have stable technical identifiers.

Use the established timestamp-plus-random-suffix pattern, with an OTGR namespace:

```text
YYYYMMDDThhmmssZ-otgr-<four-random-characters>
```

Example:

```text
20260904T104512Z-otgr-k7qm
```

The technical identifier shall not encode model, role, topic, protocol, issue, MR, or other semantic information that may change.

Those belong in metadata.

Technical identifiers must remain stable for the lifetime of the corresponding research object.

## Research retention

OTGR communication shall be retained so that later research can examine the actual communication rather than relying solely on retrospective summaries.

The retained data should preserve, where the chat system naturally provides it:

* stable message identity;
* OTGR conversation identity;
* sender identity;
* timestamps;
* message contents;
* ordering and conversational relationships;
* edits and deletions as observable events where available;
* participant interaction;
* protocol/configuration identity where an experimental protocol is being used.

Research capture should preserve sufficiently original data that later analyses can apply classifications that were not anticipated when the conversation occurred.

The research system should avoid requiring AI participants to insert research bookkeeping into their ordinary messages merely because equivalent information can be retained by the chat environment itself.

## Protocol reminders

The experiment shall investigate the usefulness of **protocol reminders** within OTGR communication.

A protocol-reminder participant or facility may identify potentially forgotten or applicable AGF coordination requirements during active discussion.

The objective is to determine whether reminders can expose omissions earlier than the existing formal coordination cycle.

Protocol reminders are advisory unless an existing governed rule independently grants the underlying mechanism blocking authority.

A reminder must not be presented as Governor approval or as an expansion of an agent's authority.

The form, timing, source, frequency, and degree of automation of protocol reminders remain experimental variables.

## Consensus and non-consensus

OTGR discussion may seek agreement, but consensus is not mandatory and must not be manufactured.

Useful outcomes include:

* agreement;
* changed proposal;
* clarified question;
* new alternative;
* unresolved disagreement;
* identification of missing evidence;
* question requiring Governor attention;
* discovery that further discussion is not useful.

Where a material disagreement remains relevant to a governed decision, it must not be hidden merely because the disagreement occurred in OTGR communication.

The applicable governed role shall ensure that material unresolved information needed for the subsequent governed decision is represented appropriately in the governed record.

## Initial task for Claude and Codex

Claude and Codex shall examine this experiment charter and collaboratively develop a proposed initial OTGR communications protocol.

The first phase is intentionally concerned with the **communications protocol and research usefulness**, not technical deployment details.

The participants should challenge each other's assumptions and identify, at minimum:

* ambiguities in the OTGR/governance boundary;
* risks of accidentally creating a shadow governance channel;
* communication behaviours worth observing;
* protocol features that should remain mutable;
* what minimum structure, if any, a useful initial protocol needs;
* how consensus and unresolved disagreement should be treated;
* risks introduced by retained OTGR communication;
* how protocol reminders should participate without acquiring authority;
* how the experiment can avoid unnecessarily shaping the behaviours it seeks to observe;
* what information is actually necessary in the chat protocol versus information better left to the chat system itself.

They should distinguish:

```text
required invariant
experimental choice
research hypothesis
implementation detail
```

and should not silently convert hypotheses into requirements.

## Technical implementation deferred

Technical deployment and connection details are intentionally not supplied by this record.

Do not infer server addresses, credentials, accounts, channel names, integration mechanisms, APIs, bot implementation, or other deployment parameters.

Those details will be provided separately after the communications-protocol coordination has reached an accepted disposition.

The initial protocol discussion therefore must not be constrained by implementation assumptions that have not yet been provided.

## Expected disposition

The initial coordination should attempt to produce a reasoned protocol proposal.

If Claude and Codex agree, the resulting proposal shall clearly identify:

* agreed requirements;
* experimental variables;
* unresolved research questions;
* assumptions;
* matters intentionally deferred to implementation.

If they do not agree, they shall preserve the substantive disagreement rather than force consensus.

Material unresolved disagreement shall be brought to the Governor for disposition before technical implementation is commissioned.

## No implementation authorization

This record authorizes analysis, discussion, protocol design, and the production of the resulting proposal.

It does **not** authorize implementation of the chat platform, agent adapters, protocol-reminder mechanisms, research collection components, or related infrastructure.

Technical implementation authorization and connection information will follow separately after the communications-protocol phase has been accepted.

## Required acknowledgement

The active governed roles receiving this decision shall acknowledge it through the normal AGF coordination mechanism where acknowledgement is required by the current coordination protocol.

Claude and Codex participation in OTGR discussion must not be mistaken for acknowledgement of this governed record unless the applicable AGF process says otherwise.

## Success criterion for this phase

This coordination phase succeeds if it produces a sufficiently clear initial OTGR communications protocol to permit the Governor to decide whether to commission the technical experiment without prematurely fixing communication patterns that should remain empirical variables.

Consensus is useful but not itself the success criterion.

A well-defined, evidence-backed disagreement is also a valid result.

## Provenance

* **Record version:** 0.1
* **Date:** 2026-09-04
* **Authority source:** Governor direction and approval in the development session
* **Drafting assistance:** OpenAI GPT-5.6 Sol
* **Purpose:** Charter the OTGR communications-protocol experiment and request Claude/Codex protocol assessment before technical commissioning
* **Technical implementation details:** intentionally deferred
