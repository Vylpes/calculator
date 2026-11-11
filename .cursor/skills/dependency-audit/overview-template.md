# Overview document template

Write `docs/dependency-audit-{version}.md` with this structure:

```markdown
# Dependency audit overview — release {version}

**Date:** {YYYY-MM-DD}
**Branch:** `release/{version}`
**Package manager:** Cargo ({rustc/cargo version if known})

## Summary

| Metric | Before | After |
|--------|--------|-------|
| Unique advisories (`cargo audit`) | {n} | {n} |
| Critical | {n} | {n} |
| High | {n} | {n} |
| Medium | {n} | {n} |
| Low | {n} | {n} |
| Unmaintained / other warnings | {n} | {n} |
| Tests / build | — | {result} |

{1–3 sentence narrative of approach and what was deferred.}

## Phase 1 — Vulnerability fixes

### Approach

1. …
2. …
3. …

### Critical / high issues addressed

| Crate | Advisory (examples) | Resolution (`Cargo.toml` bump / `cargo update -p` / `[patch.crates-io]`) |
|-------|---------------------|------------------------------------------------------------------------|
| `…` | … | `…` |

### Patches / direct requirement changes (after)

\`\`\`toml
# Relevant excerpts from Cargo.toml ([dependencies] and/or [patch.crates-io])
{ … }
\`\`\`

## Phase 2 — Non-vulnerable crate updates

### Runtime dependencies

| Crate | From (lock / req) | To |
|-------|-------------------|----|
| `…` | `…` | `…` |

### Dev / build dependencies

| Crate | From (lock / req) | To |
|-------|-------------------|----|
| `…` | `…` | `…` |

### Deferred major upgrades

Tracked under milestone **{nextVersion}** (or the audit release milestone if that is where tickets are filed):

| Crate | Current major | Latest major | Ticket |
|-------|---------------|--------------|--------|
| `…` | … | … | [#N](url) |

## Verification

\`\`\`bash
cargo audit    # 0 vulnerabilities
cargo test
cargo build
# cargo clippy -- -D warnings   # when used
\`\`\`

## Files changed

- `Cargo.toml` — …
- `Cargo.lock` — …
- `docs/dependency-audit-{version}.md` — this overview
```
