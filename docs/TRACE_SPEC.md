# Trace Spec

A **trace** is the durable record of a single task: what was attempted, the
outcome, and the friction encountered. Traces let future agents see what
happened without reading chat history. Record one at the end of every task:

```bash
scripts/bin/harness-cli trace \
  --summary "Implemented list-installed endpoint" \
  --outcome success \
  --story US-001 \
  --friction "brew --json schema for casks differs from formulae"
```

## Fields

- **summary** (required) — one line: what the task did.
- **outcome** (required) — one of: `success`, `partial`, `blocked`, `reverted`.
- **story** (optional) — the story id this work served (e.g. `US-001`).
- **friction** (optional) — what was harder than it should have been. This feeds
  the growth rule; recurring friction becomes a backlog item.

## Trace Tiers (depth expectations)

- **tiny lane** — summary + outcome is enough.
- **normal lane** — summary + outcome + story link; friction if any.
- **high-risk lane** — all of the above, and the trace must reference the
  decision record id created for the work. The trace does **not** replace the
  decision record (see `docs/HARNESS.md` → Decision Records).

## Outcome Vocabulary

| outcome  | meaning                                                        |
| -------- | -------------------------------------------------------------- |
| success  | requested change completed and available proofs ran           |
| partial  | some of the work landed; remainder documented in the trace    |
| blocked  | could not proceed; blocker described in the summary/friction   |
| reverted | change was made then rolled back; reason in summary           |

## Rule

A task is not **done** (per the Done Definition in `docs/HARNESS.md`) until a
trace exists. If you found harness friction, also record it with
`scripts/bin/harness-cli backlog add`.
