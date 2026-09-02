# ADR-0001 — MCP Action IR and JSON Canonicalization

Status: `ACCEPTED_FOR_SPEC_002`
Date: 2026-09-02

## Context

Tarif needs one deterministic representation of a consequential action before a policy decision or approval can be safely bound to it. A generic cross-protocol ontology would be premature, and proprietary canonicalization would conflict with the project's standards-before-invention rule.

MCP 2026-07-28 also introduces execution-relevant retry state (`inputResponses`, `requestState`) and per-request `_meta`. Ignoring fields that a server can observe could make two materially different executions share one authority identity.

## Decision

1. Specification 002 is intentionally MCP-first and supports only baseline `tools/call` normalization.
2. The internal schema identifier is `tarif.action/v1`.
3. The canonical Action IR shape is:

```json
{
  "schema": "tarif.action/v1",
  "protocol": {
    "name": "mcp",
    "revision": "2026-07-28"
  },
  "operation": "tools/call",
  "target": {
    "kind": "mcp_tool",
    "name": "search"
  },
  "arguments": {
    "state": "present",
    "value": {
      "q": "otters"
    }
  },
  "mcp_context": {
    "io.modelcontextprotocol/clientInfo": {
      "name": "my-app",
      "version": "1.0"
    }
  }
}
```

When `arguments` is omitted on the MCP request, the normalized form is exactly:

```json
"arguments": { "state": "absent" }
```

`mcp_context` contains only supported, present, non-trace MCP envelope fields that the server may observe. Absence remains distinct from presence. Empty `mcp_context` is represented as `{}`.

4. The supported MCP envelope fields for v0.1 are:
   - `io.modelcontextprotocol/protocolVersion` — MUST agree with the explicit supported revision and is represented by `protocol.revision`, not duplicated in `mcp_context`;
   - `io.modelcontextprotocol/clientInfo` — preserved in `mcp_context` as untrusted execution context;
   - `io.modelcontextprotocol/clientCapabilities` — preserved in `mcp_context` as untrusted execution context;
   - `io.modelcontextprotocol/logLevel` — preserved in `mcp_context` as untrusted execution context.
5. `clientInfo`, `clientCapabilities`, and `logLevel` are never interpreted as principal/workload identity or authorization authority. They are bound only so a server-visible execution-context difference cannot be silently erased.
6. W3C trace propagation metadata (`traceparent`, `tracestate`, `baggage`) is observability context and is excluded from canonical Action IR. The MCP integration layer may forward it, but Tarif policy must not infer authority from it.
7. Any other `_meta` extension key is unsupported in this first profile and fails closed rather than being silently omitted.
8. Canonical bytes use RFC 8785 JCS semantics.
9. No Unicode normalization or tool-name case folding occurs.
10. Duplicate JSON keys fail closed before a generic map representation can erase them.
11. The initial tool-name support profile is the MCP recommended 1–128 ASCII `[A-Za-z0-9_.-]` subset. Non-profile names are unsupported rather than rewritten.
12. `inputResponses`, `requestState`, and task-augmented execution fail closed until explicitly modeled.
13. Policy decisions, resource inference, authenticated identity, approvals, credentials, action digests, and evidence are outside this IR specification.

## Consequences

### Positive

- authorization and later execution can share one deterministic action value;
- server-visible supported execution context is bound without being mistaken for authenticated identity;
- JCS avoids a proprietary canonical JSON format;
- the v0.1 support claim stays narrow and auditable;
- potentially dangerous protocol evolution fails closed instead of being silently ignored.

### Costs

- some valid/non-recommended MCP tool names will initially be unsupported;
- unknown `_meta` extensions and MRTR/task-augmented tool calls are not covered by the first Action IR;
- supported self-reported MCP envelope context changes canonical action bytes even though it does not grant authority;
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

### Drop all self-reported MCP metadata from Action IR

Rejected because "not authenticated identity" does not mean "cannot affect server behavior". Supported server-visible envelope fields are therefore bound as untrusted context instead of being silently erased.

### Canonicalize trace IDs into action identity

Rejected because standard trace propagation is observability context, not action authority. Trace metadata remains outside the Action IR and must never become an authorization input.

## Revisit triggers

Revisit only with bounded evidence showing a required use case cannot be represented safely by this profile, or when a later protocol specification requires an expanded action class.
