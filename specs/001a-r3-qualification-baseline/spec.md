# Specification 001A — R3 Qualification Workflow Baseline

Status: `ACTIVE_GOVERNANCE`
Risk: `R2`
Issue: #4

## Outcome

Tarif has a stable repository-side qualification workflow/check contract and an explicit record of the still-missing external GitHub enforcement boundary, without claiming that workflow presence alone makes R3 merges mandatory.

## Scope in

- Specification 001 canonical closeout evidence;
- one GitHub Actions qualification workflow with a stable workflow/job name;
- exact-diff hygiene and canonical-governance-file checks;
- documentation of the distinction between workflow presence and mandatory branch/ruleset enforcement;
- current-program reconciliation.

## Scope out

- branch-protection/ruleset mutation that the connected GitHub tool cannot perform;
- Action IR/canonicalization implementation;
- Rust workspace or runtime product code;
- MCP/policy/credential/approval implementation;
- production-security or replacement claims.

## Acceptance

1. Specification 001 is recorded `CLOSED_CANONICAL` at merge `13780df36bd86ec172524eed6545860c90b20d89`.
2. `.github/workflows/qualification.yml` exposes stable workflow `Tarif Qualification` and job `qualification`.
3. The workflow fails on diff whitespace errors and missing canonical governance/security files.
4. Documentation explicitly states that workflow presence is not mandatory enforcement.
5. Issue #3 remains open unless live external enforcement evidence satisfies its closure contract.
6. No R3 product implementation enters this specification.

## Claim boundary

Allowed after completion:

> Tarif has a repository-side qualification workflow baseline that can be made mandatory by GitHub branch/ruleset enforcement.

Not allowed:

> Tarif's R3 merge policy is enforced on `main`.

The latter remains false until Issue #3 is closed from live evidence.
