# Tarif Current Program State

## Closed canonical work

- `000-foundation` — `CLOSED_CANONICAL` at merge `7ab1fdf2efa22e1485e49c2e7e087808c8bef6ac`
- `001-threat-model-tcb-coverage` — `CLOSED_CANONICAL` at merge `13780df36bd86ec172524eed6545860c90b20d89`
- `001a-r3-qualification-baseline` — `CLOSED_CANONICAL` at merge `89a03d76a99d16b3ce35a7ba8699219a3dbcf7dc`

## Current specification

`002-action-ir-canonicalization`

Status: `ACTIVE_R3_IMPLEMENTATION_BRANCH`
Risk: `R3`
Canonical shaping merge: `b1b3cecc7c2de32a4ecdba02a6bb752ae7a050c5`
Implementation PR: `#10`
Implementation branch: `feat/002-action-ir-canonicalization`

## Current product authority

Authorized:

- minimal Rust workspace and `tarif-core` required for this specification;
- strict `tarif.action/v1` representation of the supported MCP 2026-07-28 baseline `tools/call` profile;
- duplicate-safe strict JSON ingestion;
- RFC 8785 JCS canonical bytes through the pinned/qualified dependency wrapper;
- exact case-sensitive tool-name profile and omitted/present arguments distinction;
- supported server-visible MCP envelope context bound as untrusted execution context, never authenticated identity;
- fail-closed rejection of unsupported MRTR/task/unknown execution-affecting metadata;
- Action IR error taxonomy and adversarial tests;
- existing GitHub Actions check context `qualification` with committed-lock verification and pinned Rust fmt/clippy/test/doc-test checks.

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

Issue #3 remains open. Repository-side `qualification` is evidence, not mandatory external enforcement.

The Specification 002 implementation branch may be built, tested, reviewed, and repaired. It **must not merge into canonical `main` while Issue #3 remains unresolved**.

## Current qualification stage

The dependency lock is committed and reproducible under the pinned Rust 1.98.0 toolchain. Bootstrap CI exposed and corrected the initial lock-transfer and formatting defects; that history is recorded in `docs/evidence/spec-002-r3-implementation.md`.

The qualification workflow is now read-only and checks the immutable PR head SHA. Branch implementation is not verified until one final exact head passes all locked qualification steps and substantive semantic review is reconciled. An ancestor PASS does not qualify a successor head.

## Next eligibility

Specification 003 (`Decision Core & Default-Deny Policy`) is not implementation-authorized until Specification 002 is canonically merged, post-merge qualified, and closed.
