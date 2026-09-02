# Specification 002 Plan

## Phase A — Shaping

1. Re-read canonical Specification 001A closeout and Issue #3 state.
2. Research MCP 2026-07-28 execution semantics and established JSON canonicalization.
3. Select the narrow baseline `tools/call` Action IR boundary.
4. Record ADR-0001 and the adversarial corpus.
5. Merge this documentation-only shaping unit after exact-head qualification.

## Phase B — R3 implementation on a bounded branch

1. Re-read canonical shaping merge and verify Issue #3 remains the merge blocker unless external evidence changed.
2. Establish the minimal Rust workspace and pin the supported Rust toolchain.
3. Implement strict raw JSON ingestion with duplicate-key rejection.
4. Implement `tarif.action/v1` types and MCP baseline normalization.
5. Implement JCS canonical bytes through the qualified RFC 8785 dependency/wrapper.
6. Implement explicit fail-closed handling for unsupported protocol/method/tool-name/MRTR/task/meta states.
7. Add the full positive/negative/adversarial corpus.
8. Extend the existing `qualification` workflow job with Rust fmt/clippy/test without changing its observed check context.
9. Run focused and full exact-head verification.
10. Obtain substantive semantic review and reconcile every review thread.
11. Leave the implementation PR unmerged while Issue #3 is open.

## Phase C — merge eligibility

Only after Issue #3 is closed from live external enforcement evidence:

1. rebase/update from canonical `main` without rewriting shared history;
2. re-run exact-head R3 qualification;
3. require independent substantive review and zero unresolved threads;
4. guarded expected-head merge;
5. post-merge `qualification` success;
6. canonical reread and Specification 002 closeout.

No later specification is authorized by merely completing branch implementation.
