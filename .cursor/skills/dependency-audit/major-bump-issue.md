# Major-bump Forgejo issue body

Fill a repo issue template from `.forgejo/ISSUE_TEMPLATE/` (typically the change or story template shaped for a dependency upgrade). Core sections:

```markdown
Epic: \
Story Points:

---

Deferred major bump from the [{version} dependency audit]({overview-doc-url}). {one-line why it was deferred}.

## Description

Upgrade {scope} from `{currentRangeOrMajor}` to the latest `{targetMajor}.x` release.

- Current: `{currentMajor}.x` (`{requirement in Cargo.toml}`)
- Target: `{targetMajor}.x` (latest at audit time: `{latestVersion}`)
- Crate kind: {dependency | dev-dependency | build-dependency} ({short role})

## Acceptance Criteria

- [ ] `Cargo.toml` / `Cargo.lock` use the target major
- [ ] Behaviour covered by existing tests remains correct
- [ ] `cargo test` and `cargo build` pass (and `cargo clippy` when used)
- [ ] Breaking changes from the major bump are handled or documented
- [ ] `cargo audit` remains clean (or residuals are documented)

## Subtasks

*No subtasks*

## Notes

- Related: #{release-dependency-issue} ({version} Dependency Updates)
- Overview: `docs/dependency-audit-{version}.md`
```

**Title:** `Upgrade {crate} from {currentMajor} to {targetMajor}`

**Labels when filing via Forgejo MCP:** `type/dependencies` + `needs/estimate`
