# Test Plan: #23 — AS a user, I want to be able to type an expression without spaces

**Issue:** https://git.vylpes.xyz/RabbitLabs/calculator/issues/23
**Generated:** 2026-06-29
**Milestone:** 0.1.0

## Summary

Verify that the CLI expression handler accepts arithmetic input with or without spaces, treating whitespace as ignorable while preserving existing spaced-expression behavior and error handling.

## Test Cases

| Id | Description | Expected Results | Actual Results | Notes |
|----|-------------|------------------|----------------|-------|
| 23-0001 | Start interactive CLI (`calculator cli`). Enter `2 + 3` | Output shows `= 5` | | Acceptance criterion — with spaces |
| 23-0002 | In interactive CLI, enter `2+3` | Output shows `= 5` | | Acceptance criterion — without spaces |
| 23-0003 | Run `calculator cli "2+3"` (single-expression mode) | Prints `5` to stdout | | Without spaces, non-interactive |
| 23-0004 | In interactive CLI, enter `10-4` | Output shows `= 6` | | Subtraction without spaces |
| 23-0005 | In interactive CLI, enter `3*7` | Output shows `= 21` | | Multiplication without spaces |
| 23-0006 | In interactive CLI, enter `15/3` | Output shows `= 5` | | Division without spaces |
| 23-0007 | In interactive CLI, enter `2  +  3` (multiple spaces) | Output shows `= 5` | | Spaces are ignored, not required |
| 23-0008 | In interactive CLI, enter each spaced expression: `2 + 3`, `10 - 4`, `3 * 7`, `15 / 3` | Each evaluates to `5`, `6`, `21`, and `5` respectively | | Regression — spaced input still works |
| 23-0009 | In interactive CLI, enter `5/0` | Error message: division by zero | | Error handling unchanged |
| 23-0010 | In interactive CLI, enter `2+` | Error message: invalid expression | | Malformed input without spaces |
| 23-0011 | In interactive CLI, enter `invalid` | Error message: invalid expression | | Non-numeric input rejected |
| 23-0012 | Run `calculator cli "2 + 3"` (single-expression mode, with spaces) | Prints `5` to stdout | | Regression — spaced single-expression mode |
