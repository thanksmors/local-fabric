---
type: Learning Note
title: Test stdio JSON-RPC/ACP clients over an in-process duplex; Tauri needs webkit on Linux
description: A stdio JSON-RPC client made generic over async read/write can be fully tested against an in-process mock, and Tauri requires webkit2gtk to compile plus a desktop session to run.
resource: ../../app/src-tauri/src/acp/connection.rs
tags: [learning, tauri, acp, testing, rust]
timestamp: 2026-06-19T00:00:00Z
---
# Learning

Two reusable lessons from building the agent harness (`app/`):

1. **Make the transport generic to make it testable.** The ACP client
   (`connection.rs`) is built over any `AsyncRead`/`AsyncWrite` pair rather than
   hard-wired to child-process stdio. That let the full handshake → streaming →
   permission round-trip be exercised against an in-process mock agent over
   `tokio::io::duplex`, with no real agent binary and no flaky subprocess.
2. **Tauri on Linux has a hard build dep and a runtime dep.** Compiling the
   Tauri crate needs `libwebkit2gtk-4.1-dev` (plus GTK/soup dev libs); *running*
   the GUI needs a desktop session (`DISPLAY`). A headless CI/build container can
   compile, type-check, and unit/integration-test everything, but cannot do a
   live GUI run-through.

# Applies To

- Any stdio JSON-RPC / line-delimited protocol client in this repo (ACP and future agents).
- Building or CI-testing the `app/` Tauri project in headless environments.

# Why It Matters

- High-confidence protocol tests without spawning real agents keeps the riskiest
  layer (streaming + approvals) honest and fast to verify.
- Knowing the headless boundary up front avoids mistaking "can't run the GUI
  here" for "it doesn't work" — automated tests + build gates are the available
  proof; GUI run-through is a separate, explicit step.

# Evidence

- [`app/src-tauri/src/acp/connection.rs`](../../app/src-tauri/src/acp/connection.rs) — `full_session_flow_against_mock_agent` test.
- [v1 plan](../plans/2026-06-19-local-agent-harness-v1.md) — verification section.

# Reuse Guidance

- For a new agent/protocol, keep the pure message layer separate (testable) and
  the I/O layer generic over async read/write; mock with `duplex`.
- In headless builds, treat `cargo test` + `pnpm check` + `pnpm build` as the
  gate; defer GUI checks to a desktop session and say so explicitly.

# YAGNI Boundary

Do not build elaborate agent simulators or a full conformance suite — a focused
mock that covers the real round-trips is enough until a second agent or a real
interop bug demands more.
