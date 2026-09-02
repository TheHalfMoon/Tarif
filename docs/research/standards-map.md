# Tarif Standards Map

## Founding rule

Tarif should innovate at **composition, enforcement coverage, action normalization, approval/state binding, credential isolation, explainability, and developer experience** before inventing new identity or authorization protocols.

## Stable or mature building blocks

| Problem | Preferred starting point | Tarif posture |
| --- | --- | --- |
| Human identity | OIDC / existing IdP | Integrate; do not build a general IdP |
| OAuth security | RFC 9700 family | Adopt |
| Resource discovery | RFC 9728 / MCP authorization | Adopt |
| Delegated/token exchange | RFC 8693 | Adopt where applicable |
| Structured authorization | RFC 9396 | Adopt where applicable |
| Sender-bound tokens | DPoP / mTLS | Adopt where applicable |
| Workload identity | SPIFFE/SPIRE / cloud workload identity | Compose |
| PEP↔PDP API | AuthZEN | Prefer for external policy integration |
| Embedded policy | Cedar candidate | Evaluate, do not assume |
| External policy | OPA/Cerbos/AuthZEN-compatible PDPs | Adapter strategy |
| MCP transport | MCP specification / official SDK | Adopt |
| Telemetry | OpenTelemetry | Adopt |
| Release provenance | Sigstore/SLSA/in-toto | Adopt for release integrity |
| Generic secret storage | OpenBao/Vault/cloud managers | Integrate, do not replace |

## Emerging work that must remain version-aware

The following may materially affect Tarif but are not stable foundations merely because they are recent:

- AI Agent Authentication and Authorization drafts;
- Mission-Bound Authorization;
- Mission-Bound Runtime Enforcement;
- AAuth;
- Agent Operation Authorization;
- WIMSE credential delegation and execution-context work.

For each integration, record:

```text
exact draft/version
retrieval date
fields/semantics consumed
compatibility tests
fallback behavior
upgrade/review trigger
```

## Do-not-reinvent list

Without a written accepted ADR and a reproduced gap, Tarif must not create:

- a new cryptographic primitive;
- a new OAuth grant;
- a new identity protocol;
- a new JWT-like token format;
- a new workload identity system;
- a new generic policy language;
- a new secrets manager;
- a new telemetry transport;
- a new release-signing ecosystem;
- a new mission protocol merely to brand the project.

## Internal Action IR

Tarif may require an internal normalized **Action IR** to map MCP, future HTTP, and future A2A requests into one deterministic authorization input. That internal representation is not automatically a public protocol. Specification 002 must prove why each field is required and define canonicalization/ambiguity behavior before product code is authorized.
