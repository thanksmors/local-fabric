---
type: Decision Record
title: Track research, decisions, and learnings as OKF memory
description: Fabric should capture repo research, decisions, and reusable lessons as indexed OKF concepts.
resource: /decisions/2026-06-18-fabric-knowledge-capture.md
tags: [fabric, decisions, research, learnings, okf]
timestamp: 2026-06-18T00:00:00+08:00
status: Accepted
decision_date: 2026-06-18
review_trigger: When research capture becomes too noisy or the OKF spec materially changes.
---
# Context

Fabric already governs agent behavior with `AGENTS.md`, routes workflows with `CONTEXT.md`, and stores durable knowledge in `knowledge/`. The user prefers Perplexity-backed research via `.env`, wants external research preserved for future use, and wants learnings and decisions tracked with justification.

# Options Considered

| Option | Summary | Tradeoff |
|---|---|---|
| Keep preferences only in `AGENTS.md` | Simple, but loses research and decision history. | Too little memory. |
| Store everything in root `AGENTS.md` | Easy to find, but bloats the control plane. | Violates separation of authority and knowledge. |
| Store research, decisions, and learnings as OKF subtrees | Keeps governance concise and memory navigable. | Requires index upkeep. |

# Decision

Use OKF subtrees for persistent memory:

- `knowledge/research/` for web/Perplexity research notes.
- `knowledge/decisions/` for decision records with justification.
- `knowledge/learnings/` for reusable lessons.

Keep personal preferences and capture rules in `AGENTS.md`.

# Justification

This preserves the user's research and decision-tracking preferences while keeping the root contract concise. It also gives future agents a concrete navigation path instead of forcing them to infer where research notes or decisions belong.

# Consequences

- Research performed for repo work must produce or update a research note.
- Durable decisions must produce or update a decision record.
- Reusable learnings should be captured when they are likely to matter again.
- Indexes must be updated in the same change to avoid orphan memory.

# Supporting Research

- [Fabric reference research](../research/2026-06-18-fabric-references.md)

# Related Learnings

- [Context indexes reduce hallucinated structure](../learnings/2026-06-18-context-indexes-prevent-orphans.md)
