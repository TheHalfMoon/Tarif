# ADR-0001 — MCP Action IR and JSON Canonicalization

Status: `ACCEPTED_FOR_SPEC_002`
Date: 2026-09-02

## Context

Tarif needs one deterministic representation of a consequential action before a policy decision or approval can be safely bound to it. A generic cross-protocol ontology would be premature, and proprietary canonicalization would conflict with the project's standards-before-invention rule.

MCP 2026-07-28 also introduces execution-relevant retry state (`inputResponses`, `requestState`) and request metadata. Ignoring those fields could make two materially different executions share one authority identity.

## Decision

1. Specification 002 is intentionally MCP-first and supports only baseline `tools/call` normalization.
2. The internal schema identifier is `tarif.action/v1`.
3. The normalized action records:
   - protocol `mcp`;
   - explicit supported protocol revision;
   - operation `tools/call`;
   - exact case-sensitive tool name;
   - an explicit absent/present arguments state;
   - validated JSON object arguments when present.
4. Canonical bytes use RFC 8785 JCS semantics.
5. No Unicode normalization or tool-name case folding occurs.
6. Duplicate JSON keys fail closed before a generic map representation can erase them.
7. The initial tool-name support profile is the MCP recommended 1–128 ASCII `[A-Za-z0-9_.-]` subset. Non-profile names are unsupported rather than rewritten.
8. `inputResponses`, `requestState`, task-augmented execution, and unknown execution-affecting metadata fail closed until explicitly modeled.
9. Self-reported MCP client/server metadata is not identity authority.
10. Policy decisions, resource inference, approvals, credentials, action digests, and evidence are outside this IR specification.

## Consequences

### Positive

- authorization and later execution can share one deterministic action value;
- JCS avoids a proprietary canonical JSON format;
- the v0.1 support claim stays narrow and auditable;
- potentially dangerous protocol evolution fails closed instead of being silently ignored.

### Costs

- some valid/non-recommended MCP tool names will initially be unsupported;
- MRTR/task-augmented tool calls are not covered by the first Action IR;
- strict JSON validation is required before canonicalization;
- JCS/I-JSON number semantics may require high-precision application values to be encoded as strings.

## Rejected alternatives

### Generic MCP+A2A+HTTP IR now

Rejected as speculative abstraction without implementation evidence.

### Canonical CBOR

Not selected for the first boundary because the input and MCP ecosystem are JSON-native and JCS directly solves deterministic JSON serialization.

### Custom sorted JSON

Rejected because subtle number, Unicode, escaping, and ordering rules already have an established standard in RFC 8785.

### Normalize Unicode/tool names

Rejected because it would alter identity-bearing input and conflict with JCS string preservation and MCP case sensitivity.

## Revisit triggers

Revisit only with bounded evidence showing a required use case cannot be represented safely by this profile, or when a later protocol specification requires an expanded action class.
