---
type: Decision Record
title: Adopt Lightweight Brainstorming and Planning in Fabric
description: Decision to add Superpowers-inspired brainstorming and planning behavior without importing the full Superpowers system.
resource: ./knowledge/decisions/2026-06-19-lightweight-brainstorming-planning.md
tags: [fabric, brainstorming, planning, superpowers, yagni]
timestamp: 2026-06-19T00:00:00Z
status: accepted
decision_date: 2026-06-19
review_trigger: If planning artifacts become ceremonial or agents skip user choice framing.
---
# Adopt Lightweight Brainstorming and Planning in Fabric

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Decision index: [index.md](./index.md)  
Supporting research: [../research/2026-06-19-superpowers-brainstorming-plans.md](../research/2026-06-19-superpowers-brainstorming-plans.md)  
Related plan: [../plans/2026-06-19-fabric-brainstorming-planning.md](../plans/2026-06-19-fabric-brainstorming-planning.md)  
Related learning: [../learnings/2026-06-19-brainstorming-outputs-not-noise.md](../learnings/2026-06-19-brainstorming-outputs-not-noise.md)  
Related YAGNI decision: [2026-06-19-yagni-gate.md](./2026-06-19-yagni-gate.md)

## Context

The user wants Fabric to support brainstorming and implementation planning inspired by `obra/superpowers/skills`, with numbered choices and one recommended option so decisions are easy to review. The change must not turn Fabric into process bloat.

## Options Considered

1. **Add only an AGENTS.md behavior note** — minimal but leaves agents to invent plan formats.
2. **Add behavior rules, templates, and `knowledge/plans/`** — gives concrete shapes and durable plan memory while staying lightweight.
3. **Import the full Superpowers skill system** — powerful but too broad and likely to violate YAGNI.
4. **Create `knowledge/brainstorms/`** — preserves raw ideation but risks noisy, low-value memory.

## Decision

Adopt option 2: add lightweight brainstorming/planning rules, two templates, and a `knowledge/plans/` OKF subtree. Do not import the full Superpowers system and do not create a default `knowledge/brainstorms/` subtree.

## Justification

- The user explicitly prefers numbered choices with a recommended option.
- Templates reduce hallucinated formats and preserve consistency.
- Durable plans are useful for handoff and future context reconstruction.
- Raw brainstorming is usually transient; decisions, plans, research, and learnings are the reusable outputs.
- The YAGNI Gate allows the cheap seam but blocks mandatory ceremony for trivial tasks.

## Consequences

- Agents should present numbered options and mark one as **Recommended** for ambiguous choices.
- Agents should create durable plan records only when requested, needed for handoff, risky, multi-step, or useful for future context.
- Simple tasks continue to use short inline plans or no plan when obvious.
- Future changes to planning behavior must update `AGENTS.md`, `.fabric/templates/`, and `knowledge/plans/index.md` together.

## Review Trigger

Review this decision if agents start creating plan files for trivial work, skipping numbered recommendations for ambiguous choices, or treating Superpowers as a mandatory dependency.
