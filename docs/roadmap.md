# Tarif Evidence-Shaped Roadmap

This roadmap records dependency-ordered **program outcomes**, not pre-authorized implementation tasks. Only `specs/CURRENT.md` and the active shaped specification grant product authority.

## Initial v0.1 program

```text
000 Project Foundation & Constitution
  -> 001 Threat Model, TCB & Coverage Contract
  -> 002 Action IR & Deterministic Canonicalization
  -> 003 Decision Core & Default-Deny Policy
  -> 004 Local MCP Enforcement Gate
  -> 005 Runtime Evidence & `tarif why`
  -> 006 Approval Binding & Execution Revalidation
  -> 007 Coverage Inspection & `tarif doctor`
  -> 008 v0.1 Developer Experience & Qualification
  -> 009 v0.1 Release Closeout
```

### 000 — Project Foundation & Constitution

Establish product thesis, open-source doctrine, canonical research, repository governance, bounded program, and proof rules. No runtime implementation authority.

### 001 — Threat Model, TCB & Coverage Contract

Turn the founding threat list into explicit assets/trust boundaries/attack paths/residual risks and define what Tarif may and may not claim to mediate in v0.1.

### 002 — Action IR & Deterministic Canonicalization

Define the internal normalized action contract for supported MCP calls and prove that ambiguous/different relevant requests cannot collapse into an authorization-equivalent representation accidentally.

### 003 — Decision Core & Default-Deny Policy

Deliver the smallest deterministic `ALLOW / DENY / APPROVAL_REQUIRED` engine with explainable reasons. Evaluate AuthZEN/Cedar/OPA interfaces without inventing an unnecessary policy language.

### 004 — Local MCP Enforcement Gate

Mediate one bounded MCP `tools/call` path for local agent workflows. The acceptance claim must remain narrower than whole-agent sandboxing.

### 005 — Runtime Evidence & `tarif why`

Bind decisions to exact normalized requests and expose deterministic explanations/evidence without treating digests as semantic proof.

### 006 — Approval Binding & Execution Revalidation

Bind high-risk approval to exact trusted fields and relevant resource state; reject stale approvals after qualifying changes.

### 007 — Coverage Inspection & `tarif doctor`

Expose mediated, partially mediated, and unmediated paths and detect known bypass conditions such as model-visible credentials where feasible.

### 008 — v0.1 Developer Experience & Qualification

Make the bounded workflow understandable and reproducible against selected real MCP clients/runtimes. Qualification, not adapter count, is the objective.

### 009 — v0.1 Release Closeout

Produce reproducible install/release artifacts, native-platform qualification as selected by evidence, supply-chain provenance, documentation claim audit, and post-release verification.

## Post-v0.1 program directions

These remain intentionally coarse until observation selects them:

- credential isolation and local execution harness;
- remote MCP + OAuth/OIDC/AuthZEN interoperability;
- mission-bound authority interoperability;
- HTTP mediation;
- A2A mediation;
- delegation and authority graph;
- `tarif blast-radius`;
- credential-broker integrations;
- information-flow/provenance experiment;
- information-flow enforcement if the experiment qualifies it;
- conformance/adversarial suite;
- ecosystem adapters;
- HA/Kubernetes and production operations;
- public paid-tool replacement qualification.

## Progressive-refinement rule

Later work remains broad. Near-term work is shaped only when dependency-eligible and supported by current evidence. The repository must resist generating a large stale task backlog merely to appear complete.
