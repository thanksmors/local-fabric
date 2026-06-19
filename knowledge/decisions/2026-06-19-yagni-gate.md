---
type: Decision Record
title: Adopt the YAGNI Gate for Fabric artifact creation
description: Fabric should require a YAGNI check before creating or expanding durable context structure.
resource: /decisions/2026-06-19-yagni-gate.md
tags: [fabric, yagni, decisions, simplicity, no-orphans]
timestamp: 2026-06-19T00:00:00+08:00
status: Accepted
decision_date: 2026-06-19
review_trigger: When Fabric blocks useful work, creates too little memory, or starts accumulating unused scaffold.
---
# Context

Fabric now includes governance, Claude compatibility, ICM routing, OKF knowledge, research capture, decision memory, and learning memory. The risk is that agents may create elaborate context structure before the project has earned it.

The user asked to bake YAGNI into Fabric after reviewing the YAGNI Manifesto.

# Options Considered

| Option | Summary | Tradeoff |
|---|---|---|
| Keep only Simplicity First | Minimal rules, but not enough guidance for docs, OKF, ICM, and workflow artifacts. | Too implicit. |
| Add YAGNI as a short style note | Lightweight, but agents may still create speculative stages or concepts. | Too weak for artifact creation. |
| Add a YAGNI Gate | Explicit creation criteria for code, docs, OKF, ICM, scripts, templates, and abstractions. | Slightly more process, but prevents context bloat. |

# Decision

Adopt the YAGNI Gate as a first-class Fabric governor.

Before adding durable structure, agents must name the current need, current consumer, cost added, cost of delaying, and the smallest useful artifact. Non-trivial exceptions must be recorded as decision records.

# Justification

Fabric is useful only if it stays navigable and lean. YAGNI protects the system from becoming context bureaucracy while preserving clean seams such as empty indexes, templates, and no-orphans checks.

# YAGNI / Speculation Check

| Question | Answer |
|---|---|
| Current owner | Repo owner / user. |
| Current use case | Prevent agents from over-creating ICM stages, OKF concepts, child contracts, scripts, and abstractions. |
| Cost added | A short creation gate in `AGENTS.md`, plus examples in OKF memory and tests. |
| Cost of delay | Fabric tests may normalize speculative context before the gate exists. |
| Smallest useful artifact | Root YAGNI Gate, one decision record, one learning note, and targeted test prompts. |

# Consequences

- Empty scaffold files remain acceptable as cheap seams.
- New real artifacts must pass the YAGNI Gate or be explicitly requested.
- ICM stages are harder to create than OKF notes because stages imply repeatable workflow structure.
- OKF memory remains source-backed and concise, not diary-style.

# Supporting Research

- [YAGNI Manifesto](../research/2026-06-19-yagni-manifesto.md)

# Related Learnings

- [YAGNI keeps Fabric lean](../learnings/2026-06-19-yagni-keeps-fabric-lean.md)
