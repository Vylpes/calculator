# Test Plan: #24 — AS a user, I want to be able to cycle the current session's history

**Issue:** https://git.vylpes.xyz/RabbitLabs/calculator/issues/24
**Generated:** 2026-06-29
**Milestone:** 0.1.0

## Summary

Verify that interactive CLI mode maintains a per-session input history and lets the user recall prior expressions with the Up and Down arrow keys to edit and re-run previous calculations.

## Test Cases

| Id | Description | Expected Results | Actual Results | Notes |
|----|-------------|------------------|----------------|-------|
| 24-0001 | Start interactive CLI (`calculator cli`). Enter and submit `2 + 3`, then `10 - 4`. At a new prompt, press Up once | Prompt shows `10 - 4` | | Acceptance criterion — Up recalls most recent entry |
| 24-0002 | Continuing from 24-0001, press Up again | Prompt shows `2 + 3` | | Up cycles further back in history |
| 24-0003 | Continuing from 24-0002, press Down once | Prompt shows `10 - 4` | | Acceptance criterion — Down cycles forward |
| 24-0004 | Continuing from 24-0003, press Down again | Prompt shows an empty input line (or latest/new entry position) | | Down reaches the end of history |
| 24-0005 | Start a fresh interactive session. Enter and submit three expressions in order: `1 + 1`, `2 + 2`, `3 + 3`. Press Up repeatedly until history stops changing | History cycles backward through `3 + 3`, then `2 + 2`, then `1 + 1` in order | | Multi-entry backward traversal |
| 24-0006 | From the oldest recalled entry in 24-0005, press Down repeatedly until history stops changing | History cycles forward back through `2 + 2`, then `3 + 3`, then empty/latest position | | Multi-entry forward traversal |
| 24-0007 | Enter and submit `5 * 2`. Press Up to recall `5 * 2`, change it to `5 * 3`, and submit | Result shows `= 15` | | Recalled expression can be edited and re-run |
| 24-0008 | Enter and submit `4 + 4`. Press Up to recall it, then press Enter without editing | Result shows `= 8` again | | Recalled expression can be re-submitted unchanged |
| 24-0009 | In one session, enter and submit `7 - 1`. Type `quit` and exit. Start a new interactive session and press Up at the prompt | No prior expression is recalled; history is empty for the new session | | History is session-scoped only |
| 24-0010 | At a fresh prompt with no submitted expressions yet, press Up and Down | Input line remains empty; no errors or crashes | | Empty history edge case |
| 24-0011 | Submit `2 + 3`, press Enter on an empty line twice, then submit `4 + 5`. Press Up once | Prompt shows `4 + 5`, not a blank line | | Empty submissions are not stored in history |
| 24-0012 | Submit an invalid expression such as `2 +`, then submit a valid expression `3 + 3`. Press Up once | Prompt shows `3 + 3` | | Only meaningful submitted inputs are recalled |
| 24-0013 | Submit `2 + 3`, then type `quit` and exit. Start a new session, submit one expression, and press Up | Only the expression from the new session is recalled | | Prior session history is not persisted |
| 24-0014 | Run `calculator cli "2 + 3"` (single-expression mode, non-interactive) | Expression evaluates and exits; arrow-key history is not applicable | | Out of scope — interactive mode only |
