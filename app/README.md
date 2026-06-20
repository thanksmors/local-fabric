# local-fabric — agent harness

A local-first desktop app (Tauri 2 + Svelte 5) that wraps agent CLIs over the
[Agent Client Protocol (ACP)](https://agentclientprotocol.com). The first
working agent is **opencode**, driven by a **MiniMax** API key.

Governance and design live in the repo's Fabric docs: see
[`AGENTS.md`](./AGENTS.md) (this app's contract) and the
[v1 plan](../knowledge/plans/2026-06-19-local-agent-harness-v1.md).

## Prerequisites

- **Node 18+** and **pnpm**
- **Rust** (stable) + Tauri's Linux deps: `libwebkit2gtk-4.1-dev`,
  `build-essential`, `libssl-dev`, `libayatana-appindicator3-dev`,
  `librsvg2-dev`, `libxdo-dev` (see the Tauri docs for your OS)
- **opencode** — `curl -fsSL https://opencode.ai/install | bash`
  (or `npm i -g opencode-ai`)

## Set up opencode + MiniMax

The harness launches `opencode acp` and lets opencode pick the model from its own
config — ACP is model-agnostic. So "use MiniMax" is opencode configuration, not
app configuration. **Your API key never goes in this repo.**

The default model and provider auth are set for you in the repo-root
[`opencode.json`](../opencode.json):

```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "minimax-coding-plan/MiniMax-M3",
  "provider": {
    "minimax-coding-plan": {
      "options": { "headers": { "X-Api-Key": "{env:MINIMAX_API_KEY}" } }
    }
  }
}
```

opencode finds this project config by walking up from its working directory, so
you don't touch any hidden config folder. **You only supply the key via an
environment variable** — it never goes in the repo:

```bash
export MINIMAX_API_KEY="<your-minimax-coding-plan-key>"   # from platform.minimax.io
opencode run "say hello in five words"                    # sanity check
```

### Why the explicit `X-Api-Key` header

`minimax-coding-plan` is an Anthropic-compatible provider (`@ai-sdk/anthropic`,
endpoint `https://api.minimax.io/anthropic/v1`) that authenticates with an
`X-Api-Key` header. Current opencode builds do **not** reliably attach the stored
credential for this provider — even after `opencode auth login` you get
`login fail: Please carry the API secret key in the 'X-Api-Key' field`. The
`headers` override above forces the key from `MINIMAX_API_KEY` onto every request,
which fixes it. (If your plan lives on `minimaxi.com` instead of `minimax.io`,
switch the model's provider accordingly.)

## Run the app

```bash
pnpm install
pnpm tauri dev          # needs a desktop session (DISPLAY)
```

In the window: the agent selector defaults to **opencode** — set your working
directory and click **Start session**, then prompt the agent. Approval requests
surface as a blocking dialog.

## Verify the build (no GUI / headless friendly)

```bash
pnpm install
pnpm check              # svelte-check: 0 errors
pnpm build              # vite build -> build/
cd src-tauri
cargo test              # unit + mock-agent integration tests
cargo build             # whole app links

# End-to-end through real opencode (needs opencode installed + MiniMax authed):
cargo test --test opencode_acp -- --ignored --nocapture
```

The ignored `opencode_acp` test spawns a real `opencode acp` subprocess, runs
`initialize -> session/new -> session/prompt` through the harness's own ACP
client, and asserts a streamed agent message comes back.
