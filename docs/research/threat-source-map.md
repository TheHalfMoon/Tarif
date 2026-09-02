# Tarif Threat Source Map

Specification 001 owns the formal threat model. This file records founding inputs and test directions.

## Primary external inputs

- NIST Software & AI Agent Identity and Authorization project/concept paper
- OWASP GenAI / Agentic Security Initiative material
- Cloud Security Alliance AI-agent confused-deputy and prompt-injection research
- OAuth Security Best Current Practice (RFC 9700)
- MCP authorization/security requirements
- OpenAI Codex sandboxing/network/credential safety material
- SPIFFE/SPIRE workload-attestation model
- AuthBench and related least-privilege-agent research

## Threat families

| Family | Founding test question |
| --- | --- |
| Prompt injection | Can untrusted content cause a consequential action outside approved authority? |
| Confused deputy | Can a validly authenticated agent misuse a valid credential for attacker-chosen effect? |
| Credential exfiltration | Can model-visible context retrieve or transmit downstream credentials? |
| Argument substitution | Can authorization for one parameter set be replayed for another? |
| Tool spoofing | Can equivalent/confusable tool identifiers bypass policy? |
| Replay | Can a prior approval/grant be reused outside its intended lifetime/context? |
| Stale state / TOCTOU | Does a material resource change invalidate a prior approval where required? |
| Delegation escalation | Can a sub-agent obtain greater authority than its delegator? |
| Parser ambiguity | Do semantically different requests normalize to the same authorization identity? |
| Policy bypass | Can an alternate path avoid the configured PEP? |
| Direct shell/network bypass | Can an agent bypass Tarif despite a "protected" claim? |
| Compromised tool/server | Does a malicious tool response create new authority? |
| Evidence tampering | Can audit/evidence be modified without detection or without surfacing uncertainty? |

## Information-flow research track

Long-term authorization must consider compositions such as:

```text
ALLOW read(confidential)
ALLOW write(external)
```

which must not automatically imply:

```text
ALLOW confidential -> external
```

Information-flow/taint/provenance controls remain a research direction. No production implementation is authorized until a bounded experiment defines source/sink labels, propagation rules, false-positive/false-negative evidence, performance cost, and a GO/NO-GO decision.
