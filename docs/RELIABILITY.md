# Reliability Policy

How `brew-mgt` stays predictable while wrapping an external, sometimes-slow,
sometimes-failing CLI.

## Principles

1. **The external tool will fail.** `brew` can be missing, slow, return
   non-zero, or print unexpected JSON. Treat every invocation as fallible.
2. **Fail loud, not silent.** Surface `brew` failures through the API error
   envelope; never return an empty success in place of an error.
3. **Read operations are idempotent.** Listing/inspecting must have no side
   effects and be safe to retry.
4. **Mutations are explicit and reportable.** An upgrade reports `from`/`to` and
   a clear status; partial failure is reported, not hidden.

## Mechanisms (to implement with the app)

- **Timeouts.** Every `brew` invocation runs with a timeout; a hung command
  fails with a clear error rather than blocking the request thread.
- **Exit-code mapping.** Non-zero `brew` exit → `BREW_COMMAND_FAILED` with the
  captured stderr in `detail`.
- **JSON-parse guards.** A schema/shape mismatch is a handled error, not a 500
  stack trace; log the offending payload.
- **No partial writes to product state.** v0 keeps no app DB; if one is added, a
  failed mutation must not leave inconsistent state.
- **Long operations.** If `brew upgrade` proves too slow for a sync request,
  move it to an async job with status polling — that is a new story + decision,
  not an ad-hoc change.

## Observability

See `sops/local-observability-loop.md`. At minimum: structured logs around every
adapter invocation (subcommand, args, duration, exit code) so failures are
diagnosable from logs alone.

## Verification

Reliability behaviors (timeouts, error mapping, parse guards) must have unit
coverage against a mocked/faulted `BrewClient`. Real-`brew` failure modes are
exercised in the `test:platform` lane on a controlled machine.
