---
type: Implementation Plan
title: Bake Lightweight Brainstorming and Planning into Fabric
description: Plan for adding Superpowers-inspired, YAGNI-limited brainstorming and planning behavior to Fabric.
resource: ./knowledge/plans/2026-06-19-fabric-brainstorming-planning.md
tags: [fabric, brainstorming, planning, superpowers, yagni]
timestamp: 2026-06-19T00:00:00Z
status: completed
---
# Bake Lightweight Brainstorming and Planning into Fabric

Governed by: [../../AGENTS.md](../../AGENTS.md)  
Knowledge root: [../index.md](../index.md)  
Plan index: [index.md](./index.md)  
Supporting research: [../research/2026-06-19-superpowers-brainstorming-plans.md](../research/2026-06-19-superpowers-brainstorming-plans.md)  
Supporting decision: [../decisions/2026-06-19-lightweight-brainstorming-planning.md](../decisions/2026-06-19-lightweight-brainstorming-planning.md)  
Related learning: [../learnings/2026-06-19-brainstorming-outputs-not-noise.md](../learnings/2026-06-19-brainstorming-outputs-not-noise.md)

## Goal

Add lightweight brainstorming and implementation planning to Fabric without importing the full Superpowers process or creating speculative ceremony.

## YAGNI Gate

- Current need: The user wants numbered choices with a recommended option and wants planning support baked into Fabric.
- Current owner/consumer: Future agents working in Fabric repos and the repo owner reviewing decisions.
- Cost added: One plan index, two templates, a few AGENTS rules, and one starter plan record.
- Cost of delay: Future agents may invent inconsistent brainstorming/planning formats or over-import Superpowers.
- Smallest useful plan chosen: Behavior preference + templates + `knowledge/plans/`, not a full process framework.

## Options Considered

1. **Behavior-only rule** — lowest footprint, but agents may still invent plan formats.
2. **Behavior + templates + `knowledge/plans/`** — small stable seam and examples for durable plans.
3. **Full Superpowers import** — powerful but too heavy for Fabric and contrary to YAGNI.

**Recommended / Chosen:** Option 2, because it gives agents concrete examples and durable plan memory while avoiding mandatory ceremony.

## Tasks

### Task 1: Update Fabric governance

**Files:**
- `AGENTS.md`

**Steps:**
- [x] Add Brainstorming and Planning Preference.
- [x] Add Plan memory to the Fabric Context Artifact Index.
- [x] Add Plan Index ownership and no-orphan rules.
- [x] Add Obra Superpowers as an inspiration/reference.

**Verification:** Root `AGENTS.md` links `knowledge/plans/index.md`, `.fabric/templates/brainstorm.md`, and `.fabric/templates/implementation-plan.md`.

### Task 2: Add plan memory and templates

**Files:**
- `knowledge/plans/index.md`
- `knowledge/plans/log.md`
- `.fabric/templates/brainstorm.md`
- `.fabric/templates/implementation-plan.md`
- `.fabric/templates/index.md`

**Steps:**
- [x] Create a plan index with an empty-or-populated table shape.
- [x] Add templates that encode numbered options and a recommended choice.
- [x] Update the template index.

**Verification:** `bash scripts/fabric-check.sh` passes.

### Task 3: Capture supporting memory

**Files:**
- `knowledge/research/2026-06-19-superpowers-brainstorming-plans.md`
- `knowledge/decisions/2026-06-19-lightweight-brainstorming-planning.md`
- `knowledge/learnings/2026-06-19-brainstorming-outputs-not-noise.md`

**Steps:**
- [x] Capture external research used for the change.
- [x] Record the durable decision and rejected alternatives.
- [x] Record the reusable learning about capturing outputs, not raw brainstorms.

**Verification:** Each file has OKF frontmatter and is linked from its owning index.

### Task 4: Update validation and tests

**Files:**
- `scripts/fabric-check.sh`
- `.fabric/test-prompts.md`

**Steps:**
- [x] Extend Fabric checker for `knowledge/plans/` and new templates.
- [x] Add test prompts for brainstorming, planning, and YAGNI-safe behavior.

**Verification:** `bash scripts/fabric-check.sh` prints `FABRIC CHECK: ok`.
