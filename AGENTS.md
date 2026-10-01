# Tarif Repository Engineering Rules

## Canonical reading order

Before changing the repository, read:

1. `AGENTS.md`;
2. `CONSTITUTION.md`;
3. `specs/CURRENT.md`;
4. `docs/execution-master-plan.md`;
5. the active specification's `spec.md`, `plan.md`, and `tasks.md`;
6. referenced ADRs, research, standards, evidence, tests, and implementation files.

## Authority rules

- Live repository and GitHub truth override stale handoffs.
- Only the active bounded specification authorizes product mutation.
- Roadmap categories and future specifications do not grant implementation authority.
- Re-read canonical `main` after every merge before selecting successor work.
- Do not invent a successor merely to continue activity; require dependency eligibility and fresh evidence where the program calls for it.

## Change rules

- Prefer small, independently reviewable changes.
- Every changed path must trace to the active specification outcome, acceptance, or required supporting evidence.
- No adjacent cleanup, speculative refactor, or dependency addition without active authority.
- Do not invent cryptography, identity protocols, OAuth variants, policy languages, token formats, or mission formats without a documented standards gap.
- Do not execute untrusted repository or external code merely for inspection.

## Proof rules

Tarif follows Diffcipline's proof-before-done discipline:

- `NOT RUN` is never `PASS`.
- Executor or agent self-report is a claim, not proof.
- Verification must bind to the exact implementation revision under review.
- High-risk (`R3`) changes require negative-path and adversarial evidence in addition to ordinary positive tests.
- Unavailable, skipped, neutral, or billing-blocked review systems are not approvals.
- Do not claim `PASS`, `VERIFIED`, `MERGED`, `RELEASED`, `REPLACEABLE`, or `CLOSED_CANONICAL` without exact evidence.

## Risk model

- `R0` — editorial/documentation-only changes with no behavior or governance weakening.
- `R1` — low-risk tooling or non-security behavior.
- `R2` — behavior-affecting work outside the trusted computing base.
- `R3` — authorization, canonicalization, credentials, policy enforcement, identity binding, approvals, cryptography, mediation, evidence integrity, or other security-boundary work.

R3 qualification should include, where relevant: positive tests, negative-path tests, malformed-input tests, stale/replay tests, property/fuzz testing, full repository verification, exact-head CI, substantive independent review, unresolved-thread closure, guarded merge, and post-merge verification.

## Content language

Repository code, comments, specifications, plans, tasks, ADRs, evidence, commit messages, PR bodies, and technical documentation are written in English.

<!-- graft:start -->
## Graft — repository context layer

Use Graft (https://github.com/trailhq/Graft, `@nanonets/graft`) as local developer/agent repository-context and navigation tooling. It does not authorize product mutation, create successor eligibility, or establish security/verification evidence.

If Graft is unavailable or the local `graft/` graph is absent/stale, run `graft init`, select the active agent(s), then run `graft build`. Before broad source exploration prefer `graft check`, `graft map`, `graft ask "<question>" --source`, `graft skeleton <file>`, `graft callers <symbol>`, and `graft grep "<literal>"`. Do not use Graft as a reason to execute untrusted repository/external code during inspection.

Treat `graft/` as a local regenerable cache and do not commit it. Keep usage zero-cost; do not introduce paid model/API usage or unauthorized external egress.

Graft output is context only, never PASS/VERIFIED/R3 evidence, review approval, or closure authority. Continue all active-spec, exact-head, deterministic, test, Jev where applicable, Alibaba Open Code Review, CI, security, and post-merge gates. Never fabricate Graft output, execution, reviews, or evidence.
<!-- graft:end -->
