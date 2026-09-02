# Tarif Current Program State

## Closed canonical work

- `000-foundation` — `CLOSED_CANONICAL` at merge `7ab1fdf2efa22e1485e49c2e7e087808c8bef6ac`
- `001-threat-model-tcb-coverage` — `CLOSED_CANONICAL` at merge `13780df36bd86ec172524eed6545860c90b20d89`

## Current governance specification

`001a-r3-qualification-baseline`

Status: `ACTIVE_GOVERNANCE`
Risk: `R2`
Canonical issue: #4

## Current authority

Authorized now:

- record Specification 001 closeout truth;
- establish the repository-side `Tarif Qualification` workflow with observed check-run context `qualification`;
- document and preserve the distinction between workflow presence and mandatory external enforcement;
- reconcile governance state without adding R3 product behavior.

Not authorized by Specification 001A:

- Action IR/canonicalization implementation;
- MCP gateway/proxy implementation;
- policy engine, approval, credential, identity, crypto, or evidence-integrity product code;
- branch/ruleset protection claims not proven by live GitHub state;
- production-security or paid-product replacement claims.

## Mandatory R3 merge blocker

Issue #3 remains open. Live GitHub truth at selection showed `main` unprotected, required status checks disabled, and no repository rulesets.

A repository-side workflow is not sufficient to close Issue #3. Before any R3 implementation PR may merge, live external branch/ruleset evidence and the bounded negative/positive merge tests required by Issue #3 must exist.

The first exact-head execution of the new workflow on PR #8 reported GitHub check-run context `qualification`; that observed context is the current candidate for future required-check configuration.

R3 work may be shaped and implemented on bounded branches once its own specification is active, but must not merge while Issue #3 remains unresolved.

## Next eligibility

Specification 002 (`Action IR & Deterministic Canonicalization`) may be shaped after Specification 001A is canonically merged and re-read. Its implementation will be R3 and therefore merge-blocked by Issue #3 until external enforcement is proven.
