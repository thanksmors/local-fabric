---
type: Learning Note
title: Context indexes reduce hallucinated structure
description: Empty but shaped indexes and templates reduce agent invention while preserving no-orphans navigation.
resource: /learnings/2026-06-18-context-indexes-prevent-orphans.md
tags: [fabric, no-orphans, indexes, templates]
timestamp: 2026-06-18T00:00:00+08:00
---
# Learning

Agents are less likely to invent folder structures when the base repo contains minimal, shaped control files and templates.

# Applies To

- Root `AGENTS.md` top-level artifact index.
- Root `CONTEXT.md` Stage Index.
- `knowledge/index.md` and directory-level OKF indexes.
- `.fabric/templates/` starter files.

# Why It Matters

The no-orphans rule needs visible parent surfaces. Empty tables with `_None yet_` rows communicate both the required structure and the instruction not to create speculative artifacts.

# Evidence

- [Fabric reference research](../research/2026-06-18-fabric-references.md)
- [Fabric knowledge-capture decision](../decisions/2026-06-18-fabric-knowledge-capture.md)

# Reuse Guidance

When extending Fabric, add the smallest owning index or template needed before asking agents to generate new artifact types.
