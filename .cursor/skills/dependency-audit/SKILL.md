---
name: dependency-audit
description: >-
  Run a release dependency audit for this Rust/Cargo project: fix vulnerabilities,
  apply within-range crate updates, defer majors to Forgejo tickets, write an
  overview doc, and comment it on the release dependency issue. Use when the user
  asks for a dependency audit, cargo audit for a release, vulnerability remediation
  plus crate updates, or to file major-bump tickets after an audit.
---

# Dependency audit (release)

Audit Cargo dependencies for a named release: fix vulns first, then within-range updates, then document and ticket deferred majors.

## Inputs

- **Release version** (required): e.g. `0.1.0` — from the user, branch name `release/X.Y.Z`, or open milestone
- **Package manager**: Cargo (`Cargo.toml` + `Cargo.lock`)
- **Forgejo owner/repo**: from `git remote get-url origin` (`ssh://git@host:port/Owner/repo.git`)

## Prerequisites

- Rust toolchain available (`rustc` / `cargo`)
- **`cargo-audit` installed** where relevant (`cargo install cargo-audit` or distro package). Re-check with `cargo audit --version` before Step 2.
- Optional but useful: `cargo-outdated` (`cargo install cargo-outdated`) for Phase 2 planning

## Workflow

Copy this checklist and track progress:

```
Task Progress:
- [ ] Step 1: Baseline
- [ ] Step 2: Audit and summarize
- [ ] Step 3: Fix vulnerabilities
- [ ] Step 4: Update non-vulnerable packages (within range)
- [ ] Step 5: Verify
- [ ] Step 6: Overview document
- [ ] Step 7: Forgejo tickets for deferred majors
- [ ] Step 8: Comment overview on release dependency issue
- [ ] Step 9: Report back
```

Do **not** commit or push unless the user explicitly asks.

### Step 1: Baseline

1. Confirm branch (prefer `release/{version}`) and clean or known working tree
2. Read `Cargo.toml` (direct `[dependencies]` / `[dev-dependencies]` / `[build-dependencies]`, and any existing `[patch.crates-io]`)
3. Note current direct dependency version requirements and the resolved versions in `Cargo.lock`

### Step 2: Audit and summarize

Run `cargo audit` and summarize by advisory (RustSec id). Prefer unique advisory counts over path-inflated totals.

```bash
cargo audit
# machine-readable when useful:
cargo audit --json
```

Also gather outdated / within-range candidates for Phase 2 planning:

```bash
# Preferred when installed:
cargo outdated

# Always available: preview lockfile bumps within existing Cargo.toml ranges
cargo update --dry-run
```

### Step 3: Fix vulnerabilities

Order of preference:

1. Upgrade **direct** crates in `Cargo.toml` that pull in vulnerable transitive crates (stay on the **same major** when a patched release exists there)
2. Run `cargo update -p <crate>` (and parents as needed) so `Cargo.lock` picks up patched transitive versions still allowed by existing requirements
3. For remaining transitive vulns that cannot be reached via direct bumps, add/expand `[patch.crates-io]` pins to patched crates.io (or git) versions
4. Re-run `cargo audit` until unique advisories are **0** (or document unblockable residuals)

`[patch.crates-io]` may diverge from published graphs; that is acceptable for security if the verify step passes. Prefer removing patches later when upstream parents catch up.

### Step 4: Update non-vulnerable packages (within range)

After the audit is clean:

1. Bump lockfile resolutions to the latest versions still allowed by existing `Cargo.toml` requirements (`cargo update`, or selective `cargo update -p <crate>`)
2. Optionally raise direct requirements in `Cargo.toml` when still on the **same major** (e.g. `4.4` → `4.5`) — **do not** change the major component in this pass
3. **Do not** apply major crate bumps in this pass
4. Record deferred majors (crate, current major, latest major) for Step 7

If a major bump is blocked by policy/review tooling, leave it deferred — do not fight it mid-audit.

### Step 5: Verify

Run the project’s standard Cargo checks:

```bash
cargo audit    # 0 vulnerabilities (or documented residuals)
cargo test
cargo build
# Optional when already used in the repo/CI:
cargo clippy -- -D warnings
```

Fix breakages from patches/upgrades before documenting.

### Step 6: Overview document

Write `docs/dependency-audit-{version}.md` using [overview-template.md](overview-template.md).

Include before/after audit metrics, `[patch.crates-io]` / direct requirement changes, within-range bumps, deferred majors (ticket links once created), verification commands, and files changed.

### Step 7: Forgejo tickets for deferred majors

For **each** deferred major bump, create one issue via Forgejo MCP `create_issue`.

**Defaults** (match existing RabbitLabs patterns; adjust if labels/milestones differ):

| Field | Value |
|-------|--------|
| Milestone | The same milestone as the release this audit was for (e.g. audited `0.1.0` → milestone `0.1.0`) |
| Labels | `type/dependencies` + `needs/estimate` (use org/repo label IDs from existing issues) |
| Body | Project issue template under `.forgejo/ISSUE_TEMPLATE/` filled with description + acceptance criteria (see [major-bump-issue.md](major-bump-issue.md)) |

**Title pattern:** `Upgrade {crate} from {currentMajor} to {targetMajor}`
For tightly coupled pairs (e.g. `gtk4` + `glib`), one combined ticket is fine.

Each body should link:

- The overview doc path / branch URL
- Related release dependency issue if one exists (e.g. `#N` style `{version} Dependency Updates`)

Update the overview doc deferred table with ticket URLs.

### Step 8: Comment overview on release dependency issue

1. Find the open issue for this release’s dependency work (title like `{version} Dependency Updates`, or ask the user)
2. `create_issue_comment` with the **full markdown body** of `docs/dependency-audit-{version}.md` (not just a link)
3. Return the comment URL to the user

### Step 9: Report back

Summarize:

- Audit before → after (unique advisories)
- Key patches / direct upgrades
- Overview doc path
- Major-bump ticket links
- Comment URL on the release dependency issue
- Remind that nothing was committed unless they asked

## Rules

- Vulns first, then within-range updates, then docs/tickets
- No major bumps during the audit pass — ticket them instead
- No commit/push unless explicitly requested
- Prefer deduplicated advisory counts in human summaries
- Use Forgejo MCP (`user-forgejo` / `forgejo`), not `gh`, for this workflow’s tickets/comments
- This skill is Cargo-only for calculator — do not use yarn/npm/pnpm audit or lockfile commands here

## Additional resources

- Overview markdown structure: [overview-template.md](overview-template.md)
- Issue body pattern for majors: [major-bump-issue.md](major-bump-issue.md)
