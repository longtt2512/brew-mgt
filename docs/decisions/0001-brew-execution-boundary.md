# Decision 0001: Single port/adapter for all Homebrew execution

- **Id:** 0001-brew-execution-boundary
- **Status:** accepted
- **Date:** 2026-06-15
- **Lane:** high-risk
- **Related stories:** US-001, US-002, US-005

## Context

`brew-mgt` shells out to the `brew` CLI. Shell execution is the most dangerous
surface in the app: it can mutate the user's machine and is a command-injection
risk. We need this surface to be small, auditable, and testable from day one,
before any feature code is written.

## Decision

All Homebrew interaction goes through **one port** (`BrewClient` interface) with
exactly **one adapter** (`ShellBrewClient`) that executes shell. No other class
runs shell commands. The adapter builds `brew` invocations from a **fixed
allowlist of subcommands** and passes arguments as a list (never interpolating
user input into a shell string). Read subcommands are unrestricted within the
allowlist; mutating subcommands (`upgrade`, `cleanup`, `uninstall`,
`services start/stop`, `pin`) are gated behind explicit high-risk use cases.

## Alternatives Considered

- **Let each service shell out as needed** — rejected: scatters the dangerous
  surface, impossible to audit, hard to mock in tests.
- **Use a Homebrew API/library instead of the CLI** — rejected for v0: no
  stable official Java binding; the CLI with `--json=v2` is the supported
  contract.

## Consequences

- Positive: one place to audit/security-review; service layer is unit-testable
  with a mocked `BrewClient`; destructive actions are centrally gated.
- Negative: the adapter must normalize differing formula/cask JSON shapes.
- Follow-up: a separate decision record is required before US-005 (upgrade) to
  cover dry-run/confirmation policy, timeouts, and partial-failure reporting.

## Register

```bash
scripts/bin/harness-cli decision add \
  --id 0001-brew-execution-boundary \
  --title "Single port/adapter for all Homebrew execution" \
  --doc docs/decisions/0001-brew-execution-boundary.md \
  --notes "Accepted at harness bootstrap; constrains all brew access."
```
