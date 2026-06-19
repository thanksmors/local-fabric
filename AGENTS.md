# AGENTS.md — Fabric Root Contract

**Fabric** is this repo's linked context system for AI agents. It combines DOX-style governance, Claude-compatible entrypoints, ICM-style workflow context, OKF-style durable knowledge, YAGNI artifact gating, lightweight brainstorming/planning, and research/decision/learning/plan memory without duplicating rules.

Fabric is not a runtime, protocol, or standard. It is a repo convention inspired by Agent Zero DOX, Andrej Karpathy's LLM Wiki pattern, Anthropic Claude Code memory/import conventions, Google Cloud Open Knowledge Format, Perplexity API research workflows, Obra's Superpowers brainstorming/planning skills, the YAGNI Manifesto, and Van Clief & McDermott's ICM/MWP. See [References and Inspirations](#references-and-inspirations).

## Precedence

1. Direct user instructions in the current session
2. Nearest `AGENTS.md` for local work details
3. Parent `AGENTS.md` files up to this root
4. ICM `CONTEXT.md` files for workflow-stage instructions
5. OKF files in `knowledge/` for durable facts, research, decisions, and learnings
6. External references and inspirations

No child doc, workflow doc, knowledge file, template, or external reference may weaken this root Behavioral Core, DOX rules, security rules, or No-Orphans rules.

## Fabric Context Artifact Index

| Plane | Artifact | Status | Purpose |
|---|---|---:|---|
| Governance | [`AGENTS.md`](./AGENTS.md) | present | Root Fabric contract and single source of truth for agent behavior. |
| Tool entrypoint | [`CLAUDE.md`](./CLAUDE.md) | present | Claude Code import shim; normally exactly `@AGENTS.md`. |
| Workflow / ICM | [`CONTEXT.md`](./CONTEXT.md) | present, empty | Root ICM router and Stage Index owner. No stages are active until indexed there. |
| Knowledge / OKF | [`knowledge/index.md`](./knowledge/index.md) | present | Root OKF bundle index and Knowledge Index owner. |
| Research memory | [`knowledge/research/index.md`](./knowledge/research/index.md) | present | External web/Perplexity research notes and source summaries. |
| Decision memory | [`knowledge/decisions/index.md`](./knowledge/decisions/index.md) | present | Decision records with context, options, justification, and consequences. |
| Learning memory | [`knowledge/learnings/index.md`](./knowledge/learnings/index.md) | present | Reusable lessons learned from work and research. |
| Plan memory | [`knowledge/plans/index.md`](./knowledge/plans/index.md) | present | Durable implementation plans, design plans, and handoff plans. |
| Templates | [`.fabric/templates/index.md`](./.fabric/templates/index.md) | present | Canonical templates for child `AGENTS.md`, ICM stages, OKF concepts, brainstorming, plans, research, decisions, and learnings. |
| Verification | [`scripts/fabric-check.sh`](./scripts/fabric-check.sh) | present | Best-effort no-orphans, OKF shape, and secret-safety check. |
| Test prompts | [`.fabric/test-prompts.md`](./.fabric/test-prompts.md) | present | Prompts for validating Fabric behavior in a fresh repo. |
| Local secrets template | [`.env.example`](./.env.example) | present | Documents supported local environment variables without storing secrets. |
| Ignore rules | [`.gitignore`](./.gitignore) | present | Ensures `.env` and local secret variants are not committed. |

Empty scaffold files are intentional. They provide concrete table shapes and rules so agents do not invent formats, but they do not imply that a real workflow, stage, or project-specific concept already exists.

## Behavioral Core

Applies to all work in all subtrees. Bias toward caution over speed; use judgment on trivial tasks.

### Think Before Coding
- State assumptions explicitly. If uncertain, ask before implementing unless the task is already clear enough to proceed.
- If multiple interpretations exist, present them; do not pick silently.
- If a simpler approach exists, say so. Push back when warranted.

### Simplicity First
- Minimum code or documentation that solves the problem. Nothing speculative.
- No unrequested features, abstractions, configurability, or error handling for impossible cases.
- Test: "Would a senior engineer call this overcomplicated?" If yes, simplify.

### YAGNI Gate

YAGNI is a first-class Fabric governor: build what is needed, keep change cheap, and delay unused behavior while preserving clean seams for real change. YAGNI is not anti-design; quality, tests, clear names, and boundaries still matter.

Apply the YAGNI Gate before creating or expanding code, docs, workflows, child contracts, OKF concepts, ICM stages, templates, scripts, configuration, abstractions, or frameworks.

Before adding durable structure, answer:

1. Who needs this today?
2. What current task, user need, operational risk, security requirement, or repo evidence requires it?
3. What cost does this add?
4. What is the cost of delaying it?
5. Can a smaller seam, link, table row, note, or local convention solve the present need?

Use the Rule of Three:

- First real case: solve directly.
- Second real case: tolerate some duplication if the future is still unclear.
- Third real case: extract the shared abstraction, template, workflow, child contract, or knowledge pattern.

Build anyway when waiting would create high migration cost, data loss, security/privacy/compliance risk, auditability gaps, broken public contracts, or when there are multiple real consumers or a funded roadmap with specific dates.

Record non-trivial YAGNI exceptions in `knowledge/decisions/` with the current owner, current use case, cost of delay, tradeoffs, and review trigger.

### Surgical Changes
- Touch only what the request requires. Match existing style.
- Do not refactor, improve, or reformat adjacent code without need.
- Remove only orphans your own change created. Mention pre-existing dead code; do not delete it unless asked.
- Scope exception: inside `AGENTS.md`, `CONTEXT.md`, OKF, and Fabric template files, stale or contradictory durable text must be deleted immediately.

### Goal-Driven Execution
- Turn tasks into verifiable goals.
- For multi-step work, state a brief plan with verification per step, then loop until checks pass.

## Personal Preferences

These preferences are durable user-level defaults for this repo. Add scoped preferences to child `AGENTS.md` files only when they apply to a durable subtree instead of the whole repo.

### Research Provider Preference
- When external research is needed for repo work, prefer the Perplexity API if local tooling supports it and `PERPLEXITY_API_KEY` is available from `.env` or the process environment.
- Load `.env` only as a local secret source. Never print, log, commit, copy, or summarize secret values.
- Do not create `.env` with real values. Use [`.env.example`](./.env.example) as the public template.
- If Perplexity is unavailable, use the best available web research tool and state the fallback in the research note.

### Research Capture Preference
- Any web or Perplexity research used for repo work must be captured in OKF before closeout.
- Store research notes under [`knowledge/research/`](./knowledge/research/) using [`.fabric/templates/research-note.md`](./.fabric/templates/research-note.md).
- Update [`knowledge/research/index.md`](./knowledge/research/index.md) in the same change.
- Capture query/prompt, date, tool, sources, key findings, reliability limits, and what the research informed.
- Apply YAGNI to research capture: preserve durable source-backed memory, but do not turn trivial lookups into essays or broad speculative docs.
- Do not copy long source passages. Summarize, cite, and link.

### Learning, Decision, and Plan Memory
- Durable decisions belong under [`knowledge/decisions/`](./knowledge/decisions/) using [`.fabric/templates/decision-record.md`](./.fabric/templates/decision-record.md).
- Reusable learnings belong under [`knowledge/learnings/`](./knowledge/learnings/) using [`.fabric/templates/learning-note.md`](./.fabric/templates/learning-note.md).
- Durable implementation plans, design plans, and handoff plans belong under [`knowledge/plans/`](./knowledge/plans/) using [`.fabric/templates/implementation-plan.md`](./.fabric/templates/implementation-plan.md).
- Decision records must include context, options considered, decision, justification, tradeoffs, consequences, and review triggers.
- Link decisions and plans to supporting research notes, source files, issues, PRs, or human instructions.
- Record a learning when a discovered constraint, failure mode, implementation pattern, or project convention is likely to matter again.

### Brainstorming and Planning Preference
- Use lightweight brainstorming for ambiguous product, design, architecture, workflow, planning, or behavior changes.
- Prefer numbered choices when asking the user to decide. Include tradeoffs and mark one option as **Recommended**.
- Default choice format:
  1. Option A — concise description and tradeoff.
  2. Option B — concise description and tradeoff.
  3. Option C — concise description and tradeoff.

  Recommended: Option N, because `<reason>`.
- Ask one question at a time when clarification is needed. Prefer multiple-choice questions when the answer space is bounded; use open-ended questions when it is not.
- Do not brainstorm ceremonially for trivial, mechanical, or already-specified tasks.
- Before non-trivial, risky, durable, or multi-step implementation, present the smallest useful plan and get approval when user intent is not already explicit.
- Save plans only when durable, requested, or needed for handoff. Short inline plans do not need OKF capture.
- Raw brainstorming usually should not become permanent OKF memory. Capture the durable result instead: a decision, implementation plan, research note, or learning.
- Use [`.fabric/templates/brainstorm.md`](./.fabric/templates/brainstorm.md) for complex choice framing and [`.fabric/templates/implementation-plan.md`](./.fabric/templates/implementation-plan.md) for durable plans.

## DOX: Agent Zero Documentation Governance

Fabric uses DOX-style governance from Agent Zero: a hierarchy of `AGENTS.md` files that agents read before editing and update when durable project context changes.

`AGENTS.md` files are binding contracts for their subtrees. Work products, instructions, and durable docs must stay understandable from the nearest applicable `AGENTS.md` plus every parent above it.

### Read Before Editing
1. Read this root doc.
2. Identify paths you expect to touch; walk root → target, reading every `AGENTS.md` on each route.
3. Read `CONTEXT.md` before touching ICM artifacts or staged workflow outputs.
4. Read `knowledge/index.md` before touching OKF artifacts.
5. Read the relevant `knowledge/research/`, `knowledge/decisions/`, `knowledge/learnings/`, or `knowledge/plans/` index before touching those memories.
6. Re-read the applicable chain in the current session. Do not rely on memory.

### Update After Editing

Before closeout, update the closest owning `AGENTS.md` when a change affects purpose, scope, ownership, responsibilities, durable structure, contracts, workflows, operating rules, required inputs/outputs, permissions, constraints, side effects, artifacts, or durable user preferences.

Also update owning indexes when changing `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`, `stages/`, `knowledge/`, `.fabric/templates/`, `scripts/fabric-check.sh`, `.env.example`, `.gitignore`, or their index entries. Pure code edits that change no durable contract leave docs unchanged — but say so explicitly at closeout.

### Child Docs
- Create a child `AGENTS.md` only when a folder is a durable boundary with its own purpose, rules, responsibilities, or quality standards.
- Start from [`.fabric/templates/child-AGENTS.md`](./.fabric/templates/child-AGENTS.md).
- Section order: Purpose, Ownership, Local Contracts, Work Guidance, Verification, Child DOX Index.
- Work Guidance and Verification stay empty until real standards or checks exist.
- Creating, moving, renaming, or deleting a child `AGENTS.md` requires updating the parent Child DOX Index in the same change.

## Index Ownership

Root `AGENTS.md` owns only top-level Fabric navigation, child governance navigation, and durable user preferences.

| Index | Owner | Lists |
|---|---|---|
| Fabric Context Artifact Index | `AGENTS.md` | Top-level Fabric surfaces and support files. |
| Child DOX Index | nearest parent `AGENTS.md` | Child `AGENTS.md` files only. |
| Stage Index | `CONTEXT.md` | Existing `stages/<NN_slug>/CONTEXT.md` files. |
| Knowledge Index | `knowledge/index.md` | OKF top-level concept groups and directory indexes. |
| Research Index | `knowledge/research/index.md` | External research notes and source summaries. |
| Decision Index | `knowledge/decisions/index.md` | Decision records and decision log. |
| Learning Index | `knowledge/learnings/index.md` | Reusable learning notes. |
| Plan Index | `knowledge/plans/index.md` | Durable implementation plans, design plans, and handoff plans. |
| Template Index | `.fabric/templates/index.md` | Fabric template files. |

Do not duplicate full ICM stage lists or OKF concept catalogs in root `AGENTS.md`; link to the owning index instead. Keep empty table skeletons with a single `_None yet_` row until real entries exist.

## No Orphans

Every durable context artifact must be reachable from a parent control surface and link back to its governing context when practical.

Parent and back-link rules:
- `CLAUDE.md` → imports this root `AGENTS.md` with `@AGENTS.md`; listed in the Fabric Context Artifact Index.
- Child `AGENTS.md` → listed in nearest ancestor Child DOX Index.
- `CONTEXT.md` → listed in the Fabric Context Artifact Index and links back to `AGENTS.md`.
- ICM stage folders → listed in `CONTEXT.md` Stage Index.
- Stage `CONTEXT.md` files → link to root `CONTEXT.md` and nearest governing `AGENTS.md`.
- `knowledge/index.md` → listed in the Fabric Context Artifact Index and links back to `AGENTS.md`.
- OKF concept files → linked from `knowledge/index.md` or a linked directory `index.md`.
- Research notes → linked from `knowledge/research/index.md`; if they support a decision or learning, link both ways when useful.
- Decision records → linked from `knowledge/decisions/index.md`; link to supporting research and justification.
- Learning notes → linked from `knowledge/learnings/index.md`; link to source work, research, or decisions when useful.
- Plan records → linked from `knowledge/plans/index.md`; link to supporting research, decisions, source files, verification, and handoff context when useful.
- Fabric templates → listed in `.fabric/templates/index.md`.
- Durable ICM outputs promoted to docs or knowledge → linked from the owning `CONTEXT.md`, `knowledge/index.md`, or nearest `AGENTS.md`.
- `.env.example` and `.gitignore` → listed in the Fabric Context Artifact Index; `.env` must remain untracked.

Run before closeout when context artifacts change:

```bash
bash scripts/fabric-check.sh
```

## Context Architecture

Fabric has three planes:
- **Governance:** `AGENTS.md` and child `AGENTS.md` files.
- **Workflow:** `CONTEXT.md` and optional `stages/` ICM workspace.
- **Knowledge:** `knowledge/` OKF bundle, including research, decisions, learnings, and durable plans.

The base scaffold ships with `CONTEXT.md`, `knowledge/index.md`, and OKF subindexes so agents see the intended shape. These files are cheap seams, not permission to create speculative content. Add real stages, child contracts, scripts, templates, or project concepts only when they pass the YAGNI Gate, reduce future context assembly, clarify a repeatable workflow, or the user explicitly asks.

### ICM Rules

Use ICM for repeatable multi-step workflows where stage boundaries, human review, or intermediate artifacts matter.

Rules:
- `CONTEXT.md` owns the Stage Index.
- Pass the YAGNI Gate before creating `stages/`. If the workflow is uncertain, start with a checklist or note in `CONTEXT.md`; promote to numbered stages only after repetition, real handoffs, human review points, or explicit user request.
- Do not create `stages/` unless there is at least one real repeatable stage.
- Each stage does one job and has its own `stages/<NN_slug>/CONTEXT.md`.
- Start stage files from [`.fabric/templates/stage-CONTEXT.md`](./.fabric/templates/stage-CONTEXT.md).
- Stage-local `output/` contains working artifacts. Durable outputs promoted to docs or OKF must be linked from the owning index.
- Stable references live in `_config/`, `shared/`, or stage-local `references/` only when they serve more than one run or stage.

### OKF Rules

Use OKF for durable reusable knowledge: architecture, APIs, schemas, domain terms, decisions, runbooks, workflows, datasets, metrics, research notes, source summaries, learnings, implementation plans, handoff plans, or external resources.

Rules:
- `knowledge/index.md` owns the Knowledge Index.
- Pass the YAGNI Gate before adding project-specific concepts: durable, reusable knowledge earns a concept; transient observations do not.
- `knowledge/index.md` is an OKF reserved index file, not a concept; only the bundle-root index may include `okf_version` frontmatter.
- Every non-reserved OKF concept file under `knowledge/` must contain YAML frontmatter with a non-empty `type`.
- Recommended concept fields: `title`, `description`, `resource`, `tags`, `timestamp`.
- Producer-specific fields are allowed when useful, especially for `tool`, `status`, `decision_date`, or `review_trigger`.
- Start concept files from the closest matching template in [`.fabric/templates/`](./.fabric/templates/).
- Use normal Markdown links. Prefer one consistent style inside `knowledge/`: bundle-root links like `/architecture/service-map.md` or relative links.
- Use directory `index.md` files for progressive disclosure when a folder has multiple concepts.
- Use `log.md` for meaningful bundle or directory history, especially decisions.
- Do not duplicate binding agent rules from `AGENTS.md`; OKF describes knowledge, not authority.
- Do not generate unsupported facts. Generated OKF concepts should include citations to source files, docs, URLs, or human-provided context.

### Research, Decision, Learning, and Plan Capture

When research or meaningful reasoning informs repo work:

1. Create or update a research note in `knowledge/research/`.
2. Create or update a decision record in `knowledge/decisions/` when a durable choice was made.
3. Create or update a learning note in `knowledge/learnings/` when the insight is likely reusable.
4. Create or update a plan record in `knowledge/plans/` when an implementation/design/handoff plan is durable, requested, or needed by another agent.
5. Use the smallest durable OKF record that preserves future value; avoid diary-style notes and raw brainstorm transcripts.
6. Update the relevant directory index and, when meaningful, `knowledge/log.md`, `knowledge/decisions/log.md`, or `knowledge/plans/log.md`.
7. Link related notes bidirectionally where it helps future agents reconstruct context.

Every speculative feature, abstraction, workflow, or context artifact must name its current owner, current use case, and expected cost of delay. If those cannot be named, do not build it.

## References and Inspirations

- [Agent Zero DOX](https://github.com/agent0ai/dox) — source of the DOX pattern: root/child `AGENTS.md` hierarchy, read-before-edit traversal, update-after-change, and no orphan child docs.
- [AGENTS.md convention](https://agents.md/) — common project instruction file for coding agents.
- [Andrej Karpathy's LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) — persistent Markdown wiki maintained by an LLM with a schema file such as `CLAUDE.md` or `AGENTS.md`.
- [Anthropic Claude Code memory/imports](https://code.claude.com/docs/en/memory) — `CLAUDE.md` imports shared instructions with `@AGENTS.md`.
- [Google Cloud Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md) — Markdown + YAML-frontmatter knowledge bundles for humans and agents.
- [Perplexity API key management](https://docs.perplexity.ai/docs/admin/api-key-management) — API keys should be stored securely, commonly in `PERPLEXITY_API_KEY`, never hardcoded or exposed.
- [Obra Superpowers skills](https://github.com/obra/superpowers/tree/main/skills) — inspiration for lightweight brainstorming, numbered alternatives with a recommendation, and implementation planning.
- [YAGNI Manifesto summary](./knowledge/research/2026-06-19-yagni-manifesto.md) — build what is needed, keep change cheap, and record speculative tradeoffs.
- [Van Clief & McDermott ICM/MWP](https://arxiv.org/abs/2603.16021) — filesystem-based staged workflows using numbered folders, Markdown context files, and local scripts.

## Closeout

1. Re-check changed paths against the DOX chain.
2. Confirm any new durable structure passed the YAGNI Gate, or record the exception in `knowledge/decisions/`.
3. Update nearest owning docs and affected parents/children.
4. Refresh affected Child DOX Index, Stage Index, Knowledge Index, Template Index, and Fabric Context Artifact Index.
5. Capture any external research used for repo work in `knowledge/research/`.
6. Capture durable decisions, reusable learnings, and durable plans when applicable.
7. Run `bash scripts/fabric-check.sh` if any context artifact changed.
8. Run existing verification when relevant.
9. Report docs intentionally left unchanged and why.

## Child DOX Index

| Child contract | Scope | Purpose | Notes |
|---|---|---|---|
| [`app/AGENTS.md`](./app/AGENTS.md) | `app/` | Local-first agent harness: Tauri + Svelte app wrapping agent CLIs over ACP. | Built per the [v1 plan](./knowledge/plans/2026-06-19-local-agent-harness-v1.md). |
