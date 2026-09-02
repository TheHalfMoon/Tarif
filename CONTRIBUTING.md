# Contributing to Tarif

Tarif welcomes contributions that improve correctness, interoperability, security, developer experience, evidence quality, or clarity without weakening the project's trust boundaries.

## Read before changing behavior

Read, in order:

1. `AGENTS.md`
2. `CONSTITUTION.md`
3. `specs/CURRENT.md`
4. `docs/execution-master-plan.md`
5. the active specification and referenced research/standards

A roadmap item or interesting idea is not implementation authority by itself.

## Contribution principles

- Keep changes bounded and independently reviewable.
- Every changed path should trace to an active outcome, acceptance criterion, defect, or explicitly authorized supporting change.
- Prefer standards and existing project/native capabilities before new dependencies or proprietary protocols.
- Do not combine unrelated refactors or cleanup with a security-sensitive change.
- Preserve explicit non-goals and claim boundaries.
- Record external prior art and licensing/provenance where it materially influences implementation.

## Security-sensitive changes

Changes touching authorization, canonicalization, credentials, approvals, policy enforcement, identity binding, cryptography, mediation, or evidence integrity are normally `R3`.

R3 work should include relevant positive, negative, malformed-input, replay/stale-state, and adversarial tests. Property or fuzz testing should be used where it materially strengthens the proof boundary.

`NOT RUN` is not `PASS`.

## Claims

Do not add claims such as "secure", "prevents", "production-ready", "faster", or "replaces <product>" without evidence that supports the exact wording and boundary.

## Pull requests

A good PR explains:

- exact outcome;
- scope in / scope out;
- risk level and affected trust boundary;
- standards or prior-art dependencies;
- tests/evidence actually executed;
- residual risks;
- claim boundary after merge.

Unavailable or skipped automated reviewers are not approvals.
