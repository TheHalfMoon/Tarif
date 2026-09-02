# Action IR and Canonicalization Research

Last reviewed: 2026-09-02.

This research shapes Specification 002. It does not itself authorize implementation.

## Problem

Tarif cannot authorize one representation of an action and execute a materially different representation. The first product boundary therefore needs a deterministic, narrow representation of a supported MCP `tools/call` request whose security-relevant semantics can be reproduced exactly.

## MCP 2026-07-28 observations

Primary source: https://blog.modelcontextprotocol.io/posts/2026-07-28/

The 2026-07-28 protocol is stateless at the protocol layer. A modern Streamable HTTP request carries `MCP-Protocol-Version`, `Mcp-Method`, and, for tool calls, `Mcp-Name`; the body carries `method = tools/call`, `params.name`, optional `arguments`, and request metadata.

Multi Round-Trip Requests materially affect request meaning. A retry can include `inputResponses` and an opaque echoed `requestState`. The TypeScript SDK treats `requestState` as untrusted input and recommends integrity binding to the principal, originating method/parameters, and expiry.

Therefore a Tarif v0.1 action identity must not silently discard `inputResponses` or `requestState`. The initial bounded profile will reject those retry fields as unsupported rather than authorize only the visible tool name/arguments and accidentally ignore execution state.

Reserved request `_meta` envelope values such as `clientInfo` are self-reported. MCP SDK guidance says `clientInfo`/`serverInfo` are for display/logging/debugging and must not be security decision inputs. Trace context is observability metadata, not action authority.

Unknown or extension `_meta` keys may carry semantics that Tarif has not modeled. The initial profile therefore rejects unknown execution-affecting metadata instead of silently dropping it from the authority identity.

## Tool names

Current MCP tool documentation recommends:

- 1–128 characters;
- case-sensitive treatment;
- ASCII letters, digits, `_`, `-`, and `.` only;
- no spaces, commas, or other special characters.

These are protocol `SHOULD` rules rather than a universal security proof. Tarif v0.1 intentionally adopts the recommended ASCII profile as a stricter supported subset. Non-conforming names fail closed as unsupported. Tarif does not case-fold, Unicode-normalize, or infer aliases.

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

Security consequence: two Unicode strings that are canonically equivalent under NFC/NFD but byte/code-point distinct remain distinct Tarif values. Tarif must never normalize them before authorization.

## Rust implementation candidate

`serde_json_canonicalizer` 0.3.2 is an MIT-licensed Rust implementation targeting RFC 8785 and was published in February 2026:

https://docs.rs/crate/serde_json_canonicalizer/0.3.2

It is a candidate dependency, not automatic authority. Specification 002 implementation must wrap any serializer with strict input validation so duplicate object keys and unsupported input cannot be silently collapsed before canonicalization.

The dependency's own documentation warns that arbitrary-precision JSON numbers are converted to doubles and can lose precision. Tarif therefore adopts JCS/I-JSON semantics explicitly; applications requiring exact higher precision must use strings. The execution adapter must execute from the validated normalized value, not separately reinterpret an unvalidated raw number representation.

## Action model boundary

Specification 002 should normalize only the first supported MCP action class:

```text
protocol = mcp
revision = 2026-07-28
operation = tools/call
target = exact tool name
arguments = explicit absent/present state plus validated JSON object
```

The normalized action is an internal Tarif contract, not a new Internet protocol.

It must preserve the distinction between omitted arguments and an explicitly present empty object.

It must not treat self-reported MCP client metadata as a principal or workload identity. Identity binding belongs to later integrations.

It must not embed policy decisions, resource inference, approval state, credentials, or evidence digests. Those belong to later specifications.

## Initial unsupported states

Fail closed for the v0.1 Action IR profile when any of the following is present or required but not modeled:

- MCP method other than `tools/call`;
- protocol revision other than the explicitly supported revision;
- empty or non-profile tool name;
- `arguments` present but not a JSON object;
- duplicate JSON object keys anywhere in the parsed action input;
- invalid Unicode / JCS-incompatible JSON;
- unsupported numeric input outside JCS/I-JSON semantics;
- `inputResponses` or `requestState`;
- unknown execution-affecting `_meta` extensions;
- task-augmented or other extension semantics not explicitly represented.

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
11. unknown/unsupported metadata fail-closed behavior;
12. unknown Action IR schema version rejection;
13. bounded input/depth behavior selected by the caller or adapter.

## Deferred questions

- final transport-level size/depth defaults belong to the MCP gate/integration specification after evidence about real workloads;
- cryptographic digests and evidence binding belong to Specification 005;
- approval binding belongs to Specification 006;
- AuthZEN subject/action/resource/context mapping belongs to the decision/policy layer, not canonicalization;
- A2A and HTTP normalization remain future work.
