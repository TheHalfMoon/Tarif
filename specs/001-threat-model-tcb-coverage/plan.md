# Specification 001 Plan

## Method

Controlled documentation flow because the artifacts constrain later security implementation.

## Sequence

1. Reconcile Specification 000 closeout from live GitHub truth.
2. Define assets, adversaries, external assumptions, and initial local deployment boundary.
3. Translate the founding threat-source map into a canonical threat register without inventing mitigations.
4. Define the logical TCB and explicitly exclude model self-report from authority.
5. Define enforcement coverage states and narrow v0.1 MCP claim vocabulary.
6. Record repository protection/required-CI gap as a prerequisite to R3 implementation, without implementing CI in this specification.
7. Perform exact documentation-scope comparison and semantic review.
8. Merge only if the exact head remains within Specification 001 authority.
9. Re-read canonical `main`; only then may Specification 002 shaping begin.

## Expected change surface

```text
docs/evidence/spec-000-closeout.md
docs/threat-model.md
docs/tcb.md
docs/coverage-contract.md
specs/000-foundation/tasks.md
specs/001-threat-model-tcb-coverage/**
specs/CURRENT.md
```

No runtime/product source or workflow mutation is authorized.
