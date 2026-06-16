# SOP: Local observability & feedback loop

A run → observe → diagnose → fix loop for backend work, replacing the Advanced
Pack's Chrome DevTools UI loop (brew-mgt is API-only for now).

## Loop

1. **Run.** Start the service: `RUN_START_COMMAND=1 ./init.sh` (once scaffolded,
   `START_CMD` = `./gradlew bootRun`).
2. **Exercise.** Hit the endpoint under test:

   ```bash
   curl -s localhost:8080/api/packages | jq .
   curl -s localhost:8080/api/packages/outdated | jq .
   ```
3. **Observe.** Watch structured logs. Per `docs/RELIABILITY.md`, every adapter
   invocation logs subcommand, args, duration, and exit code. Tail them:

   ```bash
   ./gradlew bootRun | tee /tmp/brew-mgt.log
   grep BREW_ /tmp/brew-mgt.log
   ```
4. **Diagnose.** Compare actual `brew` output to
   `docs/references/brew-json-reference.md`. If the shape drifted, that's
   encodable knowledge (see `encode-unseen-knowledge.md`).
5. **Fix & prove.** Make the change, add/extend the unit test that reproduces
   the issue with a faulted/mocked `BrewClient`, re-run verification.
6. **Record.** Update `feature_list.json` evidence, `feature sync`, trace.

## Without the app yet

The harness layer is observable now:

```bash
scripts/bin/harness-cli query stats
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query backlog --open
```

Use these to confirm the durable state matches the markdown before ending a
session.
