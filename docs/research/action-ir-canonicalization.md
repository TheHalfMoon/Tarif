# Action IR and Canonicalization Research

Last reviewed: 2026-09-02.

This research shapes Specification 002. It does not itself authorize implementation.

## Problem

Tarif cannot authorize one representation of an action and execute a materially different representation. The first product boundary therefore needs a deterministic, narrow representation of a supported MCP `tools/call` request whose security-relevant semantics can be reproduced exactly.

## MCP 2026-07-28 observations

Primary sources:

- https://modelcontextprotocol.io/specification/2026-07-28/basic/index
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/server/tools.mdx
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/changelog.mdx

The 2026-07-28 protocol is stateless at the protocol layer. Every request is self-contained and carries its protocol version and client capabilities in request `_meta`. For Streamable HTTP, a modern request also carries `MCP-Protocol-Version`, `Mcp-Method`, and, for tool calls, `Mcp-Name`; HTTP header/body agreement is deferred to the later MCP gate specification.

For the request body, `io.modelcontextprotocol/protocolVersion` and `io.modelcontextprotocol/clientCapabilities` are required on every request. `clientCapabilities` is per-request state; an empty object means no optional client capabilities, and a server must not infer capabilities from a prior request. `io.modelcontextprotocol/clientInfo` is optional and self-reported. `io.modelcontextprotocol/logLevel` is optional and deprecated in the 2026-07-28 revision but remains in the specification during its compatibility window.

Multi Round-Trip Requests materially affect request meaning. A retry can include `inputResponses` and an opaque echoed `requestState`. The TypeScript SDK treats `requestState` as untrusted input and recommends integrity binding to the principal, originating method/parameters, and expiry.

Therefore a Tarif v0.1 action identity must not silently discard `inputResponses` or `requestState`. The initial bounded profile will reject those retry fields as unsupported rather than authorize only the visible tool name/arguments and accidentally ignore execution state.

### Request `_meta` classification

The MCP 2026-07-28 schema defines reserved request-envelope fields under `io.modelcontextprotocol/*`, including protocol version, client info, client capabilities, and log level. It also documents W3C trace propagation keys `traceparent`, `tracestate`, and `baggage`.

`clientInfo` is self-reported and must not be treated as authenticated identity or authorization authority. However, "self-reported" does not prove that a server cannot observe or branch on a supported envelope field. Silently deleting supported server-visible metadata from an execution identity would therefore make the first boundary weaker than its claim.

The v0.1 decision is:

- request `_meta` is mandatory for the supported MCP 2026-07-28 profile;
- `io.modelcontextprotocol/protocolVersion` is mandatory, must agree with the explicit supported revision, and is represented by `protocol.revision` rather than duplicated in `mcp_context`;
- `io.modelcontextprotocol/clientCapabilities` is mandatory, must be an object, and is always preserved under Action IR `mcp_context`, including when it is `{}`;
- optional present `io.modelcontextprotocol/clientInfo` and `io.modelcontextprotocol/logLevel` are preserved under `mcp_context` as **untrusted execution context**;
- preserved metadata never becomes principal/workload identity or proof of authority;
- standard trace propagation is excluded from Action IR because it is observability context, not action authority;
- every other `_meta` extension key is unsupported and fails closed until explicitly modeled.

This is intentionally conservative. It avoids both trusting self-reported metadata and pretending that unknown server-visible metadata cannot affect execution.

## Tool names

Current MCP tool documentation recommends:

- 1–128 characters;
- case-sensitive treatment;
- ASCII letters, digits, `_`, `-`, and `.` only;
- no spaces, commas, or other special characters.

These are protocol `SHOULD` rules rather than a universal security proof. Tarif v0.1 intentionally adopts the recommended ASCII profile as a stricter supported subset. Non-conforming names fail closed as unsupported. Tarif does not case-fold, Unicode-normalize, infer aliases, or rely on self-reported `serverInfo.name` for cross-server disambiguation.

This is a compatibility narrowing, not a claim that MCP universally forbids every other name.

## JSON canonicalization

Primary source: RFC 8785, JSON Canonicalization Scheme (JCS): https://www.rfc-editor.org/rfc/rfc8785.html

Tarif should use JCS rather than inventing a proprietary canonical JSON algorithm.

Relevant JCS requirements:

- input is constrained to the I-JSON domain;
- object property names must not be duplicated;
- JSON numbers must be expressible as IEEE-754 double-precision values;
- higher-precision or longer integer application values should be represented as strings;
- strings are preserved as-is;
- Unicode normalization is explicitly not performed;
- object properties are deterministically sorted;
- canonical output emits no inter-token whitespace.

Security consequence: two Unicode strings that are canonically equivalent under NFC/NFD but code-point distinct remain distinct Tarif values. Tarif must never normalize them before authorization.

## Rust implementation candidate

`serde_json_canonicalizer` 0.3.2 is an MIT-licensed Rust implementation targeting RFC 8785 and was published in February 2026:

https://docs.rs/crate/serde_json_canonicalizer/0.3.2

It is a candidate dependency, not automatic authority. Specification 002 implementation must wrap any serializer with strict input validation so duplicate object keys and unsupported input cannot be silently collapsed before canonicalization.

The dependency's own documentation warns that arbitrary-precision JSON numbers are converted to doubles and can lose precision. Tarif therefore adopts JCS/I-JSON semantics explicitly; applications requiring exact higher precision must use strings. A later execution adapter must execute from the validated normalized value rather than separately reinterpreting an unvalidated raw number representation.

## Exact Action IR shape

Specification 002 normalizes only the first supported MCP action class. Example with arguments and the minimum required client capabilities context:

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
    "io.modelcontextprotocol/clientCapabilities": {}
  }
}
```

When arguments are omitted:

```json
"arguments": { "state": "absent" }
```

The minimum valid supported context is:

```json
"mcp_context": {
  "io.modelcontextprotocol/clientCapabilities": {}
}
```

`protocolVersion` is intentionally absent from `mcp_context` because the validated request value is represented by `protocol.revision`.

The normalized action is an internal Tarif contract, not a new Internet protocol.

It must preserve the distinction between omitted arguments and an explicitly present empty object. Supported untrusted MCP context is bound to the canonical action but does not become authenticated identity.

It must not embed policy decisions, resource inference, authenticated principal/workload identity, approval state, credentials, or evidence digests. Those belong to later specifications.

## Initial unsupported states

Fail closed for the v0.1 Action IR profile when any of the following is present, absent when required, or otherwise not modeled:

- MCP method other than `tools/call`;
- protocol revision other than the explicitly supported revision;
- missing request `_meta`;
- missing request-envelope `io.modelcontextprotocol/protocolVersion`;
- missing request-envelope `io.modelcontextprotocol/clientCapabilities`;
- request-envelope protocol version that disagrees with the explicit revision;
- `clientCapabilities` present with a non-object value;
- serialized Action IR missing required `io.modelcontextprotocol/clientCapabilities` from `mcp_context`;
- empty or non-profile tool name;
- `arguments` present but not a JSON object;
- duplicate JSON object keys anywhere in the parsed action input;
- invalid Unicode / JCS-incompatible JSON;
- unsupported numeric input outside JCS/I-JSON semantics;
- `inputResponses` or `requestState`;
- task-augmented execution;
- `_meta` extension keys outside the explicit supported-envelope/trace allowlist.

## Required adversarial corpus

Implementation qualification must cover at least:

1. object-key ordering and whitespace equivalence;
2. deterministic repeated canonical output;
3. duplicate-key rejection at nested depths;
4. malformed JSON rejection;
5. JCS number serialization and incompatible-number rejection;
6. precomposed/decomposed Unicode remaining distinct;
7. case-sensitive tool names;
8. non-profile tool-name rejection;
9. omitted arguments remaining distinct from `{}`;
10. `inputResponses` / `requestState` fail-closed behavior;
11. missing required request `_meta` rejection;
12. missing required request protocol version rejection;
13. missing required request client capabilities rejection;
14. unknown `_meta` rejection;
15. protocol-version disagreement rejection;
16. required `clientCapabilities` and optional supported `clientInfo`/`logLevel` changes producing distinct canonical action context without becoming identity authority;
17. trace-only metadata not granting or changing authority semantics;
18. unknown Action IR schema version rejection;
19. serialized Action IR missing required client capabilities context rejection;
20. bounded input/depth behavior selected by the caller or adapter.

## Deferred questions

- final transport-level size/depth defaults belong to the MCP gate/integration specification after evidence about real workloads;
- HTTP header/body agreement for `MCP-Protocol-Version`, `Mcp-Method`, `Mcp-Name`, and `Mcp-Param-*` belongs to the MCP gate specification;
- cryptographic digests and evidence binding belong to Specification 005;
- approval binding belongs to Specification 006;
- AuthZEN subject/action/resource/context mapping belongs to the decision/policy layer, not canonicalization;
- A2A and HTTP normalization remain future work.
