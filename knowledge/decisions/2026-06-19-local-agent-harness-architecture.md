---
type: Decision Record
title: Local-First Agent Harness Architecture (Tauri + ACP, Cloud-Free v1)
description: Adopt a Tauri desktop app that wraps existing agent CLIs through a single ACP client, ships fully local in v1, and defers cloud/sync/auth.
resource: ../plans/2026-06-19-local-agent-harness-v1.md
tags: [decision, tauri, acp, local-first, architecture]
timestamp: 2026-06-19T00:00:00Z
status: Accepted
decision_date: 2026-06-19
review_trigger: After the v1 vertical slice ships, or if ACP cannot cover the desired streaming/approval UX for a target agent.
---
# Context

The user wants a local-first desktop app that looks like an OS (start menu +
windows) but is really an agent coding harness / app factory: it must run bash,
read local files, and drive existing agent CLIs (Claude Code, Codex CLI,
opencode). The riskiest design question was how to normalize three different
agents' streaming output and approval flows, plus how to do windowing, auth, and
live UI streaming without overbuilding.

# Options Considered

| Option | Summary | Tradeoff |
|---|---|---|
| Tauri + wrap CLIs via one ACP client | Rust core speaks ACP (JSON-RPC/stdio) to agent subprocesses; per-agent code is a thin launcher. | Depends on a young standard; needs native fallbacks where ACP lacks features. |
| Bespoke per-CLI parsers | Hand-write stream + approval handling for each CLI. | Triples the riskiest work; brittle to CLI changes. |
| Electron instead of Tauri | Mature ecosystem, larger bundles/memory. | Heavier; weaker fit for a lightweight "OS" shell and local shell/fs access. |
| codehooks.io as engine/sync | Cloud backend service. | Cannot touch local shell/filesystem; disqualified as engine and sync layer. |

# Decision

Build a **Tauri** app (Rust core + system webview) whose engine **wraps existing
agent CLIs through a single ACP client** with thin per-agent launchers. Frontend
is **Svelte 5 + Vite**, windowing via **WinBox.js**, components via
**shadcn-svelte**. **v1 ships with zero cloud**: local device is source of truth,
each agent CLI manages its own auth, and SQLite stores sessions/projects/settings.
Cloud sync (real sync engine over SQLite) and codehooks auth/sharing are deferred
behind a clean seam and kept off the critical path.

# Justification

ACP is an open standard that already defines the exact client↔agent contract the
app needs (streaming, tool calls, permission requests) and all three target
agents are ACP-capable — collapsing the riskiest part into one client. Tauri +
Svelte match the lightweight, local-shell/fs requirements. Deferring cloud honors
YAGNI: v1 proves the harness with no speculative sync/auth infrastructure.

# Consequences

- Benefit: per-agent work shrinks to launchers; one normalized event stream feeds
  the UI; v1 has no cloud dependency and works offline.
- Tradeoff: reliance on an evolving standard (ACP) requires per-agent native
  fallbacks and a spike to confirm the approval UX.
- Follow-up: app code is greenfield in a new repo (this `local-fabric` repo is
  governance only); sync/auth seam must stay off the critical path.

# YAGNI / Speculation Check

| Question | Answer |
|---|---|
| Current owner | The repo owner building the harness. |
| Current use case | Ship a thin vertical slice: one agent streaming into a windowed UI with approvals, terminal, and file view. |
| Cost added | A Tauri+Svelte project and one ACP client; deliberately no cloud/sync/auth code. |
| Cost of delay | Choosing bespoke parsers or cloud-coupling now would create high rework and put cloud on the critical path. |
| Smallest useful artifact | One ACP client + Claude Code launcher + single session window + approval gateway. |

# Supporting Research

- [Local-first agent harness stack](../research/2026-06-19-local-agent-harness-stack.md)

# Related Learnings

- _None yet._
