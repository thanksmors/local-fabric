#!/usr/bin/env bash
set -u

status=0
fail() {
  printf 'FABRIC CHECK: %s\n' "$1" >&2
  status=1
}

# Required base files.
for f in AGENTS.md CLAUDE.md CONTEXT.md knowledge/index.md knowledge/research/index.md knowledge/decisions/index.md knowledge/learnings/index.md knowledge/plans/index.md .fabric/templates/index.md scripts/fabric-check.sh .env.example .gitignore; do
  [ -f "$f" ] || fail "missing $f"
done

# Secret safety.
if [ -f .gitignore ]; then
  grep -qE '^\.env$' .gitignore || fail '.gitignore should ignore .env'
  grep -qE '^\.env\.\*$' .gitignore || fail '.gitignore should ignore .env.* variants'
  grep -qE '^!\.env\.example$' .gitignore || fail '.gitignore should allow .env.example'
fi
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  if git ls-files --error-unmatch .env >/dev/null 2>&1; then
    fail '.env appears to be tracked by git; remove it from version control'
  fi
fi
if [ -f .env ]; then
  printf 'FABRIC CHECK: note: .env exists locally; do not commit or print its contents\n' >&2
fi

# Claude shim.
if [ -f CLAUDE.md ] && ! grep -q '^@AGENTS.md[[:space:]]*$' CLAUDE.md; then
  fail 'CLAUDE.md should import AGENTS.md with @AGENTS.md'
fi

# Root artifact links.
for path in CLAUDE.md CONTEXT.md knowledge/index.md knowledge/research/index.md knowledge/decisions/index.md knowledge/learnings/index.md knowledge/plans/index.md .fabric/templates/index.md scripts/fabric-check.sh .fabric/test-prompts.md .env.example .gitignore; do
  if [ -e "$path" ] && ! grep -q "$path" AGENTS.md 2>/dev/null; then
    fail "root AGENTS.md does not list/link $path"
  fi
done

# Child AGENTS.md indexing.
while IFS= read -r f; do
  d=$(dirname "$f")
  p=$(dirname "$d")
  while [ ! -f "$p/AGENTS.md" ] && [ "$p" != "." ] && [ "$p" != "/" ]; do
    p=$(dirname "$p")
  done
  if [ -f "$p/AGENTS.md" ] && ! grep -q "$d" "$p/AGENTS.md" 2>/dev/null; then
    fail "orphan child AGENTS.md: $f not indexed in $p/AGENTS.md"
  fi
done < <(find . -mindepth 2 -name AGENTS.md -not -path '*/node_modules/*' -not -path '*/.git/*' | sort)

# ICM stages.
if [ -d stages ]; then
  [ -f CONTEXT.md ] || fail 'stages/ exists but CONTEXT.md is missing'
  while IFS= read -r d; do
    [ -f "$d/CONTEXT.md" ] || { fail "stage missing CONTEXT.md: $d"; continue; }
    grep -q "$d/CONTEXT.md\|$d" CONTEXT.md 2>/dev/null || fail "stage not indexed in CONTEXT.md: $d"
    grep -q '../../CONTEXT.md\|../CONTEXT.md\|CONTEXT.md' "$d/CONTEXT.md" 2>/dev/null || fail "stage does not link back to root CONTEXT.md: $d/CONTEXT.md"
    grep -q 'AGENTS.md' "$d/CONTEXT.md" 2>/dev/null || fail "stage does not link to AGENTS.md: $d/CONTEXT.md"
  done < <(find stages -mindepth 1 -maxdepth 1 -type d -name '[0-9][0-9]_*' 2>/dev/null | sort)
fi

# OKF shape.
if [ -d knowledge ]; then
  [ -f knowledge/index.md ] || fail 'knowledge/ exists but knowledge/index.md is missing'
  grep -q '../AGENTS.md' knowledge/index.md 2>/dev/null || fail 'knowledge/index.md should link to ../AGENTS.md'
  for sub in research decisions learnings plans; do
    [ -f "knowledge/$sub/index.md" ] || fail "missing knowledge/$sub/index.md"
    grep -q "../index.md" "knowledge/$sub/index.md" 2>/dev/null || fail "knowledge/$sub/index.md should link to ../index.md"
    grep -q "knowledge/$sub/index.md\|$sub/index.md" knowledge/index.md 2>/dev/null || fail "knowledge/index.md does not link knowledge/$sub/index.md"
  done

  index_files=$(find knowledge -type f -name index.md | sort)
  while IFS= read -r f; do
    case "$(basename "$f")" in
      index.md|log.md) continue ;;
    esac
    if ! awk 'BEGIN{in_fm=0; seen_type=0} NR==1 && $0=="---"{in_fm=1; next} in_fm && $0=="---"{exit} in_fm && $0 ~ /^type:[[:space:]]*[^[:space:]]/{seen_type=1} END{exit seen_type?0:1}' "$f"; then
      fail "OKF concept missing non-empty type: $f"
    fi
    rel=${f#knowledge/}
    base=$(basename "$f")
    found=0
    while IFS= read -r idx; do
      if grep -q "$rel\|$base" "$idx" 2>/dev/null; then
        found=1
        break
      fi
    done <<EOF_INDEXES
$index_files
EOF_INDEXES
    [ "$found" -eq 1 ] || fail "OKF concept not linked from an index: $f"
  done < <(find knowledge -type f -name '*.md' | sort)
fi

# Template index.
if [ -f .fabric/templates/index.md ]; then
  for t in child-AGENTS.md stage-CONTEXT.md okf-concept.md okf-directory-index.md okf-log.md brainstorm.md implementation-plan.md research-note.md decision-record.md learning-note.md; do
    [ -f ".fabric/templates/$t" ] || fail "missing template: .fabric/templates/$t"
    grep -q "$t" .fabric/templates/index.md || fail "template not indexed: $t"
  done
fi

if [ "$status" -eq 0 ]; then
  printf 'FABRIC CHECK: ok\n'
fi
exit "$status"
