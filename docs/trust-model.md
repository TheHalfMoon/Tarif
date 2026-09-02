# Tarif Trust Model — Founding Draft

This document is a founding boundary, not a completed security proof. Specification 001 must refine it against reproducible threat cases before production behavior is authorized.

## Core separation

Tarif separates **proposal** from **authority**.

Humans, models, agents, tools, remote services, and policy-generation assistants may propose actions or authority. Deterministic Tarif components decide whether supported execution paths satisfy the configured authorization contract.

## Untrusted by default

The following are not authorization authority merely because they claim success or intent:

- LLM reasoning or natural-language self-report;
- prompt or retrieved content;
- remote tool descriptions;
- agent-generated approval summaries;
- sub-agent claims;
- repository content;
- unverified runtime metadata;
- executor success responses.

## Trusted or conditionally trusted inputs

Depending on deployment and integration, the trusted computing base may include:

- Tarif deterministic canonicalization and enforcement code;
- configured policy decision point or embedded policy engine;
- verified identity/workload assertions;
- explicit human approval captured over trusted fields;
- trusted credential source/broker;
- verified resource state used for pre-execution revalidation;
- cryptographic and platform primitives selected by the active specification.

Specification 001 must state which of these are in the v0.1 TCB and which remain external assumptions.

## Security invariants

1. Unknown consequential actions fail closed.
2. Authorization binds to the normalized action and relevant parameters, not only a broad tool name.
3. Approval must not silently survive a material state change outside its authorized binding.
4. The model is not the root of trust for the description of what will execute.
5. The model should not need direct access to downstream long-lived secrets.
6. Evidence records must not claim semantic correctness merely because a digest exists.
7. Tarif must distinguish mediated, partially mediated, and unmediated execution paths.

## Founding threat corpus

Specification 001 must classify at least:

- prompt injection;
- confused deputy behavior;
- credential exfiltration;
- privilege escalation;
- argument substitution;
- tool-name spoofing;
- replay;
- stale approval;
- TOCTOU between approval and execution;
- delegation escalation;
- malformed/ambiguous requests;
- policy bypass;
- direct network bypass;
- direct shell bypass;
- compromised MCP server/tool;
- compromised agent process;
- evidence tampering.

Each threat should be marked `MITIGATED`, `PARTIALLY_MITIGATED`, `DETECTED`, `OUT_OF_SCOPE`, or `UNRESOLVED` with exact evidence. No status is inferred from intent alone.
