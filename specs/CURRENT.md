# Tarif Current Program State

## Closed canonical work

- `000-foundation` — `CLOSED_CANONICAL` at merge `7ab1fdf2efa22e1485e49c2e7e087808c8bef6ac`
- `001-threat-model-tcb-coverage` — `CLOSED_CANONICAL` at merge `13780df36bd86ec172524eed6545860c90b20d89`
- `001a-r3-qualification-baseline` — `CLOSED_CANONICAL` at merge `89a03d76a99d16b3ce35a7ba8699219a3dbcf7dc`

## Current specification

`002-action-ir-canonicalization`

Status: `ACTIVE_R3_BRANCH_VERIFICATION`
Risk: `R3`
Canonical shaping merge: `b1b3cecc7c2de32a4ecdba02a6bb752ae7a050c5`
Active implementation PR: `#11`
Superseded implementation PR: `#10` (closed unmerged; replaced only to obtain a non-draft review surface)
Implementation branch: `feat/002-action-ir-canonicalization`

## Current product authority

Authorized:

- minimal Rust workspace and `tarif-core` required for this specification;
- strict `tarif.action/v1` representation of the supported MCP 2026-07-28 baseline `tools/call` profile;
- duplicate-safe strict JSON ingestion;
- required MCP 2026-07-28 per-request `_meta` enforcement for protocol version and client capabilities;
- required client capabilities plus optional supported MCP envelope context bound as untrusted execution context, never authenticated identity;
- sealed validated Action construction and serialized Action IR wire-shape/required-context validation;
- RFC 8785 JCS canonical bytes through the pinned/qualified dependency wrapper;
- exact case-sensitive tool-name profile and omitted/present arguments distinction;
- fail-closed rejection of unsupported MRTR/task/unknown execution-affecting metadata;
- stable Action IR error taxonomy, including unsupported protocol/version/method/name/state/metadata handling;
- adversarial tests for malformed, ambiguous, missing-required-context, and unsupported states;
- existing GitHub Actions check context `qualification` with least-privilege exact-head and integrated-candidate committed-lock/fmt/clippy/test/doc-test verification.

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

Issue #3 remains open. Live GitHub reads on 2026-09-02 still show `main` unprotected, no required status-check enforcement, and no repository rulesets. PR #11 is reported mergeable while Issue #3 remains open. This is direct negative evidence that mandatory R3 merge enforcement is not currently active.

Repository-side `qualification` is evidence, not mandatory external enforcement. The Specification 002 implementation branch may be built, tested, reviewed, and repaired, but it **must not merge into canonical `main` while Issue #3 remains unresolved**.

## Current qualification stage

The dependency lock is committed and reproducible under pinned Rust 1.98.0. The current workflow:

- grants only `contents: read`;
- disables checkout credential persistence;
- checks the immutable PR head;
- verifies locked metadata, formatting, clippy, tests, and doc tests;
- separately checks the synthetic integrated PR candidate;
- verifies its parents are the exact PR base/head SHAs;
- repeats the locked Rust suite on the integrated candidate;
- performs no branch write-back.

Predecessor `0a44608f3fee3d32843b0c8ff4afe881030f0c55` passed workflow run `33655495482`, job `100332896163`, but independent review then found the MCP 2026-07-28 required-request-metadata defect. The repair and this reconciliation create a successor head, so final-head qualification must be observed again. An ancestor PASS never qualifies a successor head.

## Review stage

Substantive findings repaired so far include:

- programmatic Action construction bypass;
- ambiguous serialized absent/present argument shapes;
- checkout credential persistence while executing PR-controlled code;
- missing synthetic integrated-candidate qualification;
- missing stable unsupported-protocol taxonomy for serialized Action IR;
- acceptance of MCP 2026-07-28 requests lacking required `_meta`, protocol version, or client capabilities, including the corresponding serialized Action IR required-context bypass.

These findings are retained in `docs/evidence/spec-002-r3-implementation.md`.

No author-side review or automated status/summary output is represented as independent R3 approval. The final exact successor still requires substantive independent review/approval, and every substantive thread must be reconciled before merge eligibility.

## Next eligibility

Specification 003 (`Decision Core & Default-Deny Policy`) is not implementation-authorized until Specification 002 is canonically merged, post-merge qualified, and closed.
