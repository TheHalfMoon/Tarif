# Tarif Product Thesis

## Category

Tarif is **open-source authority infrastructure for AI agents**.

It is not primarily an identity provider, secrets manager, policy language, agent framework, MCP catalog, or SIEM. It composes those systems around a runtime authority boundary.

## User problem

Agentic systems increasingly possess multiple individually valid permissions. Traditional authentication and coarse authorization do not fully answer whether one specific consequential action is appropriate at execution time.

Tarif focuses on the normalized question:

```text
WHO
wants to do WHAT
on WHICH RESOURCE
with WHICH PARAMETERS
on behalf of WHOM
under WHICH AUTHORITY
through WHICH DELEGATION
using WHICH RUNTIME CONTEXT
at WHICH MOMENT?
```

The initial decision surface is deliberately small:

```text
ALLOW
DENY
APPROVAL_REQUIRED
```

## Product promise

Tarif aims to provide one self-hostable layer that can eventually combine:

- agent/workload identity context;
- least-privilege runtime authority;
- exact-action and parameter enforcement;
- approval binding and stale-state revalidation;
- credential isolation/brokering;
- delegation controls;
- explainable evidence;
- enforcement-coverage inspection;
- standards-native interoperability;
- information-flow protection where research proves it practical.

## Open-source promise

Tarif's core security capability should not be metered by agent count, policy count, environment count, audit retention, or a production-use license. Managed hosting or support may be viable later, but the self-hosted open-source product must remain genuinely production-capable.

## Initial wedge

The first product wedge is intentionally narrower than the long-term thesis:

> A local-first deterministic authorization gate for supported MCP agent actions, with exact-parameter decisions, approval revalidation, explainable evidence, and honest mediation coverage.

This wedge is selected because it is understandable to individual developers, exercises the core authority thesis, and can be independently verified without requiring Tarif to become a general identity provider or cloud platform.

## Non-goals

Tarif does not initially aim to:

- replace Okta, Entra, Keycloak, Authentik, or other human identity providers;
- become a generic secrets store replacing Vault/OpenBao/cloud secret managers;
- create a new OAuth dialect;
- create a proprietary policy language when existing PDPs or AuthZEN are sufficient;
- create a proprietary mission/token format while relevant standards are actively evolving;
- claim whole-agent sandboxing when only selected transports are mediated;
- ship enterprise dashboards before the developer execution boundary is proven.

## North star

```text
Any agent.
Any model.
Any tool.
Any identity provider.

One authority layer.
No action beyond authority.
No hidden security paywall.
No claim beyond observed enforcement.
```
