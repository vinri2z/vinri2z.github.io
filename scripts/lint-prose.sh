#!/usr/bin/env bash
#
# lint-prose.sh - strip mechanical "signs of AI writing" from repo content.
#
# Two classes of tell:
#   1. Typographic (auto-fixable): em-dash, en-dash, curly quotes, ellipsis char.
#      Replaced with plain ASCII equivalents. Checked across all shipped text:
#      Markdown posts, static HTML pages, and the prose baked into src/main.rs.
#      (No language here uses these glyphs as syntax, so replacing them is safe.)
#   2. Lexical (flagged only): overused AI filler words. Checked in prose files
#      (Markdown + HTML) only, to avoid false positives on code identifiers.
#      Rewriting these needs human judgement, so the script never edits them.
#
# Usage:
#   scripts/lint-prose.sh            # check mode: report tells, exit 1 if any typographic
#   scripts/lint-prose.sh --fix      # fix typographic tells in place
#
# Reference: https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FIX=0
[[ "${1:-}" == "--fix" ]] && FIX=1

# Lexical tells: word-boundary, case-insensitive. Warned, never auto-edited.
LEXICAL='delve|tapestry|testament to|realm of|boasts|nestled|dive into|game.?changer|moreover|furthermore|leverage|seamless|meticulous|underscore|foster|myriad|plethora|pivotal|holistic|paradigm|embark|multifaceted|in conclusion|it is worth noting|in today.?s world'

TYPO_RE='[\x{2014}\x{2013}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}]'

# collect_files <base-dir> <ext...> -> populates global `files` array
collect() {
  files=()
  while IFS= read -r line; do files+=("$line"); done < <("$@")
}

typo_files() {
  find "$ROOT/content" -type f -name '*.md'
  find "$ROOT/static" -type f -name '*.html' 2>/dev/null || true
  find "$ROOT/src" -type f -name '*.rs' 2>/dev/null || true
}
prose_files() {
  find "$ROOT/content" -type f -name '*.md'
  find "$ROOT/static" -type f -name '*.html' 2>/dev/null || true
}

typo_hits=0
lex_hits=0

# --- typographic pass (all shipped text) ---
collect typo_files
for f in "${files[@]}"; do
  if [[ $FIX -eq 1 ]]; then
    perl -CSD -i -pe 's/\s*[\x{2014}\x{2013}]\s*/ - /g' "$f"           # dashes -> spaced hyphen
    perl -CSD -i -pe 's/[\x{2018}\x{2019}]/'"'"'/g; s/[\x{201C}\x{201D}]/"/g' "$f"  # curly -> straight
    perl -CSD -i -pe 's/\x{2026}/.../g' "$f"                           # ellipsis -> dots
  else
    if perl -CSD -ne "exit 1 if /$TYPO_RE/" "$f"; then :; else
      echo "typo: ${f#$ROOT/}"
      perl -CSD -ne "print \"  \$.: \$_\" if /$TYPO_RE/" "$f"
      typo_hits=$((typo_hits+1))
    fi
  fi
done

# --- lexical pass (prose files only, always report) ---
collect prose_files
for f in "${files[@]}"; do
  if grep -niEq "\b($LEXICAL)\b" "$f"; then
    echo "warn (lexical, review by hand): ${f#$ROOT/}"
    grep -niE "\b($LEXICAL)\b" "$f" | sed 's/^/  /'
    lex_hits=$((lex_hits+1))
  fi
done

if [[ $FIX -eq 1 ]]; then
  echo "fixed typographic tells across content/, static/, src/."
  [[ $lex_hits -gt 0 ]] && echo "note: $lex_hits file(s) still have lexical tells to review."
  exit 0
fi

echo
echo "$typo_hits file(s) with auto-fixable typographic tells (run: scripts/lint-prose.sh --fix)"
echo "$lex_hits file(s) with lexical tells (rewrite by hand)"
[[ $typo_hits -gt 0 ]] && exit 1
echo "clean: no typographic tells."
