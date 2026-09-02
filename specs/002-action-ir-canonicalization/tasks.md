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
- [x] T002-13 Implement sealed Action IR types and MCP baseline normalization candidate.
- [x] T002-14 Implement RFC 8785 canonical bytes through pinned `serde_json_canonicalizer` 0.3.2 candidate.
- [x] T002-15 Implement fail-closed unsupported-state/error taxonomy candidate.
- [x] T002-16 Add positive/negative/adversarial corpus candidate, including serialized Action IR wire-shape regressions.
- [x] T002-17 Harden `qualification` under the same check context with committed-lock verification, least-privilege credential-free exact-head checkout, pinned Rust fmt/clippy/test/doc-test checks, synthetic integrated-candidate checkout, exact parent ancestry proof, and the repeated locked Rust suite on the integrated candidate.
- [ ] T002-18 Execute focused and full exact-head R3 verification on the final immutable implementation head. Latest fully green code head: `d63a172e0506267188045d3c2fc750d40c2a175a`; the evidence-reconciliation successor must requalify before this task closes.
- [ ] T002-19 Obtain substantive independent semantic review/approval of the final implementation head and reconcile all findings/threads. Author-side review and automated summaries do not by themselves close this gate.
- [x] T002-20 Maintain a bounded implementation PR without merge: draft PR #10 was closed unmerged as superseded; active non-draft PR #11 preserves the implementation branch and remains blocked by Issue #3.

## Canonical merge and closeout

- [ ] T002-21 Observe Issue #3 closed from required live external enforcement evidence and bounded negative/positive enforcement-path proof.
- [ ] T002-22 Requalify exact implementation head after current-main reconciliation.
- [ ] T002-23 Guarded expected-head merge with required independent approval and zero unresolved threads.
- [ ] T002-24 Require canonical post-merge `qualification` success.
- [ ] T002-25 Re-read canonical `main` and close Specification 002.

Specification 003 is not implementation-authorized before T002-25.
