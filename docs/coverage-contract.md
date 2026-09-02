# Tarif Enforcement Coverage Contract — v0.1 Target

## Purpose

Tarif must never collapse partial mediation into a generic "agent protected" claim.

Coverage is a first-class security fact.

## Initial target boundary

The first implementation wedge may claim enforcement only for **supported MCP `tools/call` requests that actually traverse the configured Tarif mediation path**.

A supported mediated call should eventually be observable as:

```text
MEDIATED
  protocol: MCP
  method: tools/call
  normalized action: <known>
  decision: ALLOW | DENY | APPROVAL_REQUIRED
```

## Coverage states

Tarif uses at least:

- `MEDIATED` — request is known to traverse the supported enforcement path;
- `PARTIALLY_MEDIATED` — only some security-relevant subpaths/credentials are controlled;
- `UNMEDIATED` — Tarif does not enforce the path;
- `UNKNOWN` — Tarif lacks evidence to classify the path.

`UNKNOWN` must not be promoted to `MEDIATED` by assumption.

## Initial non-mediated surfaces

Until separately implemented and qualified, the following remain outside the initial MCP enforcement claim:

- arbitrary shell execution;
- arbitrary filesystem access;
- arbitrary direct HTTP clients;
- arbitrary direct network sockets;
- browser automation;
- A2A calls;
- model-provider traffic;
- tools invoked outside the configured Tarif path;
- credentials already directly visible to the agent.

## Bypass honesty rule

If an agent can perform the same consequential effect through an unmediated path, Tarif must surface that as residual/bypass risk. A successful MCP policy decision does not prove the host as a whole is constrained.

## `tarif doctor` target

Specification 007 should make known coverage machine-readable and human-readable, including warnings such as:

```text
MCP tools/call        MEDIATED
Direct HTTP           UNMEDIATED
Shell                 UNMEDIATED
Model-visible token   CRITICAL BYPASS RISK
Overall               PARTIAL MEDIATION
```

Exact detection capability must be evidence-backed; doctor must not claim to detect paths it cannot observe.

## Claim vocabulary

Preferred narrow wording:

> "Tarif mediates supported MCP tools/call requests through the configured local gate."

Prohibited without later broader evidence:

- "Tarif secures this agent."
- "Tarif sandboxes Claude/Codex."
- "Tarif prevents all unauthorized actions."
- "Tarif blocks prompt injection."

## Failure behavior

For a request inside the declared mediated surface, unknown or ambiguous consequential operations should fail closed according to the active decision/canonicalization specifications.

For a request outside the declared surface, Tarif should report absence of enforcement rather than fabricate a security decision.
