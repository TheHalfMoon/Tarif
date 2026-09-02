# Security Policy

Tarif is a security-sensitive open-source project. Responsible disclosure and precise claim boundaries are part of the engineering model.

## Current security status

Tarif is in its founding phase and does **not** yet claim production-ready enforcement, sandboxing, credential isolation, or protection from prompt injection. The absence of production code does not remove the need to report security issues in governance, future code, examples, build/release paths, or documentation claims.

## Reporting a vulnerability

Prefer GitHub's private vulnerability reporting / Security Advisory workflow for this repository when it is available to you.

If a private reporting path is unavailable, do not publish exploit details, secrets, credentials, or a working attack in a public issue. Open only a minimal public issue asking the maintainers to establish a private reporting channel, without including sensitive technical details.

A useful private report should include:

- affected exact revision/version;
- affected component or trust boundary;
- reproduction steps or proof of concept;
- expected versus observed behavior;
- security impact and prerequisites;
- known workarounds or mitigations, if any.

## Handling principles

- Security reports are evidence inputs, not automatic severity labels.
- Fixes touching authorization, canonicalization, credentials, approvals, identity binding, policy enforcement, mediation, cryptography, or evidence integrity are normally treated as `R3` work.
- A fix is not complete merely because the reporter or implementing agent says it works; exact-change verification and relevant adversarial regression evidence are required.
- Public disclosure timing should avoid needlessly exposing users before a fix or clear mitigation is available.

## Supported versions

No production release exists yet. A supported-version matrix will be added before the first production-capable release and must remain consistent with published release artifacts.
