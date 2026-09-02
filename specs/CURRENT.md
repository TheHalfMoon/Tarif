# Tarif Current Program State

## Closed canonical work

- `000-foundation` — `CLOSED_CANONICAL` at merge `7ab1fdf2efa22e1485e49c2e7e087808c8bef6ac`
- `001-threat-model-tcb-coverage` — `CLOSED_CANONICAL` at merge `13780df36bd86ec172524eed6545860c90b20d89`
- `001a-r3-qualification-baseline` — `CLOSED_CANONICAL` at merge `89a03d76a99d16b3ce35a7ba8699219a3dbcf7dc`

## Current specification

`002-action-ir-canonicalization`

Status: `ACTIVE_R3_IMPLEMENTATION_AFTER_SHAPING_MERGE`
Risk: `R3`

This candidate state becomes canonical only when the Specification 002 shaping PR is merged. Until then, canonical `main` remains authoritative.

## Product authority after shaping merge

Authorized:

- minimal Rust workspace and `tarif-core` required for this specification;
- strict `tarif.action/v1` representation of the supported MCP 2026-07-28 baseline `tools/call` profile;
- duplicate-safe strict JSON ingestion;
- RFC 8785 JCS canonical bytes through a qualified dependency/wrapper;
- explicit case-sensitive tool-name profile and omitted/present arguments distinction;
- fail-closed rejection of unsupported MRTR/task/unknown execution-affecting metadata;
- Action IR error taxonomy and adversarial tests;
- extension of existing GitHub Actions check context `qualification` with Rust fmt/clippy/test.

Not authorized:

- MCP proxy/network forwarding;
- AuthZEN/PDP policy decisions;
- identity authentication;
- credential brokering;
- approval/revalidation;
- cryptographic action digest/evidence chain;
- MRTR/task support beyond rejection;
- A2A/HTTP normalization;
- information-flow enforcement;
- release publication;
- production-security or paid-product replacement claims.

## Mandatory R3 merge blocker

Issue #3 remains open. Live post-001A GitHub truth showed `main` still unprotected and required status-check enforcement off even though repository-side `qualification` runs succeed.

Specification 002 implementation may proceed on a bounded branch after the shaping merge, but **must not merge into canonical `main` while Issue #3 remains unresolved**.

## Next eligibility

Specification 003 (`Decision Core & Default-Deny Policy`) is not implementation-authorized until Specification 002 is canonically merged, post-merge qualified, and closed.
