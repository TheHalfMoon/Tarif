# R3 Enforcement Negative Probe

Status: `INTENTIONAL_FAILURE_PROBE`

This branch exists only to demonstrate the negative path required by Issue #3.
It must never be merged into `main`.

The line below intentionally contains trailing whitespace so the stable `qualification`
job fails at `git diff --check`. After mandatory external enforcement is configured,
this pull request should remain unmergeable because its required `qualification` check
is not successful.

INTENTIONAL_TRAILING_WHITESPACE_PROBE: true  

Do not repair this probe until the negative enforcement observation has been recorded.
