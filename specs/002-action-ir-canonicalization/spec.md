# Specification 002 — MCP Action IR & Deterministic Canonicalization

Status: `ACTIVE_R3_IMPLEMENTATION_AFTER_SHAPING_MERGE`
Risk: `R3`

## Outcome

Tarif can transform one supported MCP 2026-07-28 baseline `tools/call` request into a strict `tarif.action/v1` value and deterministic RFC 8785 canonical bytes, while failing closed on ambiguous, malformed, unsupported, or execution-relevant state that the first IR does not model.

## Security invariant

Two executions that differ in any security-relevant field modeled by this specification must not silently collapse to the same normalized action because of case folding, Unicode normalization, duplicate-key loss, omitted/present-field collapse, or ignored retry/extension state.

## Scope in

- minimal Rust workspace required for `tarif-core`;
- strict Action IR data types for the bounded MCP `tools/call` profile;
- RFC 8785 JCS-compatible canonical serialization;
- duplicate-key-safe strict JSON ingestion;
- exact tool-name validation for the initial 1–128 ASCII `[A-Za-z0-9_.-]` support profile;
- explicit distinction between absent arguments and present object arguments;
- rejection of unsupported MRTR/task/unknown execution-affecting metadata;
- deterministic error taxonomy suitable for later adapters;
- adversarial, negative-path, property/fuzz-style tests where justified;
- qualification workflow extension with Rust fmt/clippy/test under existing check context `qualification`.

## Scope out

- network/stdio MCP proxying or forwarding;
- `Mcp-Method`/`Mcp-Name` HTTP-header agreement enforcement (Specification 004 integration concern);
- AuthZEN/PDP mapping or authorization decisions;
- identity/principal/workload authentication;
- credentials or secrets;
- approvals/revalidation;
- cryptographic action digest or evidence chain;
- MRTR support beyond explicit rejection;
- task-augmented execution support;
- A2A/HTTP action normalization;
- information-flow tracking;
- release publication.

## Acceptance

1. Canonicalization of semantically equivalent supported JSON object ordering/whitespace is byte-identical under JCS.
2. Duplicate keys at any nested depth are rejected before value collapse.
3. Invalid Unicode or JCS-incompatible input fails closed.
4. Tool names are preserved exactly and are case-sensitive; v0.1 rejects names outside the explicit supported profile instead of rewriting them.
5. Omitted `arguments` and present `{}` produce distinct Action IR values/canonical bytes.
6. Arguments, when present, must be a JSON object.
7. `inputResponses`, `requestState`, task-augmented state, or unknown execution-affecting metadata cannot be silently omitted from action identity; unsupported states are rejected.
8. Self-reported MCP `clientInfo`/`serverInfo` never becomes security identity input.
9. Repeated normalization of the same supported input is deterministic.
10. The implementation exposes stable typed errors for unsupported protocol/version/method/name/state and malformed/canonicalization failures.
11. `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `cargo test --workspace --all-targets --locked` pass on the exact implementation head and are executed under the existing `qualification` workflow context.
12. R3 negative/adversarial evidence covers the corpus defined in `docs/research/action-ir-canonicalization.md`.
13. No product claim exceeds the narrow supported MCP Action IR boundary.

## Dependency decision boundary

`serde_json_canonicalizer` 0.3.2 is the preferred JCS implementation candidate because it targets RFC 8785 and is MIT licensed. Adoption is authorized only if the implementation branch documents the dependency and the required adversarial corpus confirms the wrapper's strict-input contract. `serde`/`serde_json` may be used as supporting Rust serialization primitives.

No additional runtime dependency is authorized without a separate documented need.

## Merge gate

Implementation work may proceed on a bounded R3 branch after this shaping specification is canonically merged and re-read.

**No Specification 002 R3 implementation PR may merge while Issue #3 remains open.** Exact-head test success alone does not replace the missing external branch/ruleset enforcement proof.

## Claim boundary after implementation but before merge

Allowed on the implementation branch only:

> The candidate implementation passes the recorded Action IR/canonicalization tests on the stated exact revision.

Not allowed until canonical merge and later product qualification:

- production-ready authorization;
- MCP gateway protection;
- prompt-injection prevention;
- full MCP support;
- paid-product replacement or security-superiority claims.
