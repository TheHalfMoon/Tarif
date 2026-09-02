# R3 Qualification Governance Boundary

Tarif treats authorization, canonicalization, credentials, approvals, identity binding, policy enforcement, mediation, cryptography, and evidence-integrity changes as R3 security-boundary work.

## Repository-side qualification workflow

The repository defines one stable GitHub Actions workflow/job contract:

- workflow: `Tarif Qualification`
- job: `qualification`
- observed GitHub check-run context: `qualification`

GitHub's first exact-head run on PR #8 reported check-run `name = "qualification"`. That observed API value, not a guessed UI-composed label, is the canonical required-check context candidate for future branch/ruleset configuration.

The initial workflow validates exact-diff hygiene and the presence of canonical governance/security files. When the Rust workspace is established, Rust formatting, linting, tests, and any later deterministic qualification steps should be added under this stable job rather than renaming the required-check context without a separate governance decision.

## Critical limitation

A workflow file is not a mandatory enforcement boundary by itself.

Until live branch protection or a repository/organization ruleset requires the `qualification` check and required review behavior on `main`, a repository actor with sufficient GitHub permission can merge without satisfying the workflow. Tarif must not call the R3 merge boundary enforced merely because `.github/workflows/qualification.yml` exists.

Issue #3 owns the remaining external enforcement prerequisite.

## Required external evidence

Issue #3 may close only after live GitHub evidence demonstrates the intended controls, including:

- PR-based change control for `main`;
- exact required check context `qualification`, unless later live evidence explicitly changes it through a separate governance decision;
- substantive independent review requirement for R3 work;
- no ordinary force-push/destructive history path;
- bounded negative proof that an unqualified head cannot merge;
- bounded positive proof that a qualified head can merge;
- documented administrative bypass/residual risk.

## R3 execution while external enforcement is absent

R3 work may be researched, shaped, implemented, and verified on a bounded branch when an active specification authorizes it. It must not be merged into canonical `main` while Issue #3 remains unresolved.

This separation preserves forward progress without manufacturing repository-enforcement evidence.
