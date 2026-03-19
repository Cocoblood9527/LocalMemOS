# V2-R15 LoCoMo Closure Design

## Goal

Close the LoCoMo V2 track without introducing further retrieval behavior changes.

## Closure Criteria

1. V2-R14 auto-stop gate is merged and usable.
2. Required 4-command validation matrix is green.
3. `run-full-gate.sh` is green with thresholds and drift checks passing.
4. Closure evidence is written in a release document.
5. Stable tag is created from merged `main`.

## Scope

In-scope:

- closure docs
- final verification evidence capture
- stable tag creation

Out-of-scope:

- new retrieval tuning
- new ranking/query rewrite rules

## Success Condition

If all closure criteria are met, mark LoCoMo V2 as closed and shift to maintenance-only mode.
