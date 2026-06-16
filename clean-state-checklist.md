# Clean-State Checklist

Run through this before ending any session. The goal: the next session starts
cleanly with **no manual fixes**. Agents must check these as part of the
end-of-session routine.

## Checklist

- [ ] **Startup works** — `./init.sh` completes without error.
- [ ] **Verification runs** — the `VERIFY_CMD` baseline passes (once the app
      exists, `./gradlew test` is green).
- [ ] **Progress log updated** — `claude-progress.md` has a new session entry and
      the Current Verified State block is accurate.
- [ ] **Handoff updated** — `session-handoff.md` reflects this session.
- [ ] **Feature list honest** — `feature_list.json` has no false `passing`
      entries; exactly zero or one feature is `in_progress`; evidence is filled
      for anything marked `passing`. Then `harness-cli feature sync` was run.
- [ ] **Test matrix synced** — `harness-cli query matrix` matches reality.
- [ ] **Trace recorded** — `harness-cli trace ...` for this task.
- [ ] **Decisions durable** — any high-risk decision has both a
      `docs/decisions/*.md` file and a `harness-cli decision add` record.
- [ ] **No half-finished work** — uncommitted experiments are either finished,
      reverted, or documented in the handoff and progress log.
- [ ] **Backlog captured** — harness friction recorded via `harness-cli backlog
      add`.
- [ ] **harness.db not committed** — it stays git-ignored.

## If any item fails

Do not end the session as "done". Either fix it, or record the exact blocker in
`session-handoff.md` and `claude-progress.md` so the next session can resume.
