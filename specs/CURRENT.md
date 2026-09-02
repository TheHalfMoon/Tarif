# Tarif Current Program State

## Closed canonical work

- `000-foundation` — `CLOSED_CANONICAL` at merge `7ab1fdf2efa22e1485e49c2e7e087808c8bef6ac`

## Current specification

`001-threat-model-tcb-coverage`

Status: `ACTIVE_DOCUMENTATION`
Risk: `R2`

## Current authority

Authorized now:

- reconcile Specification 000 closeout evidence;
- define the v0.1 threat model and deployment assumptions;
- define logical trusted-computing-base responsibilities;
- define enforcement coverage states and narrow MCP claim vocabulary;
- record known residual/bypass risks;
- record repository governance prerequisites before future R3 implementation.

Not authorized now:

- Rust/runtime product implementation;
- MCP gateway/proxy implementation;
- Action IR or canonicalization code;
- policy engine code;
- approval/runtime code;
- credential brokering;
- GitHub workflow/ruleset implementation as part of Spec 001;
- A2A/HTTP mediation;
- information-flow enforcement;
- production-security or replacement claims.

## Next eligibility

Specification 002 (`Action IR & Deterministic Canonicalization`) may be shaped only after Specification 001 is merged, canonical `main` is re-read, and its documentation contract is closed.

Before any R3 implementation PR may merge, repository-controlled PR/CI/review enforcement must be established in a separately authorized governance unit.
