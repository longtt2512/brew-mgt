# Quality Document

A health snapshot of the **codebase itself**, graded over time. This is distinct
from `evaluator-rubric.md`:

- Evaluator rubric → "Did the agent do good work this session?"
- Quality document → "Is the project getting stronger or weaker over time?"

(This file plays the `QUALITY_SCORE.md` role from the OpenAI Advanced Pack.)

Grade each item **A–D**. Read this before a session to find the weakest area;
update it after significant sessions, before benchmark comparisons, after
cleanup passes, and when onboarding a new agent/model.

## Snapshot — 2026-06-15

### Product domains

| domain      | verification | agent legibility | test stability | key gaps                          | grade |
| ----------- | ------------ | ---------------- | -------------- | --------------------------------- | ----- |
| inventory   | none         | B (docs only)    | n/a            | no code; US-001/002/004 unbuilt   | D     |
| services    | none         | B (docs only)    | n/a            | no code; US-003 unbuilt           | D     |
| lifecycle   | none         | B (docs only)    | n/a            | no code; US-005 needs safety ADR  | D     |

### Architectural layers

| layer                 | boundary enforcement      | agent legibility | grade |
| --------------------- | ------------------------- | ---------------- | ----- |
| web (controllers)     | defined in ARCHITECTURE   | B                | C     |
| service (use cases)   | defined                   | B                | C     |
| BrewClient port       | defined (decision 0001)   | A                | B     |
| ShellBrewClient adapter | defined; not implemented | B                | D     |

All domains grade **D** because no application code exists — grades reflect that
intent is documented but unverified. They should rise as features reach
`passing` with recorded evidence.

## Harness simplification tie-in

Every harness component encodes an assumption about what the model cannot do. As
models improve, assumptions go stale. To test whether a component still earns
its place: snapshot this document, remove one component, run the benchmark task
suite, snapshot again. If grades did not drop, the component was overhead — keep
it removed. If they dropped, restore it. Record each experiment here.

| date | component removed | grade delta | kept? |
| ---- | ----------------- | ----------- | ----- |
|      |                   |             |       |
