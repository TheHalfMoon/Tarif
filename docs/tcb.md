# Tarif Trusted Computing Base Contract — v0.1 Target

This document defines logical trust roles for the first implementation program. It intentionally does not pre-commit to crate boundaries or dependencies.

## TCB principle

Keep the correctness-sensitive trusted computing base as small, deterministic, inspectable, and replaceable as practical.

The model is outside the TCB.

## Logical components expected to enter the TCB

### 1. Request intake and canonicalization

Responsibilities:

- parse supported MCP request forms;
- reject malformed/ambiguous consequential requests;
- normalize security-relevant action/resource/parameter fields deterministically;
- produce a stable representation suitable for decision/evidence binding.

Failure mode: fail closed for consequential actions.

### 2. Policy enforcement point (PEP)

Responsibilities:

- ensure supported mediated requests pass through authorization before forwarding;
- prevent `ALLOW`/`APPROVAL_REQUIRED`/`DENY` confusion;
- avoid forwarding a request that has changed after authorization.

### 3. Policy decision path

May be an embedded engine or an external standards-native PDP adapter, but the trusted contract must define:

- exact decision input;
- exact decision output;
- failure behavior;
- provenance of policy inputs;
- no implicit allow on unknown/unavailable policy.

AuthZEN compatibility should be preferred for external PDP integration where suitable.

### 4. Approval binding and revalidation

Responsibilities:

- render trusted action fields for human approval;
- bind approval to exact relevant request/resource state;
- enforce expiry/single-use/context constraints selected by Specification 006;
- revalidate relevant mutable state immediately before execution where required.

### 5. Evidence binding

Responsibilities:

- bind observed decision/effect metadata to the exact normalized request and implementation/runtime revision facts that are actually known;
- never reinterpret a cryptographic digest as semantic correctness.

## Conditionally trusted external systems

Tarif may rely on external systems under explicit deployment assumptions:

- OIDC identity provider;
- SPIFFE/SPIRE or cloud workload identity;
- AuthZEN/Cedar/OPA/Cerbos-like PDP;
- Vault/OpenBao/cloud credential source;
- OS keychain;
- system TLS implementation;
- MCP SDK/runtime library.

Each integration must document what claim is inherited versus independently verified.

## Explicitly outside the TCB

- LLM/model output;
- agent reasoning or self-reported intent;
- retrieved content;
- remote tool descriptions;
- remote tool output;
- marketing metadata;
- automated reviewer summaries;
- unverified identity strings supplied by an agent.

## Dependency rule

A dependency that enters the TCB requires a documented capability, security, maintenance, provenance/license, and portability justification. "Popular" or "convenient" is not sufficient.

## Repository-control prerequisite before R3 implementation

Before the first R3 implementation PR is merged, `main` must have an organization/repository-controlled enforcement path that prevents direct unqualified R3 merges. At minimum, the repository should establish:

- pull-request-based change control for `main`;
- required deterministic CI appropriate to the active language/toolchain;
- expected-head/branch freshness or equivalent stale-head protection where practical;
- a documented independent-review requirement for R3 changes;
- no bypass that a repository-controlled change can silently use to weaken the mandatory baseline without review.

The exact GitHub ruleset/workflow design is a separately bounded governance task selected before R3 implementation, not an excuse to broaden Specification 001 into runtime code.
