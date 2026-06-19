---
type: Implementation Plan
title: Local-First Agent Harness v1 (Vertical Slice)
description: Plan for a thin vertical slice of a Tauri agent harness — one agent streaming into a windowed UI with approvals, terminal, and file view.
resource: ./knowledge/plans/2026-06-19-local-agent-harness-v1.md
tags: [tauri, acp, svelte, agent-harness, local-first, plan]
timestamp: 2026-06-19T00:00:00Z
status: approved
---
# Local-First Agent Harness v1 (Vertical Slice)

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Knowledge root: [../index.md](../index.md)  
Plan index: [index.md](./index.md)  
Supporting decision: [../decisions/2026-06-19-local-agent-harness-architecture.md](../decisions/2026-06-19-local-agent-harness-architecture.md)  
Supporting research: [../research/2026-06-19-local-agent-harness-stack.md](../research/2026-06-19-local-agent-harness-stack.md)

> The app is greenfield in a **new project/repo**. This `local-fabric` repo is
> the Fabric governance scaffold and holds this plan as durable handoff memory,
> not the app code.

## Goal

Prove the harness end-to-end with one agent (Claude Code): a single Tauri+Svelte
window that streams ACP events with working approvals, a terminal, and a file
view, validating the architecture before the OS metaphor or app factory.

## Assumptions

- Engine = wrap existing agent CLIs via one ACP client; per-agent code is a thin launcher.
- v1 is fully local: no cloud, sync, or central auth; each CLI manages its own credentials.
- Reuse libraries for windowing (WinBox.js) and components (shadcn-svelte); custom work is wiring/state only.

## YAGNI Gate

- Current need: a working vertical slice proving streaming + approvals + local shell/fs.
- Current owner/consumer: the repo owner building the harness.
- Cost added: one Tauri+Svelte project and one ACP client; no cloud code.
- Cost of delay: bespoke parsers or cloud-coupling would create high rework.
- Smallest useful plan chosen: one agent, single window, ACP client + Claude Code launcher.

## File Map (new project)

| Path | Create/Modify | Responsibility |
|---|---|---|
| `src-tauri/src/acp/mod.rs` | create | ACP client (JSON-RPC/stdio) + normalized `SessionEvent` type. |
| `src-tauri/src/acp/launchers.rs` | create | Per-agent launch config (Claude Code first). |
| `src-tauri/src/process.rs` | create | PTY/process spawning (`portable-pty`). |
| `src-tauri/src/approval.rs` | create | Approval gateway relaying ACP permission requests. |
| `src-tauri/src/store.rs` | create | SQLite session/project/settings persistence. |
| `src-tauri/tauri.conf.json` + capabilities | create | Least-privilege shell + fs allowlist. |
| `src/` (Svelte) | create | Session window, event renderer, file tree, terminal, approval dialog. |

## Options Considered

1. **Tauri + one ACP client + thin launchers** — collapses multi-CLI risk into one standard.
2. **Bespoke per-CLI parsers** — triples the riskiest work; brittle.
3. **Electron** — heavier; weaker fit for a lightweight local shell.

**Recommended / Chosen:** Option 1 (see supporting decision).

## Tasks

### Task 1: Tauri + Svelte scaffold
**Goal:** App boots with Svelte 5/Vite frontend and a least-privilege shell+fs capability allowlist.

### Task 2: ACP client in Rust
**Goal:** Speak JSON-RPC over stdio to an agent subprocess; surface a normalized event stream over Tauri events/Channel.

### Task 3: Claude Code launcher
**Goal:** Start Claude Code as an ACP agent (stream-json fallback if needed); clean startup/teardown.

### Task 4: Session window UI
**Goal:** Render the event stream — messages, tool calls, output — distinctly in one window.

### Task 5: Approval gateway
**Goal:** Surface ACP permission requests as UI prompts that block execution until the user decides; relay the decision to the agent.

### Task 6: File tree + terminal panel
**Goal:** Read-only fs browse of the working dir + live terminal (`portable-pty`) for raw shell output.

### Task 7: Local SQLite store
**Goal:** Persist sessions/projects/settings with a schema friendly to a future sync engine.

## Verification (milestone-based)

1. **ACP handshake:** Rust launches Claude Code as ACP subprocess; session init completes over stdio.
2. **Stream proof:** a prompt yields streamed messages/tool-calls rendered distinctly in the UI.
3. **Approval proof:** a risky action raises a permission request; the UI blocks until decided; decision reaches the agent.
4. **Terminal/fs proof:** agent shell output shows live; file tree reflects on-disk changes.
5. **Persistence proof:** restart reloads prior sessions from SQLite.
6. **Offline proof:** with network disconnected, v1 still functions.
7. **Second-launcher proof (v1.5 gate):** a Codex launcher flows through the same ACP client with no UI changes.

## Out of v1 Scope

Start-menu/multi-window OS metaphor, in-shell app runtime, standalone app builder,
codehooks/auth/sync, Codex + opencode launchers.

## Phased Roadmap (after v1)

- **v1.5** — Codex + opencode launchers behind the same ACP client; engine picker; OS-shell windowing (WinBox.js) + shadcn-svelte start menu.
- **v2** — standalone app builder (harness pointed at an empty dir + scaffolding templates).
- **v3** — sync engine over SQLite (ElectricSQL / PowerSync / Yjs); codehooks for auth + shared workspaces.
- **v4** — in-shell app runtime + sandbox (separate design effort).

## Closeout

- [ ] v1 milestones 1–6 pass.
- [x] Supporting research and decision captured in this repo's knowledge base.
- [x] Fabric indexes updated.
- [ ] App code committed to its own (new) repo.
