# Research Index

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Knowledge root: [../index.md](../index.md)  
Template: [../../.fabric/templates/research-note.md](../../.fabric/templates/research-note.md)

## Scope

Research notes capture external web or Perplexity research used for repo work. They should preserve the question asked, tool used, sources checked, key findings, limits, and what the research informed.

## Research Notes

| Date | Topic | Path | Tool | Used In |
|---|---|---|---|---|
| 2026-06-18 | Fabric reference research | [2026-06-18-fabric-references.md](./2026-06-18-fabric-references.md) | Web research | [Fabric knowledge-capture decision](../decisions/2026-06-18-fabric-knowledge-capture.md) |
| 2026-06-19 | YAGNI Manifesto | [2026-06-19-yagni-manifesto.md](./2026-06-19-yagni-manifesto.md) | User-provided PDF | [YAGNI Gate decision](../decisions/2026-06-19-yagni-gate.md) |
| 2026-06-19 | Superpowers brainstorming and writing plans | [2026-06-19-superpowers-brainstorming-plans.md](./2026-06-19-superpowers-brainstorming-plans.md) | Web research | [Lightweight brainstorming/planning decision](../decisions/2026-06-19-lightweight-brainstorming-planning.md) |
| 2026-06-19 | Local-first agent harness stack (Tauri, ACP, Svelte) | [2026-06-19-local-agent-harness-stack.md](./2026-06-19-local-agent-harness-stack.md) | Web research | [Local agent harness architecture decision](../decisions/2026-06-19-local-agent-harness-architecture.md) |

## Logs

| Log | Scope |
|---|---|
| [log.md](./log.md) | Research index history. |

## Rules

- Create one research note per coherent question or topic.
- Prefer Perplexity when `PERPLEXITY_API_KEY` is available locally and the agent has tooling for it.
- Record fallback web research when Perplexity is unavailable.
- Summarize sources; do not paste long copyrighted excerpts.
- Link research notes to decisions, learnings, issues, PRs, docs, or source files they informed.
- Apply YAGNI to note length: keep durable findings, limits, and uses; delete unsupported or speculative filler.
