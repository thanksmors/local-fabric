---
type: Research Note
title: YAGNI Manifesto
description: Summary of the YAGNI team guide used to add the Fabric artifact creation gate.
resource: user-provided:yagni_manifesto.pdf
tags: [research, yagni, fabric, simplicity]
timestamp: 2026-06-19T00:00:00+08:00
tool: user-provided-pdf
---
# Query / Prompt

User asked whether YAGNI has a place in Fabric and then asked to bake it in.

# Key Findings

- YAGNI means building what is needed now while keeping change cheap.
- YAGNI is not anti-design; clean seams, tests, clear names, and boundaries are still part of quality.
- Real need beats predicted need: build when customer, product, security, operational, or repo evidence requires it.
- The Rule of Three should guide extraction: solve directly first, tolerate unclear duplication second, extract the shared abstraction when the pattern is real.
- Speculative work should name its current owner, current use case, and expected cost of delay.
- Build ahead anyway when delaying would create high migration cost, data loss, security/privacy/compliance risk, auditability gaps, or broken public contracts.

# Sources

| Source | URL/Citation | Why it matters |
|---|---|---|
| YAGNI Manifesto | `user-provided:yagni_manifesto.pdf`, page 1 | Defines the YAGNI Gate now used by Fabric. |

# Reliability / Limits

- The source is a one-page user-provided team guide, not an external standard.
- Fabric treats it as a local governing preference and decision-making discipline.

# Used In

- [YAGNI Gate decision](../decisions/2026-06-19-yagni-gate.md)
- [YAGNI keeps Fabric lean](../learnings/2026-06-19-yagni-keeps-fabric-lean.md)
- [AGENTS.md](../../AGENTS.md)

# Citations

[1] User-provided `yagni_manifesto.pdf`, page 1.
