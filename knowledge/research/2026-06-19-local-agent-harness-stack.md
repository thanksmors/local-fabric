---
type: Research Note
title: Local-First Agent Harness Stack (Tauri, ACP, Svelte)
description: Research resolving the framework, engine-integration, auth, and interop questions for a local-first desktop agent harness / app factory.
resource: https://agentclientprotocol.com
tags: [research, tauri, acp, svelte, agent-harness, local-first]
timestamp: 2026-06-19T00:00:00Z
tool: Web research
status: completed
---
# Local-First Agent Harness Stack (Tauri, ACP, Svelte)

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Research index: [index.md](./index.md)  
Used by decision: [../decisions/2026-06-19-local-agent-harness-architecture.md](../decisions/2026-06-19-local-agent-harness-architecture.md)  
Used by plan: [../plans/2026-06-19-local-agent-harness-v1.md](../plans/2026-06-19-local-agent-harness-v1.md)

## Question

For a local-first desktop app that behaves like an OS (start menu + windows) but
is really an agent coding harness / app factory wrapping existing agent CLIs,
which choices resolve: (1) frontend framework + windowing, (2) engine
integration across Claude Code / Codex CLI / opencode, (3) auth / API keys, and
(4) Tauri JS↔Rust interop for live streaming?

## Sources Checked

| Source | URL | Relevance |
|---|---|---|
| Agent Client Protocol | `https://agentclientprotocol.com` | Open standard (JSON-RPC over stdio) defining client↔agent streaming, tool calls, and permission/approval requests. |
| ACP agent registry | `https://agentclientprotocol.com` | Confirms Claude Code (`@zed-industries/claude-code-acp`), Codex, and opencode are available as ACP agents. |
| Tauri — Calling Rust from the Frontend | `https://v2.tauri.app/develop/calling-rust/` | `invoke()` commands, `emit` events, and `tauri::ipc::Channel` for continuous streaming. |
| TauRPC typed IPC | `https://github.com/MatsDK/TauRPC` | Specta-based typed IPC layer incl. typed Channel callbacks for end-to-end type safety. |
| WinBox.js | `https://nextapps-de.github.io/winbox/` | ~5.7 kB zero-dep HTML5 window manager (drag/resize/min/max/z-index); Svelte wrapper exists. |
| shadcn-svelte / Bits UI | `https://shadcn-svelte.com` | Runes-native copy-in components on Bits UI headless primitives + Tailwind. |

## Key Findings

- **Engine integration is the biggest win:** the Agent Client Protocol (ACP)
  already standardizes the client↔agent contract — JSON-RPC over stdio with
  streaming output, tool calls, and permission requests. All three target agents
  are ACP-capable, so "normalize three CLI formats" collapses to "implement one
  ACP client + thin per-agent launchers." Native per-CLI fallbacks exist where
  ACP lacks a feature (Claude Code `--output-format stream-json`; Codex `--json`
  / app-server; opencode `run --format json` / `serve`).
- **Frontend = Svelte 5 + Vite** (no-vDOM, small/fast bundles fit a desktop
  shell); Tauri serves static assets so SPA / SvelteKit static adapter applies.
- **Windowing should not be hand-rolled:** WinBox.js provides OS-style window
  chrome (drag/resize/min/max/fullscreen/z-index) as a tiny library with a
  Svelte wrapper; svelte-moveable / @neodrag/svelte are fallbacks for custom
  frames. shadcn-svelte (Bits UI + Tailwind) covers in-window content, the start
  menu, and context menus.
- **Auth: each agent CLI owns its own credentials** (Claude Code, Codex, opencode
  each manage login/keys). v1 needs no central key store; a unified key/proxy is
  deferred cloud work.
- **Tauri JS↔Rust interop fits the design:** `invoke()` for JS→Rust (serde
  args/returns), `emit` events + `tauri::ipc::Channel` for Rust→JS streaming —
  the natural carrier for the ACP event stream into the Svelte UI. tauri-specta /
  TauRPC add an end-to-end-typed boundary.
- **codehooks.io is cloud-only** — cannot access local shell/filesystem, so it is
  not the agent engine and not the sync layer; reclassified as optional future
  auth/sharing. Local-first sync (if ever added) belongs to a real sync engine
  (ElectricSQL / PowerSync / Replicache / Yjs / Automerge over local SQLite).

## YAGNI Filter

Preserved because this research directly drives a multi-phase greenfield build:
it locks the v1 stack (Tauri + Svelte + ACP) and records why the riskiest part
(multi-CLI streaming/approvals) is solved by an open standard. Reference "OS
desktop" projects (daedalOS, web-desktop-environment, OrbitOS, Aurora OS) are
noted as UX inspiration only, not dependencies.

## Reliability / Limits

- ACP is young and evolving; per-agent ACP coverage of the desired approval UX
  must be confirmed during the v1 spike, keeping native fallbacks where needed.
- Source pages were checked at note time; upstream APIs/flags may change.
- Reference OS-desktop projects are mostly React — inspiration, not stacks to
  adopt.

## Used In

- [Local agent harness architecture decision](../decisions/2026-06-19-local-agent-harness-architecture.md)
- [Local agent harness v1 implementation plan](../plans/2026-06-19-local-agent-harness-v1.md)

## Citations

[1] https://agentclientprotocol.com
[2] https://v2.tauri.app/develop/calling-rust/
[3] https://github.com/MatsDK/TauRPC
[4] https://nextapps-de.github.io/winbox/
[5] https://shadcn-svelte.com
