# Decision Index

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Knowledge root: [../index.md](../index.md)  
Template: [../../.fabric/templates/decision-record.md](../../.fabric/templates/decision-record.md)

## Scope

Decision records capture durable choices, justification, options considered, tradeoffs, consequences, and review triggers.

## Decision Records

| Date | Decision | Path | Status | Justification |
|---|---|---|---|---|
| 2026-06-18 | Track research, decisions, and learnings as OKF memory | [2026-06-18-fabric-knowledge-capture.md](./2026-06-18-fabric-knowledge-capture.md) | Accepted | Avoids orphaned reasoning and gives future agents reusable context. |
| 2026-06-19 | Adopt the YAGNI Gate for Fabric artifact creation | [2026-06-19-yagni-gate.md](./2026-06-19-yagni-gate.md) | Accepted | Prevents Fabric from becoming context bureaucracy while preserving clean seams. |
| 2026-06-19 | Adopt lightweight brainstorming and planning in Fabric | [2026-06-19-lightweight-brainstorming-planning.md](./2026-06-19-lightweight-brainstorming-planning.md) | Accepted | Gives agents numbered-choice and plan shapes without importing full Superpowers ceremony. |
| 2026-06-19 | Local-first agent harness architecture (Tauri + ACP, cloud-free v1) | [2026-06-19-local-agent-harness-architecture.md](./2026-06-19-local-agent-harness-architecture.md) | Accepted | One ACP client collapses multi-CLI risk; local-first v1 defers cloud behind a clean seam. |

## Logs

| Log | Scope |
|---|---|
| [log.md](./log.md) | Decision history. |

## Rules

- Record durable decisions, not every minor choice.
- Include context, considered options, final decision, justification, consequences, and review trigger.
- Link supporting research, source files, issues, PRs, or user instructions.
- For speculative or hard-to-reverse work, name the current owner, current use case, expected cost of delay, and review trigger.
