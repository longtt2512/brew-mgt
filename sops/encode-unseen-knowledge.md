# SOP: Encode unseen knowledge into the repository

Use this whenever you learn something the repo did not already say — a `brew`
quirk, a JSON shape change, a non-obvious constraint, a recurring mistake. The
goal: the next agent never has to rediscover it from chat history.

## Decide where it belongs

| What you learned                                   | Where it goes                                  |
| -------------------------------------------------- | ---------------------------------------------- |
| A discrete accepted decision / trade-off           | `docs/decisions/NNNN-*.md` + `decision add`    |
| Reasoning/principle spanning many decisions        | `docs/design-docs/`                            |
| Shape of an external tool's output                 | `docs/references/`                             |
| A product rule or behavior                         | `docs/product/`                                |
| A process gap that wasted your time (friction)     | `harness-cli backlog add` + `HARNESS_BACKLOG`  |
| A shortcut you took on purpose                     | `docs/exec-plans/tech-debt-tracker.md`         |
| Where the project stands / what's next             | `claude-progress.md`, `session-handoff.md`     |

## Steps

1. Write the smallest durable note in the right place (table above).
2. If it's a rule agents keep forgetting, prefer turning it into a **mechanical
   check** (test/lint/CLI) over prose — see core belief #4.
3. Cross-link it (`[[ ]]`/relative links) from the doc most likely to be read.
4. Record a trace noting what you encoded:
   `harness-cli trace --summary "Encoded <thing> into <file>" --outcome success`.

## Rule of thumb

If you found yourself reasoning through something that wasn't written down, that
is a signal to write it down. Knowledge that lives only in this session is lost
at session end.
