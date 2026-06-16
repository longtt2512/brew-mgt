# Plans

Router into execution plans. A **plan** is a multi-step body of work too large
for a single story — it sequences stories/changes toward a goal and lives beside
the code so agents inherit it without chat history.

- **Active plans:** `docs/exec-plans/active/` — at most a few at a time.
- **Completed plans:** `docs/exec-plans/completed/` — archived for history.
- **Tech debt:** `docs/exec-plans/tech-debt-tracker.md` — known shortcuts and
  their repayment plan.
- **Template:** `docs/templates/exec-plan.md`.

## How plans relate to stories

```text
initiative / goal
  -> exec-plan (docs/exec-plans/active/<slug>.md)      # the sequence
       -> story packets (docs/stories/US-xxx.md)        # the units
            -> feature_list.json entries                # the tracker
                 -> harness.db (via feature sync)        # the durable record
```

Keep plans honest: when a step lands, update the plan and the feature list in
the same session. Move a plan to `completed/` only when its exit criteria are
met and verified.

## Current

No active multi-step plan. The next likely plan is **"Scaffold the Spring Boot
application"** (high-risk; needs a decision record) followed by implementing
`US-001`..`US-004`.
