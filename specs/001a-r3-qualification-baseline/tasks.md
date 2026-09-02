# Specification 001A Tasks

- [x] T001A-01 Reverify canonical `main`, Issue #3, branch protection, and rulesets.
- [x] T001A-02 Record Specification 001 canonical closeout evidence.
- [x] T001A-03 Add stable `Tarif Qualification` workflow with job/check-run context `qualification`.
- [x] T001A-04 Document that workflow presence is not mandatory enforcement.
- [x] T001A-05 Preserve Issue #3 as the external R3 merge blocker.
- [x] T001A-06 Verify exact bounded diff.
- [x] T001A-07 Open PR #8 and observe first exact-head `qualification` check success on head `4e2ddba43a38fe188da603638f5d9c14b670ba51`.
- [ ] T001A-08 Resolve substantive findings and requalify the final exact head.
- [ ] T001A-09 Guarded merge exact qualified head.
- [ ] T001A-10 Re-read canonical `main` and close 001A.

The observed GitHub check-run context is `qualification`. Workflow success does not close Issue #3 because `main` is not yet externally configured to require that check.
