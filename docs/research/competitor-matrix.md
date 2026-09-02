# Tarif Competitor and Replacement Research

This file records requirements-mining targets. It does **not** claim that Tarif currently replaces any product.

## Commercial targets

| Product | Relevant capabilities to study | Tarif response hypothesis |
| --- | --- | --- |
| Descope Agentic Identity Hub | agent identities, MCP OAuth, policy, connections/credential vaulting, token exchange, audit | Open self-hosted runtime authority + integrations |
| Aembit | workload/agent identity, credential delivery, policy, attestation | No per-agent tax; standards-native identity composition |
| Permit | MCP gateway, fine-grained auth, consent/HITL, delegated trust, audit | OSS enforcement and approvals |
| Cerbos | PDP, AuthZEN patterns, MCP gateway/proxy authorization | Interoperate rather than recreate PDP |
| Auth0 for AI Agents | human auth, Token Vault, asynchronous authorization, FGA | Integrate OIDC/OAuth; provide OSS execution authority/broker path |
| Okta for AI Agents | agent identity/lifecycle, agent-to-agent connection policy | Consume enterprise identity; own execution boundary |
| Microsoft Entra Agent ID | enterprise agent identity, sponsors/owners/lifecycle | Consume enterprise identity; avoid human-IAM duplication |

## Open-source prior art

| Project | Study for | Differentiation risk |
| --- | --- | --- |
| agentgateway | Rust MCP/A2A/LLM gateway, auth, policy, observability | "Rust MCP security gateway" alone is not a moat |
| Alibaba open-agent-auth | operation authorization, OAuth/OIDC/WIMSE, request isolation, audit | Per-operation auth alone is not unique |
| Better Auth Agent Auth Protocol | agent keys, capabilities, approval, discovery/lifecycle | Agent identity alone is crowded |
| AgentAuth | HITL, signed calls, short-lived grants, audit | Signed actions/approvals alone are insufficient differentiation |

## Tarif replacement doctrine

A competitor capability can be marked one of:

- `NOT_IMPLEMENTED`
- `PARTIAL`
- `QUALIFIED`
- `REPLACEABLE_WITHIN_BOUNDARY`

`REPLACEABLE_WITHIN_BOUNDARY` requires a reproducible migration or integration scenario, exact version of the compared product, documented feature boundary, and evidence that Tarif satisfies that scenario without hidden paid infrastructure.

Do not use a generic "replaces Okta/Auth0/Descope" claim. Tarif is intended to replace the **agent-specific execution-authority layer** where qualified, not unrelated human IAM functionality.

## Economic hypothesis

Tarif's open-source advantage is not merely a lower price. The intended differentiators are:

- unlimited self-hosted agents and authorization decisions;
- no security-feature paywall;
- local/offline developer path;
- open standards and portable policy adapters;
- exact action/parameter enforcement;
- honest mediation coverage;
- explainable proof/evidence;
- eventual data-flow controls if validated by research.

This hypothesis must be tested against real deployment and competitor evidence before becoming marketing copy.
