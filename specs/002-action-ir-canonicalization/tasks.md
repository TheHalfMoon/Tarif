# Specification 002 Tasks

Task completion is exact-evidence sensitive. Branch implementation completion is not canonical merge completion.

## Shaping

- [x] T002-01 Re-read canonical post-001A state and preserve Issue #3 blocker.
- [x] T002-02 Research MCP 2026-07-28 action/retry/meta semantics.
- [x] T002-03 Research RFC 8785 JCS and Rust implementation candidates.
- [x] T002-04 Define the bounded baseline MCP `tools/call` Action IR.
- [x] T002-05 Record ADR-0001 and adversarial corpus.
- [x] T002-06 Verify exact shaping diff is documentation/evidence/specification only.
- [x] T002-07 Open shaping PR #9 and require exact-head `qualification` success.
- [x] T002-08 Resolve substantive shaping-review findings.
- [x] T002-09 Guarded merge exact qualified shaping head `dcc991af8b4793f89e25978868152f7ef95544f4` as `b1b3cecc7c2de32a4ecdba02a6bb752ae7a050c5`.
- [x] T002-10 Re-read canonical shaping merge; post-merge `qualification` succeeded.

## R3 implementation

- [x] T002-11 Establish minimal Rust workspace/toolchain candidate pinned to Rust 1.98.0.
- [x] T002-12 Implement strict duplicate-safe JSON ingestion candidate.
- [x] T002-13 Implement Action IR types and MCP baseline normalization candidate.
- [x] T002-14 Implement RFC 8785 canonical bytes through pinned `serde_json_canonicalizer` 0.3.2 candidate.
- [x] T002-15 Implement fail-closed unsupported-state/error taxonomy candidate.
- [x] T002-16 Add positive/negative/adversarial corpus candidate.
- [x] T002-17 Extend `qualification` under the same check context with committed-lock verification, read-only pinned Rust fmt/clippy/test/doc-test checks, and exact PR-head checkout.
- [ ] T002-18 Execute focused and full exact-head R3 verification on the final immutable implementation head.
- [ ] T002-19 Obtain substantive semantic review of the final implementation head and reconcile all threads.
- [x] T002-20 Open and maintain bounded draft implementation PR #10; do not merge while Issue #3 is open.

## Canonical merge and closeout

- [ ] T002-21 Observe Issue #3 closed from required live external enforcement evidence.
- [ ] T002-22 Requalify exact implementation head after current-main reconciliation.
- [ ] T002-23 Guarded expected-head merge with required independent review and zero unresolved threads.
- [ ] T002-24 Require canonical post-merge `qualification` success.
- [ ] T002-25 Re-read canonical `main` and close Specification 002.

Specification 003 is not implementation-authorized before T002-25.
