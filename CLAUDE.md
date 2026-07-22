# CLAUDE.md

Guidance for agents working in this repo.

## Project

Minimalist static site generator in Rust (`src/main.rs`). Converts Markdown
posts under `content/posts/<lang>/` into HTML. Deploys to GitHub Pages via
`.github/workflows/deploy.yml`. Zero JavaScript.

## Writing rule: no signs of AI writing

All prose in `content/` must read as human-written. Do not leave the mechanical
tells catalogued at
<https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing>.

Non-negotiable, enforced by the linter:

- **No em-dashes (`—`) or en-dashes (`–`).** Use a spaced hyphen (` - `),
  a comma, or a colon. Rewrite the sentence if it needs it.
- **Straight quotes only** (`'` and `"`), never curly (`’` `“` `”`).
- **No ellipsis character (`…`).** Write `...`.

Judgement calls (rewrite by hand, the linter only warns):

- Kill filler and marketing register: *delve, leverage, seamless, robust,
  foster, myriad, paradigm, testament to, dive into, moreover, furthermore,
  in conclusion, it is worth noting*.
- No hedging throat-clearing, no "in today's world" openers, no summary
  paragraph that restates what was just said.
- Vary sentence length. Cut adjectives that carry no information.

### Before committing content

Run the linter. It exits non-zero if any auto-fixable tell remains:

```bash
scripts/lint-prose.sh          # check (reports typographic + lexical tells)
scripts/lint-prose.sh --fix    # auto-fix typographic tells in place
```

`--fix` handles dashes/quotes/ellipsis mechanically. Lexical tells are only
flagged - rewriting them is your job, because the fix depends on meaning.
When writing a new post, apply this rule as you write; do not rely on the
linter to catch everything.
