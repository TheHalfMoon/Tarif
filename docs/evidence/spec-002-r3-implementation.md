# Specification 002 R3 Implementation Evidence

Status: `ACTIVE_CANDIDATE`

This document records branch implementation evidence for Specification 002. It does not claim canonical completion or merge eligibility.

## Canonical base

- canonical shaping merge: `b1b3cecc7c2de32a4ecdba02a6bb752ae7a050c5`
- implementation PR: #10 — `feat(002): implement Action IR canonicalization`
- implementation branch: `feat/002-action-ir-canonicalization`
- mandatory merge blocker: Issue #3

Specification 002 implementation is R3. Branch implementation work is authorized, but canonical merge remains forbidden while Issue #3 is open and until the remaining review/qualification gates are satisfied.

## Implemented branch scope

The branch candidate contains only the Specification 002 implementation surface:

- minimal Rust workspace;
- Rust toolchain pinned to 1.98.0;
- `tarif-core` crate only;
- strict recursive JSON ingestion with duplicate-key rejection before map collapse;
- MCP 2026-07-28 baseline `tools/call` normalization into `tarif.action/v1`;
- explicit omitted-vs-present arguments state;
- supported MCP envelope context bound as untrusted execution context rather than identity;
- trace-only metadata excluded from action authority;
- unknown or unsupported execution-affecting state rejected fail closed;
- RFC 8785 JCS canonical bytes through pinned `serde_json_canonicalizer = 0.3.2`;
- stable fail-closed error categories;
- positive, negative, and adversarial tests;
- repository `qualification` extended with locked Rust fmt/clippy/test checks.

The branch does not implement policy decisions, MCP proxying, identity authentication, credentials, approvals, action digests, A2A/HTTP normalization, or production-security claims.

## Bootstrap failures and corrections

Early CI outcomes are retained as evidence rather than rewritten as success:

1. Head `2a988ce26151c1e6c8a1a3932be11c73f323a34b`
   - toolchain installation: PASS;
   - initial dependency-lock generation: PASS;
   - formatting: FAIL;
   - clippy: NOT RUN;
   - tests: NOT RUN.

2. A manually transferred `Cargo.lock` contained one incorrect checksum character for `serde 1.0.228`. Locked comparison correctly failed. The committed lock was replaced with the exact Cargo-generated lock; subsequent regenerated-lock comparison and `cargo metadata --locked` passed. This was a repository-input correction, not a dependency change.

3. Head `6589a379fc3315bab7ca0759c888e0bb3772b4c8`
   - committed dependency-lock comparison: PASS;
   - `cargo metadata --locked`: PASS;
   - rustfmt candidate generation: PASS;
   - committed formatting comparison: FAIL;
   - clippy: NOT RUN;
   - tests: NOT RUN.

4. Head `9b69c65c308ae21ac3d0ebbe6d3ecc667e443358`
   - dependency-lock verification: PASS;
   - pinned Rust 1.98.0 rustfmt: PASS;
   - `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS;
   - `cargo test --workspace --all-targets --locked`: PASS;
   - the workflow then committed exact rustfmt output as `b0d34f52bb93bf5d52ee1f3ac90d6ccfc6fe427b`.

The successful run on `9b69c65...` is useful bootstrap evidence but is not exact-head qualification for its successor because the workflow changed the branch head after running the checks.

## Final qualification posture

The workflow has now been returned to a read-only design:

- `contents: read` only;
- checkout is pinned to the immutable pull-request head SHA for PR events;
- `Cargo.lock` is committed and verified with `cargo metadata --locked`;
- formatting is verification-only: `cargo fmt --all -- --check`;
- clippy uses `-D warnings` and `--locked`;
- tests use `--locked`;
- doc tests are executed separately;
- no qualification step writes back to the implementation branch.

Final R3 qualification must be observed on one exact immutable implementation head after all implementation/evidence edits are complete. Historical success on an ancestor is not sufficient.

## Remaining gates

Before Specification 002 may merge canonically:

- exact final head `qualification = success`;
- substantive semantic review of that exact implementation head;
- all substantive review findings reconciled;
- zero unresolved review threads;
- Issue #3 closed from live evidence that mandatory branch/ruleset enforcement exists;
- current-`main` reconciliation and exact-head requalification if `main` moved;
- guarded expected-head merge;
- post-merge `qualification = success`;
- canonical reread and Specification 002 closeout.

Until those gates are satisfied, status remains `ACTIVE_CANDIDATE`, not `PASS`, `MERGE_READY`, or `CLOSED_CANONICAL`.
