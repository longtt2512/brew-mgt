# Feature Intake

Every request enters the harness through intake. Intake answers two questions
before any code or doc changes: **what type of work is this**, and **which risk
lane does it belong in**. Record the result with:

```bash
scripts/bin/harness-cli intake --type <type> --summary "<one line>" --lane <lane>
```

## Input Types

- **new-spec** — a project specification that must become product docs and
  initial story candidates.
- **spec-slice** — a selected behavior taken from an existing spec or
  `docs/product/`.
- **change-request** — a bounded behavior change, bug fix, or refinement.
- **new-initiative** — a larger product area needing multiple stories.
- **maintenance** — dependency, performance, security, or operational work.
- **harness-improvement** — a process, template, proof, or instruction change.

## Risk Lanes

The lane sets how much process the task carries. Pick the **highest** lane that
any part of the work touches.

### tiny

Low blast radius, reversible, no contract change. Examples for `brew-mgt`:
fixing a typo, adjusting a log message, renaming a private method, editing a
doc. No decision record required. Proof: it still builds / the doc reads
correctly.

### normal

Default lane for real feature work. Adds or changes behavior within existing
boundaries. Examples: add a "list outdated formulae" endpoint, add a service
method that shells out to `brew outdated --json`, add a DTO. Requires a story
packet and test-matrix rows. Decision record only if a boundary shifts.

### high-risk

Touches a product contract, security/permission surface, data ownership, the
way the app invokes `brew`, or anything destructive. Examples for `brew-mgt`:
running `brew upgrade`/`brew cleanup`/`brew uninstall` on the host, executing
arbitrary shell, changing the command-execution sandbox, auth/authorization,
or changing the public API shape. **Requires** a story packet, a durable
decision record (`docs/decisions/` + `harness-cli decision add`), and explicit
human confirmation before merge.

> Note: anything that mutates the user's machine through Homebrew
> (`upgrade`, `cleanup`, `uninstall`, `services stop/start`, `pin/unpin`) is
> high-risk by default, because it has real side effects on the host.

## Decision Flow

```text
Is the request a destructive brew action, contract/security/API change?
  yes -> high-risk
  no  -> Does it add/alter user-visible behavior or cross a boundary?
           yes -> normal
           no  -> tiny
```

## After Intake

1. `tiny` — do the work in-lane, run available checks, record a trace.
2. `normal` — create a story packet from `docs/templates/story.md`, register it
   (`harness-cli story add`), add test-matrix rows, implement, verify, trace.
3. `high-risk` — all of `normal`, plus a decision record and human confirmation
   before merge.
