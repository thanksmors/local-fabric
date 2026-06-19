---
type: Research Note
title: Superpowers Brainstorming and Writing Plans
description: Research summary for adopting lightweight brainstorming and planning behavior in Fabric.
resource: https://github.com/obra/superpowers/tree/main/skills
tags: [fabric, superpowers, brainstorming, planning, yagni]
timestamp: 2026-06-19T00:00:00Z
tool: Web research
status: completed
---
# Superpowers Brainstorming and Writing Plans

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Research index: [index.md](./index.md)  
Used by decision: [../decisions/2026-06-19-lightweight-brainstorming-planning.md](../decisions/2026-06-19-lightweight-brainstorming-planning.md)  
Used by plan: [../plans/2026-06-19-fabric-brainstorming-planning.md](../plans/2026-06-19-fabric-brainstorming-planning.md)

## Question

Which parts of `obra/superpowers/skills` should Fabric adopt for brainstorming and writing implementation plans without violating YAGNI?

## Sources Checked

| Source | URL | Relevance |
|---|---|---|
| Superpowers skills directory | `https://github.com/obra/superpowers/tree/main/skills` | Shows available skills and confirms `brainstorming` and `writing-plans` exist among many other skills. |
| Brainstorming skill | `https://raw.githubusercontent.com/obra/superpowers/main/skills/brainstorming/SKILL.md` | Source for one-question-at-a-time clarification, 2–3 approaches, recommendations, approval gates, and YAGNI emphasis. |
| Writing-plans skill | `https://raw.githubusercontent.com/obra/superpowers/main/skills/writing-plans/SKILL.md` | Source for file mapping, bite-sized tasks, exact verification, no placeholders, and DRY/YAGNI/TDD planning. |

## Key Findings

- The Superpowers repo contains a broader skills system, not only brainstorming and planning. Importing the whole system would create unnecessary process weight for Fabric.
- The brainstorming skill is useful for Fabric because it emphasizes project-context exploration, one question at a time, 2–3 approaches with tradeoffs, a recommendation, and YAGNI.
- The writing-plans skill is useful for Fabric because it emphasizes concrete file maps, bite-sized independently verifiable tasks, exact commands, no placeholders, DRY, YAGNI, and TDD.
- Fabric should adopt the user-facing behavior and template shapes, not the entire Superpowers workflow or mandatory spec-commit process.

## Reliability Limits

- GitHub raw files were checked at the time of this note. Upstream Superpowers content may change.
- Fabric intentionally adapts the concepts instead of claiming compatibility with Superpowers.

## Informed

- [Lightweight brainstorming and planning decision](../decisions/2026-06-19-lightweight-brainstorming-planning.md)
- [Fabric brainstorming/planning implementation plan](../plans/2026-06-19-fabric-brainstorming-planning.md)
- [Brainstorming outputs, not noise learning](../learnings/2026-06-19-brainstorming-outputs-not-noise.md)
