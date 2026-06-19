---
type: Implementation Plan
title: Local-First Agent Harness v1 (Vertical Slice)
description: Plan for a thin vertical slice of a Tauri agent harness — one agent streaming into a windowed UI with approvals, terminal, and file view.
resource: ./knowledge/plans/2026-06-19-local-agent-harness-v1.md
tags: [tauri, acp, svelte, agent-harness, local-first, plan]
timestamp: 2026-06-19T00:00:00Z
status: in_progress
---
# Local-First Agent Harness v1 (Vertical Slice)

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Knowledge root: [../index.md](../index.md)  
Plan index: [index.md](./index.md)  
Supporting decision: [../decisions/2026-06-19-local-agent-harness-architecture.md](../decisions/2026-06-19-local-agent-harness-architecture.md)  
Supporting research: [../research/2026-06-19-local-agent-harness-stack.md](../research/2026-06-19-local-agent-harness-stack.md)

> **Implementation location (updated):** the v1 slice is built **in this repo
> under [`app/`](../../app/)** (its own Tauri+Svelte project with a dedicated
> child [`AGENTS.md`](../../app/AGENTS.md)), not a separate repo — the practical
> choice given the working repo scope. The rest of `local-fabric` remains the
> Fabric governance scaffold. Extracting `app/` to a standalone repo later is a
> cheap move if desired.

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

## File Map (as built, under `app/`)

| Path | Status | Responsibility |
|---|---|---|
| `app/src-tauri/src/acp/protocol.rs` | done | Pure JSON-RPC + ACP types and `SessionEvent` normalization (unit-tested). |
| `app/src-tauri/src/acp/connection.rs` | done | Async ACP client over agent stdio; request demux, streaming, permission tokens (integration-tested vs. mock agent). |
| `app/src-tauri/src/acp/launchers.rs` | done | Per-agent launch config. **opencode (`opencode acp`) is the validated default**; Claude Code (`npx @zed-industries/claude-code-acp`) and Codex wired too. |
| `app/src-tauri/tests/opencode_acp.rs` | done | `#[ignore]` end-to-end test: drives real `opencode acp` through `AcpConnection` (initialize → session/new → session/prompt), asserts a streamed agent message. |
| `app/src-tauri/src/store.rs` | done | SQLite sessions/settings persistence (unit-tested). |
| `app/src-tauri/src/files.rs` | done | Read-only working-dir file listing (unit-tested). |
| `app/src-tauri/src/lib.rs` | done | Tauri commands + event forwarding; approval gateway is the permission path through `connection`. |
| `app/src-tauri/tauri.conf.json` + capabilities | done | Least-privilege capabilities (`core:default`, `core:event:default`, `opener:default`). |
| `app/src/lib/api.ts` + `app/src/lib/components/*` + `app/src/routes/+page.svelte` | done | Typed IPC bridge, event-stream renderer, approval dialog, file tree, composer. |
| live PTY terminal pane (`portable-pty` + xterm.js) | deferred | Raw shell pane; deferred from v1 — agent command output already surfaces via ACP tool-call events. See Tasks/Verification. |

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

Automated coverage (`cargo test` in `app/src-tauri`, 16 tests green) plus build
gates (`pnpm check`, `pnpm build`, `cargo build`, all clean). GUI run-through is
pending a desktop session — the build container is headless (no `DISPLAY`).

1. **ACP handshake / stream / approval:** ✅ proven by the
   `full_session_flow_against_mock_agent` integration test, which drives
   `initialize` → `session/new` → `session/prompt` with a streamed
   `agent_message_chunk`, a `session/request_permission` round-trip, and turn
   completion over a real duplex transport.
2. **Normalization:** ✅ unit tests cover message classification and every
   modeled `session/update` variant, with unknown payloads preserved verbatim.
3. **File tree:** ✅ `files.rs` listing unit-tested (dirs-first ordering).
4. **Persistence:** ✅ SQLite upsert/list/idempotency unit-tested; live
   restart-reload pending GUI verification.
5. **Live GUI run-through (handshake against the real Claude Code adapter,
   streaming render, approval dialog, restart reload, offline):** ⏳ pending a
   desktop/`DISPLAY` session.
6. **Terminal/fs live pane:** ⏳ deferred (see File Map).
7. **opencode + MiniMax (first real agent):** ✅ real `opencode acp` (v1.17.8)
   replies to our `initialize` with `protocolVersion: 1`, proving live interop.
   Full prompt round-trip on MiniMax is covered by the gated `opencode_acp` test
   (`cargo test --test opencode_acp -- --ignored`), which the user runs after
   `opencode auth login`. opencode is the default agent in the UI.

## Out of v1 Scope

Start-menu/multi-window OS metaphor, in-shell app runtime, standalone app builder,
codehooks/auth/sync, Codex + opencode launchers.

## Phased Roadmap (after v1)

- **v1.5** — Codex + opencode launchers behind the same ACP client; engine picker; OS-shell windowing (WinBox.js) + shadcn-svelte start menu.
- **v2** — standalone app builder (harness pointed at an empty dir + scaffolding templates).
- **v3** — sync engine over SQLite (ElectricSQL / PowerSync / Yjs); codehooks for auth + shared workspaces.
- **v4** — in-shell app runtime + sandbox (separate design effort).

## Closeout

- [x] Core slice implemented under `app/` (ACP client, launcher, approval path, SQLite, file tree, Svelte UI).
- [x] Automated verification green: 16 Rust tests, `pnpm check`, `pnpm build`, `cargo build` (zero warnings).
- [ ] Live GUI run-through against the real Claude Code adapter (pending a desktop session).
- [ ] Live PTY terminal pane (deferred from v1).
- [x] Supporting research and decision captured in this repo's knowledge base.
- [x] Fabric indexes + child `AGENTS.md` updated.
- [x] App code committed in this repo under `app/` (not a separate repo — see note above).
