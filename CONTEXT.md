# Project Context

Governed by: [AGENTS.md](./AGENTS.md)  
Knowledge: [knowledge/index.md](./knowledge/index.md)  
Research: [knowledge/research/index.md](./knowledge/research/index.md)  
Decisions: [knowledge/decisions/index.md](./knowledge/decisions/index.md)  
Learnings: [knowledge/learnings/index.md](./knowledge/learnings/index.md)  
Plans: [knowledge/plans/index.md](./knowledge/plans/index.md)  
Stage template: [.fabric/templates/stage-CONTEXT.md](./.fabric/templates/stage-CONTEXT.md)

## Workspace Status

This is an empty ICM root router. No staged workflow is active until real stage folders are added under `stages/` and listed in the Stage Index below. Root routing is a cheap seam; real stages must pass the YAGNI Gate in `AGENTS.md`.

## Workflow Purpose

_None yet._ Add a concise purpose only when this repo has a repeatable workflow with stage boundaries, human review points, durable intermediate artifacts, or an explicit user request.

## Stage Index

| Stage | Path | Purpose | Inputs | Outputs |
|---|---|---|---|---|
| _None yet_ | — | Add only existing `stages/<NN_slug>/CONTEXT.md` files. | — | — |

## Shared References

| Path | Scope | Why |
|---|---|---|
| _None yet_ | — | Add only durable references shared across stages. |

## Workflow Rules

- Create stages only for repeatable workflows, not one-off tasks.
- Pass the YAGNI Gate before adding stages: name the current consumer, current workflow need, cost added, and cost of delaying staged structure.
- If a process is not yet repeatable, start with a checklist here; promote to numbered stages only when the pattern is real or explicitly requested.
- Each stage must do one job and have its own `CONTEXT.md`.
- Each stage `CONTEXT.md` must link back to this file and to the governing `AGENTS.md`.
- Stage-local `output/` is working space. Promote durable outputs to `knowledge/`, docs, or an owning index before relying on them later.
- External research used inside a stage must be captured in `knowledge/research/` before closeout.
- Durable stage decisions belong in `knowledge/decisions/`; reusable lessons belong in `knowledge/learnings/`; durable handoff or implementation plans belong in `knowledge/plans/`.
- Keep this file as the workflow router; keep detailed stage instructions inside stage-local `CONTEXT.md` files.
