# Specification 001 — Threat Model, TCB & Coverage Contract

Status: `ACTIVE_DOCUMENTATION`
Risk: `R2` — security architecture/claim boundary with no runtime implementation.

## Outcome

Tarif has a canonical v0.1 threat model, trusted computing base contract, and enforcement-coverage contract that future R3 implementation must satisfy before making security claims.

## Scope in

- assets and trust boundaries;
- adversaries/untrusted inputs;
- local-v0.1 deployment assumptions;
- threat register and required future evidence;
- logical TCB roles without premature crate/dependency commitment;
- explicit external trust assumptions;
- initial MCP-only mediation coverage contract;
- known bypass/residual-risk vocabulary;
- repository-control prerequisite before first R3 implementation merge;
- Specification 000 closeout reconciliation.

## Scope out

- Rust/product implementation;
- MCP proxy code;
- canonicalizer code;
- policy engine code;
- approval/runtime code;
- credential brokering;
- CI/workflow implementation itself;
- GitHub ruleset mutation itself;
- A2A/HTTP mediation;
- information-flow enforcement;
- claims that any threat is mitigated by product code.

## Acceptance

1. Threat model names assets, adversaries, deployment assumptions, and at least the founding threat corpus.
2. No threat is marked mitigated without implementation evidence.
3. TCB explicitly excludes model output and remote content from authorization authority.
4. TCB defines logical request canonicalization, PEP, policy decision, approval/revalidation, and evidence-binding responsibilities without premature implementation topology.
5. Coverage contract distinguishes `MEDIATED`, `PARTIALLY_MEDIATED`, `UNMEDIATED`, and `UNKNOWN`.
6. Initial security claim is bounded to supported MCP `tools/call` requests that traverse Tarif.
7. Shell/network/HTTP/A2A/credential-bypass surfaces are explicitly non-covered until later evidence.
8. Repository governance records that `main` is currently unprotected and requires a mandatory enforcement path before first R3 implementation merge.
9. Specification 000 closeout evidence is canonical and truthful.
10. Exact PR diff remains documentation/governance only.

## Claim boundary after completion

Allowed:

> Tarif has a defined security/threat and mediation contract for its planned v0.1 implementation.

Not allowed:

- any claim that the contract itself mitigates attacks;
- any production-ready claim;
- any statement that Tarif already mediates MCP;
- any paid-product replacement claim.
