# Specification 000 Plan

## Method profile

Use a bounded founding/governance flow inspired by SpecGrain progressive refinement and Diffcipline proof-before-done.

## Implementation sequence

1. Bootstrap the empty repository with a minimal non-claiming README.
2. Create a bounded feature branch for the founding package.
3. Add constitution, engineering rules, product thesis, trust draft, research maps, roadmap, current-state ledger, and Apache-2.0 license.
4. Verify that all future capability language is explicitly non-authorizing.
5. Compare the exact branch to bootstrap `main` and confirm no product/runtime code exists.
6. Open a founding PR with exact scope and claim boundary.
7. Observe available checks and review systems; do not invent PASS for absent checks/reviews.
8. Reconcile substantive review findings without widening scope.
9. Merge only after the exact head is qualified under the live repository controls.
10. Re-read canonical `main`, close Specification 000 if acceptance remains true, then shape Specification 001.

## Design decisions

- Apache-2.0 is selected to maximize real open-source and commercial adoption while providing an explicit patent grant.
- Stable standards are preferred over project-specific protocols.
- Evolving Internet-Drafts are research/watch inputs, not hard dependencies.
- The first implementation program targets local MCP authority because it exercises the product thesis without requiring Tarif to become an IdP or cloud service.
- Information-flow control remains research-only until a bounded experiment qualifies feasibility.

## Expected change surface

Documentation/governance only:

```text
README.md
LICENSE
AGENTS.md
CONSTITUTION.md
docs/**
specs/**
```

No `src/`, `crates/`, package manifests, workflow behavior, generated binaries, or release artifacts are authorized.
