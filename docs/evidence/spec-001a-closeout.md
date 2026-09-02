# Specification 001A Canonical Closeout Evidence

Specification 001A established the repository-side R3 qualification workflow baseline. It did not establish mandatory branch/ruleset enforcement.

## Canonical merge

- base before merge: `13780df36bd86ec172524eed6545860c90b20d89`
- final exact PR head: `eff072cab450e0447b65a17a72463f0fa3cfcbd6`
- PR: #8 — `governance: establish R3 qualification workflow baseline`
- merge: `89a03d76a99d16b3ce35a7ba8699219a3dbcf7dc`
- canonical post-merge `main`: `89a03d76a99d16b3ce35a7ba8699219a3dbcf7dc`

GitHub reports the merge commit as validly signed. The merge was performed with expected-head protection.

## Exact-head qualification

On final PR head `eff072cab450e0447b65a17a72463f0fa3cfcbd6`:

- GitHub Actions check-run context `qualification`: `success`;
- qualification job: `100268718003`;
- workflow run: `33636541004`;
- Cubic: `neutral` because its monthly review limit was reached; not PASS or approval;
- semantic review: COMMENT only, not independent approval;
- review threads: zero;
- mergeable: true.

The first workflow run also established empirically that the GitHub check-run API context is `qualification`. Earlier guessed UI-composed naming was corrected before merge.

## Post-merge qualification

On canonical merge `89a03d76a99d16b3ce35a7ba8699219a3dbcf7dc`:

- check-run context `qualification`: `success`;
- qualification job: `100269480531`;
- workflow run: `33636771720`.

## Remaining blocker

Live post-merge GitHub truth still reported:

- `main` protected: `false`;
- required status-check enforcement: `off`;
- repository rulesets: none observed.

Therefore Issue #3 remains open and blocks every R3 implementation merge. Workflow presence and successful runs are not equivalent to mandatory enforcement.

## Closeout

Status: `CLOSED_CANONICAL`.

Specification 002 may now be shaped. Its implementation is R3 and may proceed only on a bounded branch while Issue #3 remains unresolved; it must not merge into canonical `main` until that external gate is proven.
