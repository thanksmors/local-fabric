---
okf_version: "0.1"
---
# Knowledge Index

Governed by: [../AGENTS.md](../AGENTS.md)  
Workflow: [../CONTEXT.md](../CONTEXT.md)  
Concept template: [../.fabric/templates/okf-concept.md](../.fabric/templates/okf-concept.md)  
Research template: [../.fabric/templates/research-note.md](../.fabric/templates/research-note.md)  
Decision template: [../.fabric/templates/decision-record.md](../.fabric/templates/decision-record.md)  
Learning template: [../.fabric/templates/learning-note.md](../.fabric/templates/learning-note.md)  
Plan template: [../.fabric/templates/implementation-plan.md](../.fabric/templates/implementation-plan.md)

## Bundle Status

This OKF bundle is initialized with Fabric memory indexes for research, decisions, learnings, and plans. Add project-specific concepts only when durable knowledge exists and the addition passes the YAGNI Gate.

## Bundle Scope

Durable, reusable project knowledge belongs here: architecture, APIs, schemas, domain terms, decisions, runbooks, workflows, datasets, metrics, external resources, research findings, reusable lessons, implementation plans, and handoff plans. Do not use OKF as a diary or speculative roadmap; capture the smallest source-backed record that future agents can reuse.

## Concept Index

| Concept | Type | Path | Description |
|---|---|---|---|
| _None yet_ | — | — | Add project-specific OKF concept files here or through a linked directory index. |

## Directory Index

| Directory | Scope | Index |
|---|---|---|
| `research/` | External web/Perplexity research notes and source summaries. | [research/index.md](./research/index.md) |
| `decisions/` | Decision records with context, options, justification, and consequences. | [decisions/index.md](./decisions/index.md) |
| `learnings/` | Reusable lessons learned from work and research. | [learnings/index.md](./learnings/index.md) |
| `plans/` | Durable implementation plans, design plans, and handoff plans. | [plans/index.md](./plans/index.md) |

## Logs

| Log | Scope |
|---|---|
| [log.md](./log.md) | Bundle-level knowledge updates. |

## OKF Notes

- This root `index.md` may declare `okf_version`; other `index.md` files should not use frontmatter.
- `index.md` and `log.md` are reserved OKF files, not concept files.
- Every other `.md` file under `knowledge/` must have YAML frontmatter with a non-empty `type`.
- Use links to express relationships. Generated concepts should cite source material instead of inventing facts.
- Apply YAGNI: a concept must have a current consumer, current task, durable reuse value, or explicit user request.
