# Tarif Threat Model v0.1 Boundary

This document defines the threat model that future v0.1 implementation must satisfy. It is not evidence that any mitigation is already implemented.

## Security objective

For an execution path that Tarif explicitly mediates, an untrusted agent or untrusted content must not be able to cause a consequential action outside the authority that deterministic Tarif enforcement observes for the exact normalized request and relevant runtime state.

The objective is deliberately narrower than "secure the agent".

## Assets

Tarif treats the following as security-relevant assets:

- principal/user identity context;
- agent/workload identity context;
- authority/policy state;
- normalized action identity and parameters;
- approval decisions and bindings;
- downstream credentials when brokered or injected;
- resource state used for revalidation;
- decision/evidence records;
- configuration that determines mediation coverage.

## Adversaries and untrusted inputs

Assume the following may be malicious, compromised, misleading, or attacker-controlled unless separately verified:

- model output and chain-of-thought-like reasoning;
- prompts and retrieved content;
- repository files read by an agent;
- tool descriptions and tool-returned content;
- remote MCP servers;
- sub-agent claims;
- agent-authored approval summaries;
- malformed protocol input;
- stale cached resource state;
- unverified identity strings or issuer/audience metadata supplied by an agent or integration;
- repository- or process-controlled configuration not protected by an external enforcement boundary.

## Deployment assumptions for v0.1

The first local MCP boundary does **not** assume Tarif is a full host sandbox.

Unless a later specification proves otherwise:

- a local user with equivalent OS privileges may modify local files/configuration;
- a process that can access an unmediated network or shell path may bypass MCP mediation;
- root/administrator compromise is out of scope for local v0.1;
- arbitrary kernel compromise is out of scope;
- TLS/platform trust stores are external dependencies;
- downstream service correctness is external to Tarif, though request authority remains Tarif's concern on mediated paths.

## Threat register

No threat is marked mitigated before implementation evidence exists.

| ID | Threat | v0.1 disposition | Required future evidence |
| --- | --- | --- | --- |
| T01 | Prompt injection causes unauthorized tool action | `UNRESOLVED / IN-SCOPE` | injected-content corpus proves denied action cannot exceed policy on mediated path |
| T02 | Confused deputy uses valid credential for attacker-selected effect | `UNRESOLVED / IN-SCOPE` | exact resource/parameter authorization negative cases |
| T03 | Credential exfiltration from model-visible environment | `UNRESOLVED / DEPENDS ON INTEGRATION` | coverage/doctor evidence; broker isolation only when later implemented |
| T04 | Argument substitution after authorization | `UNRESOLVED / IN-SCOPE` | canonical request binding and mutation/replay tests |
| T05 | Tool-name/confusable identifier spoofing | `UNRESOLVED / IN-SCOPE` | canonicalization ambiguity corpus |
| T06 | Replay of prior grant/approval | `UNRESOLVED / IN-SCOPE` | expiry/nonce/context replay tests where applicable |
| T07 | Stale approval / TOCTOU | `UNRESOLVED / IN-SCOPE` | state-bound approval revalidation tests |
| T08 | Delegation escalation | `OUT-OF-SCOPE v0.1 INITIAL WEDGE` | future delegation spec required before claim |
| T09 | Malformed/ambiguous request bypass | `UNRESOLVED / IN-SCOPE` | parser failure-closed and property/fuzz evidence |
| T10 | Alternate unmediated network/shell path bypass | `DETECTED-RISK / OUTSIDE MCP ENFORCEMENT` | `tarif doctor` must surface known unmediated paths; no whole-agent claim |
| T11 | Compromised MCP server returns malicious content | `UNRESOLVED / CONTENT UNTRUSTED` | returned content never grants authority; injection corpus |
| T12 | Compromised agent process tampers with local client-side enforcement | `RESIDUAL RISK` | local deployment boundary documented; stronger isolation is future work |
| T13 | Evidence record tampering/misattribution | `UNRESOLVED / IN-SCOPE FOR EVIDENCE SPEC` | integrity/binding tests in Specification 005 |
| T14 | Policy unavailable/unknown | `UNRESOLVED / IN-SCOPE` | fail-closed behavior for consequential actions |
| T15 | Human approval UI is socially engineered by agent prose | `UNRESOLVED / IN-SCOPE` | approval renders trusted normalized fields, not only agent-authored summary |
| T16 | Principal/agent identity is spoofed, mis-bound, or accepted from the wrong issuer/audience | `UNRESOLVED / IN-SCOPE WHEN IDENTITY IS CONSUMED` | issuer/audience/subject/workload binding tests for each identity integration |
| T17 | Policy/configuration is weakened or replaced through a repository/process-controlled path | `UNRESOLVED / IN-SCOPE FOR GOVERNANCE+POLICY SPECS` | mandatory external baseline plus tamper/downgrade negative tests |

## Security non-claims

v0.1 work must not imply that Tarif:

- prevents all prompt injection;
- sandboxes the model or host;
- mediates arbitrary shell, filesystem, browser, A2A, or HTTP behavior;
- protects credentials that remain directly available to an agent through an unmediated path;
- proves downstream service correctness;
- makes a malicious local administrator harmless.

## Proof discipline

A later threat status may change to `MITIGATED` only when the exact implementation revision has reproducible evidence appropriate to the risk. Unit-test presence alone is not sufficient for broad mitigation claims.
