<div align="center">

# Tarif

**Open-source authority infrastructure for AI agents.**

**Let agents act. Never beyond their authority.**

Tarif is being founded as a local-first, self-hostable authority runtime for controlling consequential AI-agent actions across tool and service boundaries.

> The model may propose. Deterministic policy authorizes. Runtime enforcement proves what happened.

</div>

---

## Status

**Founding phase — no production-security claim yet.**

Tarif is currently establishing its threat model, trusted computing base, standards strategy, action model, proof requirements, and first bounded implementation program. Roadmap entries are directions, not implementation authority.

## Product thesis

Identity answers **who an agent is**. Tarif answers a harder runtime question:

> May this exact agent perform this exact action, on this exact resource, with these exact parameters, on behalf of this principal, under the current authority and runtime state?

Tarif aims to make that control free, self-hostable, explainable, and standards-native.

## Founding invariants

- **Deterministic authority** — models may propose; they do not grant themselves authority.
- **Default deny** — unknown consequential actions fail closed.
- **Honest mediation** — Tarif never claims control over a path it cannot enforce.
- **Secret isolation** — downstream credentials should not need to be exposed to model context.
- **Standards before invention** — compose open standards instead of creating proprietary identity, OAuth, policy, or mission protocols without demonstrated need.
- **Evidence before claims** — security, compatibility, performance, and replacement claims require reproducible evidence.
- **Security is not a paywall** — production security capability belongs in the open-source core.

## Initial program

```text
000 Project foundation and constitution
  -> 001 Threat model, TCB, and coverage contract
  -> 002 Action IR and deterministic canonicalization
  -> 003 Decision core and default-deny policy
  -> 004 Local MCP enforcement gate
  -> 005 Runtime evidence and `tarif why`
  -> 006 Approval binding and execution revalidation
  -> 007 Coverage inspection and `tarif doctor`
  -> 008 v0.1 developer-experience qualification
  -> 009 v0.1 release closeout
```

Only the active specification grants implementation authority. Future areas such as A2A, HTTP mediation, delegation graphs, credential brokering, and information-flow controls remain evidence-shaped program directions until explicitly selected and shaped.

## Engineering method

Tarif uses two complementary disciplines:

- **SpecGrain** for bounded, dependency-ordered implementation authority.
- **Diffcipline** for exact-change, risk-scaled proof before work is called done.

In short:

> Authority before action. Proof after action.

## Open-source direction

Tarif is intended to be usable locally and in production without a hosted Tarif account or per-agent license. The founding plan selects Apache-2.0 as the project license, subject to the explicit Specification 000 closeout checks recorded in the repository.

## Research canon

Start with:

- [`docs/research/SOURCES.md`](docs/research/SOURCES.md)
- [`docs/research/standards-map.md`](docs/research/standards-map.md)
- [`docs/research/competitor-matrix.md`](docs/research/competitor-matrix.md)
- [`docs/research/threat-source-map.md`](docs/research/threat-source-map.md)

## Canonical continuation

Read [`AGENTS.md`](AGENTS.md), [`CONSTITUTION.md`](CONSTITUTION.md), [`specs/CURRENT.md`](specs/CURRENT.md), and [`docs/execution-master-plan.md`](docs/execution-master-plan.md) before changing product behavior.
