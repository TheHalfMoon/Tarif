# Tarif Constitution

## I. Deterministic authority

Probabilistic systems may propose actions, policies, scopes, classifications, explanations, or implementation choices. They do not independently grant security-sensitive authority. Correctness-sensitive authorization state and execution gates must be owned by deterministic contracts.

## II. Default deny at consequential boundaries

An unknown, malformed, ambiguous, stale, unverifiable, or unsupported consequential action fails closed. Convenience must not silently widen authority.

## III. Complete mediation and honest coverage

Tarif may claim enforcement only for paths it actually mediates. Unmediated shell, network, credential, transport, or side-channel paths must be surfaced as residual risk rather than hidden behind a generic "protected" status.

## IV. Credentials stay outside model authority

A model must not require direct possession of a downstream long-lived credential merely to request an authorized action. Credential acquisition, derivation, injection, use, and destruction belong outside model reasoning whenever the integration permits it.

## V. Standards before invention

Tarif composes stable open standards and mature open-source primitives before defining proprietary equivalents. A new protocol, token format, policy language, identity scheme, cryptographic construction, or transport requires a written gap analysis showing why existing standards are insufficient.

## VI. Security is not an enterprise feature

The open-source core must contain the security capabilities required to run Tarif honestly in production. Hosted convenience, managed operations, support, or services may exist later; core authorization, audit, approval, self-hosting, and enforcement must not become artificial paywalls.

## VII. Evidence before claims

No workflow, agent, documentation, benchmark, release, or maintainer may represent a security, compatibility, performance, replacement, or completion claim as established without evidence that supports the exact claim.

## VIII. Risk changes rigor

Verification scales with blast radius, not diff size. Changes touching authorization, canonicalization, credentials, approvals, cryptography, identity binding, policy evaluation, mediation, or evidence integrity are high-risk even when the patch is small.

## IX. Repository truth outranks narrative

Canonical repository state, exact revisions, executable checks, observed diffs, review state, and reproducible artifacts outrank chat handoffs, agent summaries, plans, or marketing copy.

## X. Minimality is subordinate to correctness

Prefer the smallest sufficient implementation and avoid speculative abstractions, but never reduce security, correctness, interoperability, recoverability, or explicit acceptance behavior merely to make a diff smaller.

## XI. Agent and model neutrality

Claude, Codex, OpenCode, Hermes, MCP clients, future models, and framework-specific integrations are adapters around Tarif. No provider-specific behavior may become the only path to the core authority contract.

## XII. Future roadmap is not implementation authority

A roadmap item, competitor gap, draft standard, research idea, external review, or founder ambition does not authorize code by itself. Product work becomes authorized only through the current bounded specification and its dependency/evidence gates.
