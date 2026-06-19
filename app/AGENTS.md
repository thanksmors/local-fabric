# AGENTS.md — app/ (Local-First Agent Harness)

Parent contract: [../AGENTS.md](../AGENTS.md)

## Purpose

`app/` is the **local-first agent harness** — a Tauri 2 desktop app (Rust core +
system webview) wrapping existing agent CLIs over the **Agent Client Protocol
(ACP)**. v1 is the vertical slice: one agent (Claude Code) streaming into a
single window with working approvals, a working-directory file tree, and local
SQLite persistence. Design and rationale: [v1 plan](../knowledge/plans/2026-06-19-local-agent-harness-v1.md)
and [architecture decision](../knowledge/decisions/2026-06-19-local-agent-harness-architecture.md).

## Ownership

| Area | Owner/Rule |
|---|---|
| Scope | Everything under `app/` (Tauri backend `src-tauri/`, Svelte frontend `src/`). |
| Backend | `src-tauri/src/`: `acp/` (protocol + connection + launchers), `store.rs`, `files.rs`, `lib.rs`. |
| Frontend | `src/`: `lib/api.ts` (typed IPC bridge), `lib/components/`, `routes/+page.svelte`. |
| Excluded from VCS | `node_modules/`, `build/`, `.svelte-kit/`, `src-tauri/target/`, `src-tauri/gen/schemas/` (see `.gitignore`s). |

## Local Contracts

- **One protocol, thin launchers.** All agent interaction flows through the
  single ACP client in `src-tauri/src/acp/`. Adding an agent means adding a
  `LaunchSpec` in `launchers.rs`, not new streaming/permission plumbing.
- **`protocol.rs` stays pure and tested.** Message types and normalization are
  pure data/functions with unit tests; async I/O lives in `connection.rs`.
- **Approvals are not optional.** Agent `session/request_permission` requests
  must surface to the user and block until resolved; never auto-approve.
- **Least privilege.** Keep `src-tauri/capabilities/` minimal; add a capability
  only when a feature needs it.
- **Local-first.** No cloud/sync/central-auth on any critical path; the app must
  work fully offline. SQLite (`store.rs`) is the device-local source of truth.
- **Keep the IPC boundary typed.** `src/lib/api.ts` mirrors the serde shapes in
  `acp/` and `store.rs`; update both sides together.

## Agents: opencode + MiniMax (first working agent)

- **opencode is the default agent.** Launched as `opencode acp` (an ACP server
  over ndJSON stdio) by the `Opencode` `LaunchSpec` in `launchers.rs`. Verified
  against opencode 1.17.8: it replies to our `initialize` frame with
  `protocolVersion: 1`, matching `protocol::PROTOCOL_VERSION`.
- **Model selection is opencode's, not the harness's.** ACP is model-agnostic;
  opencode picks the model from its own config (`model` key, `provider/model`).
  The default is committed at the **repo root** in
  [`opencode.json`](../opencode.json): `minimax-coding-plan/MiniMax-M3`. opencode
  resolves this project config by walking up from its working directory. Do not
  add model/provider logic in the app.
- **Keys never live in this repo.** Use `opencode auth login` (stored in
  `~/.local/share/opencode/auth.json`). The harness spawns `opencode acp` with
  the parent environment inherited; it holds no credentials and needs none.
- Setup steps live in [`README.md`](./README.md).

## Work Guidance

- Frontend uses **Svelte 5 runes** (`$state`, `$props`, `$derived`) — match the
  existing component style.
- Tauri converts camelCase JS argument keys to snake_case Rust params; call
  commands with camelCase from `api.ts`.

## Verification

Run from `app/`:

```bash
pnpm install            # first time
pnpm check              # svelte-check: 0 errors expected
pnpm build              # vite build into build/
cd src-tauri && cargo test   # 16 tests expected green
cd src-tauri && cargo build  # full app links, zero warnings
pnpm tauri dev          # live GUI run (needs a desktop session / DISPLAY)

# End-to-end through real opencode (needs opencode installed + MiniMax authed):
cd src-tauri && cargo test --test opencode_acp -- --ignored --nocapture
```

Notes:
- Building the Tauri crate on Linux needs `libwebkit2gtk-4.1-dev` and the usual
  GTK/soup dev libs installed.
- A live GUI run against the real agent needs Claude Code reachable via
  `npx @zed-industries/claude-code-acp` and a desktop session.

## Child DOX Index

| Child contract | Scope | Purpose | Notes |
|---|---|---|---|
| _None yet_ | — | Index only existing child `AGENTS.md` files. | Do not create speculative child docs. |
