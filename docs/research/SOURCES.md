# Tarif Founding Source Map

Last reviewed: 2026-09-02.

This is a curated research map, not a dependency manifest. Sources are classified so a new standard or competitor does not silently become implementation authority.

## Classification

- **ADOPT** — stable/open standard or contract Tarif should prefer where applicable.
- **COMPOSE** — mature primitive Tarif should integrate rather than recreate.
- **EVALUATE** — candidate implementation dependency requiring a separate fit/security/maintenance decision.
- **LEARN** — architectural or research prior art.
- **COMPETE** — product whose paid capability informs Tarif's open-source replacement target.
- **WATCH** — evolving draft/proposal; do not make it a hard dependency without later qualification.

## Tier 0 — mandatory founding reading

| Source | Class | Tarif relevance |
| --- | --- | --- |
| MCP specification / authorization | ADOPT | Initial transport and authorization surface |
| OpenID AuthZEN Authorization API | ADOPT | Standard PEP↔PDP decision contract |
| COAZ / COAZ-MCP | ADOPT/WATCH | MCP operation/parameter mapping to authorization context |
| NIST Software & AI Agent Identity and Authorization | LEARN | Official problem/threat framing |
| OAuth 2.0 Security BCP (RFC 9700) | ADOPT | OAuth security baseline |
| OAuth Token Exchange (RFC 8693) | ADOPT | Delegation/brokering primitive |
| Rich Authorization Requests (RFC 9396) | ADOPT | Structured authorization details |
| DPoP (RFC 9449) | ADOPT | Sender-constrained token primitive |
| OAuth mTLS (RFC 8705) | ADOPT | Certificate-bound token primitive |
| OAuth Protected Resource Metadata (RFC 9728) | ADOPT | Resource/AS discovery used by MCP |
| SPIFFE/SPIRE | COMPOSE | Workload identity and attestation |
| Envoy ext_authz | LEARN | Mature PEP/data-plane pattern |
| Cedar | EVALUATE | Embedded authorization model reference |
| OPA | COMPOSE | Mature external PDP reference |
| Google AP2 | LEARN | Mandates, exact authority binding, deterministic validation, receipts |

## Stable OAuth and request-integrity primitives

- RFC 8414 — OAuth 2.0 Authorization Server Metadata
- RFC 8707 — Resource Indicators for OAuth 2.0
- RFC 9068 — JWT Profile for OAuth 2.0 Access Tokens
- RFC 8725 — JSON Web Token Best Current Practices
- RFC 7636 — PKCE
- RFC 9126 — Pushed Authorization Requests
- RFC 9101 — JWT-Secured Authorization Request
- RFC 9470 — OAuth 2.0 Step Up Authentication Challenge Protocol
- RFC 9421 — HTTP Message Signatures
- OpenID CIBA Core — asynchronous/backchannel human authorization pattern
- OpenID Shared Signals / CAEP — revocation and state-change signaling reference

## Emerging agent-authorization work — WATCH

These are not canonical dependencies while unstable:

- IETF `draft-klrc-aiagent-auth` — AI Agent Authentication and Authorization
- Mission-Bound Authorization for OAuth 2.0
- Mission-Bound Runtime Enforcement
- AAuth Protocol
- Agent Operation Authorization
- WIMSE Credential Delegation Protocol for AI Agents
- WIMSE Agentic Execution Context / execution-context-token work
- signed authorization/evidence work associated with runtime agent execution

Policy: a draft may shape tests, adapters, or interoperability experiments, but a draft alone does not authorize a proprietary Tarif fork or hard architectural dependency.

## Capability and delegation research

- Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud
- Biscuit authorization tokens
- AIP: Agent Identity Protocol for Verifiable Delegation Across MCP and A2A
- Google Zanzibar
- OpenFGA
- SpiceDB

Use these to understand attenuation, relationship authorization, delegation, offline verification, and authority graphs. Do not adopt a capability-token system without a bounded comparison against OAuth/standards-native alternatives.

## Evidence and provenance

- RFC 9943 / SCITT architecture
- in-toto
- SLSA provenance
- Sigstore / Cosign / Rekor
- OpenTelemetry

Tarif should reuse established provenance, signing, release-integrity, and telemetry ecosystems rather than invent parallel transports.

## Credential isolation

- OpenBao
- HashiCorp Vault dynamic secrets and leases
- cloud secret managers / workload identity systems

Tarif should broker or derive credentials around authorization. It should not become a generic secret store unless future evidence selects that scope.

## Commercial requirements-mining targets

- Descope Agentic Identity Hub — COMPETE
- Aembit Agentic IAM — COMPETE
- Permit MCP Gateway / agent authorization — COMPETE
- Cerbos / AuthZEN MCP authorization patterns — COMPETE/COMPOSE
- Auth0 for AI Agents / Token Vault — COMPETE/COMPOSE
- Okta for AI Agents / Agent-to-Agent Connections — COMPETE/COMPOSE
- Microsoft Entra Agent ID — COMPETE/COMPOSE

Do not publish "Tarif replaces X" until a later reproducible migration/feature qualification establishes the exact replacement boundary.

## Open-source prior art

- `agentgateway/agentgateway`
- Alibaba `open-agent-auth`
- Better Auth Agent Auth Protocol
- `maxmalkin/AgentAuth`
- official Model Context Protocol Rust SDK

Every donor or prior-art use must remain license-aware and independently attributable. Research does not authorize code copying.

## Runtime integration targets

- Claude Code / Anthropic MCP
- OpenAI Codex
- OpenCode
- Hermes Agent
- generic MCP clients and servers

Framework-specific adapters must not become the only route to the core authority contract.

## URLs

Primary references:

- https://modelcontextprotocol.io/specification/
- https://blog.modelcontextprotocol.io/posts/2026-07-28/
- https://openid.net/wg/authzen/specifications/
- https://csrc.nist.gov/pubs/other/2026/02/05/accelerating-the-adoption-of-software-and-ai-agent/ipd
- https://www.rfc-editor.org/rfc/rfc9700.html
- https://www.rfc-editor.org/rfc/rfc8693.html
- https://www.rfc-editor.org/rfc/rfc9396.html
- https://www.rfc-editor.org/rfc/rfc9449.html
- https://www.rfc-editor.org/rfc/rfc8705.html
- https://www.rfc-editor.org/rfc/rfc9728.html
- https://spiffe.io/docs/latest/
- https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/security/ext_authz_filter.html
- https://docs.cedarpolicy.com/
- https://www.openpolicyagent.org/docs
- https://github.com/google-agentic-commerce/AP2
- https://datatracker.ietf.org/doc/draft-klrc-aiagent-auth/
- https://datatracker.ietf.org/doc/draft-mcguinness-oauth-mission/
- https://mcguinness.github.io/mission-bound-authorization/draft-mcguinness-mission-runtime.html
- https://datatracker.ietf.org/doc/draft-hardt-oauth-aauth-protocol/
- https://datatracker.ietf.org/doc/draft-liu-agent-operation-authorization/
- https://datatracker.ietf.org/doc/draft-sweeney-wimse-credential-delegation/
- https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/
- https://www.biscuitsec.org/
- https://arxiv.org/abs/2603.24775
- https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/
- https://openfga.dev/docs
- https://authzed.com/docs/spicedb
- https://www.rfc-editor.org/rfc/rfc9943.html
- https://in-toto.io/
- https://slsa.dev/
- https://docs.sigstore.dev/
- https://opentelemetry.io/docs/specs/otel/
- https://openbao.org/docs/
- https://developer.hashicorp.com/vault/docs
- https://docs.descope.com/agentic-identity-hub
- https://aembit.io/
- https://www.permit.io/
- https://www.cerbos.dev/
- https://auth0.com/ai
- https://www.okta.com/ai/
- https://learn.microsoft.com/en-us/entra/agent-id/
- https://github.com/agentgateway/agentgateway
- https://github.com/alibaba/open-agent-auth
- https://github.com/better-auth/agent-auth-protocol
- https://github.com/maxmalkin/AgentAuth
- https://github.com/modelcontextprotocol/rust-sdk
