# Plan Index

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Knowledge root: [../index.md](../index.md)  
Template: [../../.fabric/templates/implementation-plan.md](../../.fabric/templates/implementation-plan.md)  
Brainstorm template: [../../.fabric/templates/brainstorm.md](../../.fabric/templates/brainstorm.md)

## Scope

Plan records capture durable implementation plans, design plans, and handoff plans. Do not save every inline task list. Save a plan when it is requested, needed for another agent, risky, multi-step, durable, or useful for reconstructing why work was sequenced a certain way.

## Plan Records

| Date | Plan | Path | Status | Used For |
|---|---|---|---|---|
| 2026-06-19 | Bake lightweight brainstorming and planning into Fabric | [2026-06-19-fabric-brainstorming-planning.md](./2026-06-19-fabric-brainstorming-planning.md) | Completed | Fabric v4 update. |
| 2026-06-19 | Local-first agent harness v1 (vertical slice) | [2026-06-19-local-agent-harness-v1.md](./2026-06-19-local-agent-harness-v1.md) | In progress | Tauri+ACP harness; core slice built under [`app/`](../../app/), automated tests green, GUI run-through pending. |

## Logs

| Log | Scope |
|---|---|
| [log.md](./log.md) | Plan index history. |

## Rules

- Prefer short inline plans for simple tasks.
- Save durable plans under `knowledge/plans/` only when requested, needed for handoff, or justified by complexity/risk.
- Plans should include goal, assumptions, file map, numbered tasks, verification, docs/Fabric updates, and YAGNI checks.
- Use numbered options with a **Recommended** choice when the plan contains unresolved design paths.
- Link plans to decisions, research, learnings, source files, issues, PRs, or stage outputs when useful.
- Do not preserve raw brainstorming unless the user explicitly asks; capture the selected decision or plan instead.
