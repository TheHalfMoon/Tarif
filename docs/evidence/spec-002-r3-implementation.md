# Specification 002 R3 Implementation Evidence

Status: `ACTIVE_CANDIDATE`

This document records branch implementation evidence for Specification 002. It does not claim canonical completion or merge eligibility.

## Canonical base

- canonical shaping merge: `b1b3cecc7c2de32a4ecdba02a6bb752ae7a050c5`
- active implementation PR: #11 — `feat(002): implement Action IR canonicalization`
- superseded draft PR: #10 — closed without merge solely to replace the draft review surface with non-draft PR #11 on the same implementation branch
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
- mandatory per-request `_meta` enforcement for `io.modelcontextprotocol/protocolVersion` and `io.modelcontextprotocol/clientCapabilities`;
- required client capabilities preserved as untrusted canonical `mcp_context`, with optional supported `clientInfo`/`logLevel` preserved when present;
- sealed validated `Action` construction boundary so external callers cannot directly construct or deserialize an arbitrary canonicalizable Action and bypass strict numeric or required-context ingestion;
- explicit omitted-vs-present arguments state with fail-closed serialized wire-shape validation;
- rejection of unknown serialized argument fields before typed deserialization;
- trace-only metadata excluded from action authority;
- unknown or unsupported execution-affecting state rejected fail closed;
- RFC 8785 JCS canonical bytes through pinned `serde_json_canonicalizer = 0.3.2`;
- stable fail-closed error categories;
- positive, negative, and adversarial tests;
- repository `qualification` extended with read-only exact-head and integrated-candidate locked Rust verification.

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

## Semantic/security findings and repairs

Substantive findings are retained as evidence rather than hidden behind later green runs:

1. **Programmatic Action construction bypass.** An earlier public/deserializable `Action` surface allowed callers to supply arbitrary `serde_json::Value` numbers and then call `canonical_bytes`, bypassing the strict JCS/I-JSON numeric ingestion boundary. The branch now seals `Action` and its nested state behind private fields and read-only accessors; private wire types are used only after strict parsing. Regression coverage includes high-precision integer normalization.

2. **Serialized absent/present collapse.** A regression test demonstrated that Serde could accept `{"state":"absent","value":null}` for a unit variant and collapse it to the canonical absent form. The parser now validates the strict JSON `arguments` object before typed deserialization: absent accepts only `state`; present requires exactly `state` and `value`; unknown fields fail closed. Dedicated regressions cover absent+null, missing present value, non-object present value, and unknown fields in both states.

3. **Qualification credential boundary.** External CodeRabbit feedback identified that checkout credentials were persisted while PR-controlled Rust commands executed. Both checkout boundaries now use `persist-credentials: false`; the workflow grants only `contents: read`, and job evidence shows only Contents read / Metadata read token permissions.

4. **Integrated candidate coverage.** External CodeRabbit feedback identified that a green PR head alone did not prove the synthetic integrated merge candidate. The same `qualification` job now checks out `refs/pull/<number>/merge`, verifies parent 1 equals the exact PR base SHA and parent 2 equals the exact PR head SHA, then repeats locked metadata, formatting, clippy, tests, and doc tests.

5. **Unsupported protocol taxonomy.** Codex exact-head review on predecessor `8e8cc02eb8d0906205ccb389dc368c3cca6b45ee` identified that serialized Action IR with a non-MCP `protocol.name` returned generic `invalid_action_ir` rather than a stable typed unsupported-protocol category required by acceptance criterion 12. The branch now exposes `ActionError::UnsupportedProtocol(String)` with machine code `unsupported_protocol`; `validate_contract` emits it, and a dedicated serialized-Action regression covers `protocol.name = "a2a"`.

6. **MCP 2026-07-28 required request metadata under-validation.** Independent Cubic exact-head adjudication on predecessor `0a44608f3fee3d32843b0c8ff4afe881030f0c55` identified that the candidate accepted requests with missing `_meta`, missing `io.modelcontextprotocol/protocolVersion`, or missing `io.modelcontextprotocol/clientCapabilities`. Primary MCP 2026-07-28 schema, tools documentation, architecture, and changelog evidence confirmed the finding: the revision is stateless and every request carries protocol version and per-request client capabilities. The branch now requires `_meta`, requires both fields, validates the existing exact protocol-version match and object-shaped capabilities, always preserves required client capabilities in canonical `mcp_context`, and rejects serialized Action IR that omits that required context. Regressions cover all four missing-state paths. Specification and research text were strengthened to remove the stale optional-metadata implication without widening product authority.

Author-side semantic review is not counted as independent R3 approval. External semantic review evidence must bind to the final exact head after these repairs.

## Latest predecessor exact-head evidence

Head `0a44608f3fee3d32843b0c8ff4afe881030f0c55` passed workflow run `33655495482`, job `100332896163` before the required-metadata finding was repaired:

### Exact PR head

- immutable exact-head checkout: PASS;
- diff hygiene: PASS;
- canonical governance surface: PASS;
- pinned Rust 1.98.0: PASS;
- committed lock / `cargo metadata --locked`: PASS;
- `cargo fmt --all -- --check`: PASS;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS;
- `cargo test --workspace --all-targets --locked`: PASS;
- `cargo test --workspace --doc --locked`: PASS.

### Synthetic integrated candidate

- checkout of `refs/pull/11/merge`: PASS;
- exact base/head parent ancestry proof: PASS;
- integrated committed-lock verification: PASS;
- integrated formatting: PASS;
- integrated clippy with `-D warnings`: PASS;
- integrated tests: PASS;
- integrated doc tests: PASS.

That PASS is predecessor evidence only because the semantic finding required subsequent code/tests/spec/research/evidence changes. The final reconciliation successor must independently pass `qualification`; its exact SHA and run may be recorded in PR #11 metadata/conversation without mutating that successor head.

## External enforcement truth

Live GitHub reads on 2026-09-02 still show:

- `main` at `b1b3cecc7c2de32a4ecdba02a6bb752ae7a050c5`;
- `main` is not protected;
- required status-check enforcement is off and has no required contexts;
- repository rulesets are empty;
- PR #11 is reported mergeable even while Issue #3 remains open.

Issue #3 contains the current negative enforcement evidence and the minimum external protection/ruleset outcome required. Repository-side green CI is not a substitute for this mandatory external control.

## Final qualification posture

The workflow is read-only and least-privilege by repository-controlled design:

- `contents: read` only;
- checkout credentials are not persisted;
- the immutable PR head SHA is verified first;
- `Cargo.lock` is committed and verified with `cargo metadata --locked`;
- formatting is verification-only;
- clippy uses `-D warnings` and `--locked`;
- tests and doc tests use `--locked`;
- the synthetic integrated PR candidate is separately checked and its exact parents are attested;
- the full locked Rust suite is repeated on that integrated candidate;
- no qualification step writes back to the implementation branch.

## Remaining gates

Before Specification 002 may merge canonically:

- final current head `qualification = success` after the required-metadata repair and evidence reconciliation;
- substantive independent semantic review/approval of that exact final head;
- all substantive review findings reconciled;
- zero unresolved review threads;
- Issue #3 closed from live evidence that mandatory branch/ruleset enforcement exists and its negative/positive enforcement path is demonstrated;
- current-`main` reconciliation and exact-head requalification if `main` moved;
- guarded expected-head merge;
- post-merge `qualification = success`;
- canonical reread and Specification 002 closeout.

Until those gates are satisfied, status remains `ACTIVE_CANDIDATE`, not `MERGE_READY` or `CLOSED_CANONICAL`.
