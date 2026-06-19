---
type: Research Note
title: Fabric reference research
description: Source research used to shape Fabric's governance, OKF, ICM, Claude, and Perplexity conventions.
resource: https://github.com/agent0ai/dox
tags: [fabric, research, references, dox, okf, icm, claude, perplexity]
timestamp: 2026-06-18T00:00:00+08:00
tool: web
---
# Query / Prompt

Research the frameworks and conventions that inspire Fabric: Agent Zero DOX, Karpathy's LLM Wiki pattern, Anthropic Claude Code memory/imports, Google Cloud Open Knowledge Format, Perplexity API key handling, and Van Clief & McDermott's ICM/MWP.

# Key Findings

- Fabric should keep `AGENTS.md` as the governance plane and avoid duplicating rules in `CLAUDE.md`.
- Claude Code can import shared instructions from `CLAUDE.md` with `@AGENTS.md`.
- OKF is a minimal Markdown + YAML-frontmatter knowledge bundle format; non-reserved concept files require a non-empty `type`.
- OKF supports `index.md` for progressive disclosure and `log.md` for date-grouped history.
- Perplexity API keys should be treated as secrets and read from environment variables such as `PERPLEXITY_API_KEY`, not hardcoded.
- ICM/MWP supports staged filesystem workflows with numbered folders and Markdown context files.

# Sources

| Source | URL | Why it matters |
|---|---|---|
| Agent Zero DOX | https://github.com/agent0ai/dox | Source of DOX-style governance and no-orphans discipline. |
| Anthropic Claude Code memory | https://code.claude.com/docs/en/memory | Documents Claude memory files and import syntax. |
| Google Cloud OKF blog | https://cloud.google.com/blog/products/data-analytics/how-the-open-knowledge-format-can-improve-data-sharing/ | Introduces OKF as portable Markdown + YAML knowledge format. |
| OKF spec | https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md | Defines OKF concepts, reserved files, frontmatter, links, indexes, logs, citations, and conformance. |
| Perplexity API key management | https://docs.perplexity.ai/docs/admin/api-key-management | Documents API key secrecy and `PERPLEXITY_API_KEY` environment usage. |
| Karpathy LLM Wiki | https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f | Inspiration for LLM-maintained Markdown knowledge. |
| Van Clief & McDermott ICM/MWP | https://arxiv.org/abs/2603.16021 | Inspiration for staged filesystem context methodology. |

# Reliability / Limits

- OKF is v0.1 draft; keep Fabric permissive and template-driven rather than overfitted to a rigid schema.
- Perplexity provider preference depends on local tooling availability; agents should fall back to available web research tools when necessary.

# Used In

- [Fabric knowledge-capture decision](../decisions/2026-06-18-fabric-knowledge-capture.md)
- [Context indexes reduce hallucinated structure](../learnings/2026-06-18-context-indexes-prevent-orphans.md)

# Citations

[1] [Agent Zero DOX](https://github.com/agent0ai/dox)  
[2] [Anthropic Claude Code memory](https://code.claude.com/docs/en/memory)  
[3] [Google Cloud OKF blog](https://cloud.google.com/blog/products/data-analytics/how-the-open-knowledge-format-can-improve-data-sharing/)  
[4] [OKF spec](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)  
[5] [Perplexity API key management](https://docs.perplexity.ai/docs/admin/api-key-management)  
[6] [Karpathy LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f)  
[7] [Van Clief & McDermott ICM/MWP](https://arxiv.org/abs/2603.16021)
