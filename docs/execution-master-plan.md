# Tarif Execution Master Plan

This is the durable continuation contract for Tarif. `specs/CURRENT.md` owns the active frontier. Live GitHub/repository truth overrides stale narrative.

## Product objective

> Make consequential agent execution explicitly authorized, runtime-enforced, explainable, and self-hostable without proprietary security paywalls.

## Canonical reading order

1. `AGENTS.md`
2. `CONSTITUTION.md`
3. `specs/CURRENT.md`
4. this file
5. active `spec.md`, `plan.md`, `tasks.md`
6. referenced research, standards, ADRs, implementation, tests, and evidence

## Method

Tarif combines:

- **SpecGrain-style progressive refinement** — broad later outcomes, shaped next work, bounded current Grain-like units;
- **Diffcipline-style proof before done** — exact diff, risk-scaled evidence, `NOT RUN != PASS`, repository truth over self-report.

The resulting operating rule is:

> Authority before action. Proof after action.

## Initial dependency sequence

```text
000 foundation
 -> 001 threat model / TCB / coverage
 -> 002 Action IR / canonicalization
 -> 003 decision core
 -> 004 MCP gate
 -> 005 evidence / why
 -> 006 approval binding / revalidation
 -> 007 doctor / coverage
 -> 008 v0.1 qualification
 -> 009 release closeout
```

## Cross-spec execution rules

1. Only the active bounded specification authorizes product mutation.
2. Do not let future roadmap items widen current scope.
3. Prefer existing standards/native capabilities before new dependencies or proprietary protocols.
4. Every implementation change must identify its security claim boundary.
5. R3 changes require stronger negative/adversarial evidence even when small.
6. No executor, model, reviewer bot, or CI summary is proof by itself.
7. Verify exact branch/head, diff scope, checks, substantive reviews, comments/threads, and mergeability before merge where GitHub supports those observations.
8. Unavailable/skipped/neutral automated reviewers are not approvals.
9. Prefer expected-head/guarded merge behavior where supported.
10. Re-read canonical `main` after merge and require post-merge verification selected by the active risk profile.
11. Preserve blockers and residual risks rather than manufacturing completion.
12. Research and external code require provenance/license review.
13. Stable standards and unstable drafts must remain explicitly distinguished.
14. Never claim whole-agent protection from partial mediation.

## Foundation closeout condition

Specification 000 may close only when:

- repository founding documents are internally consistent;
- source map distinguishes stable standards, drafts, prior art, and competitors;
- Apache-2.0 license presence and README statements agree;
- initial program ordering is canonical;
- no runtime implementation is smuggled into the foundation diff;
- live PR diff matches the declared documentation/governance scope;
- required repository checks for this founding state, if any, are observed rather than invented;
- substantive review state is recorded honestly;
- canonical post-merge `main` is re-read.

After 000 closes, only Specification 001 is eligible to be shaped. Do not jump directly to MCP implementation.

## Success philosophy

Tarif does not optimize for number of features or number of pre-written tasks. It optimizes for:

- first useful mediated action;
- authorization correctness;
- honest enforcement coverage;
- reproducible proof;
- low developer friction;
- standards interoperability;
- open-source deployability;
- evidence-backed replacement of paid agent-specific security layers.
