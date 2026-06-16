# Session Handoff

A compact handoff note between sessions. Fill this out at the **end** of every
session so the next session (or the next agent/model) can pick up using repo
artifacts alone — never chat history. Overwrite with the latest handoff; the
durable history lives in `claude-progress.md` and `harness-cli` traces.

## Currently verified

- What is confirmed working: _(nothing app-level yet; harness scaffold only)_
- Verification run to confirm it: _(none — no app code; `init.sh` baseline is a
  no-op placeholder)_

## Changes this session

- Comprehensive harness scaffold created (see `claude-progress.md` for the full
  list). No application code.

## Still broken or unverified

- The Rust `harness-cli` has not been compiled in CI; build with
  `scripts/build-cli.sh`.
- No Spring Boot project exists; the validation ladder commands do not yet run.

## Next best action

- Run `./init.sh`.
- Then EITHER scaffold the Spring Boot app (high-risk → decision record first)
  OR start `US-001`.
- **Do not** implement destructive `brew` actions (US-005) before its dedicated
  safety decision record exists.

## Commands

```bash
./init.sh                                   # startup
scripts/build-cli.sh                        # build the harness CLI
scripts/bin/harness-cli feature sync        # mirror feature_list.json -> DB
scripts/bin/harness-cli query matrix        # proof status
# VERIFY_CMD (in init.sh) is currently `true`; becomes ./gradlew test later
```
