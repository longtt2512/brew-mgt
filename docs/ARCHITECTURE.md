# Architecture

This document records architecture **boundaries and discovery rules**, not a
finished design. There is no application code yet; the rules below constrain
how it should be built when the first story requires scaffolding.

## Discovery Rules

Before changing or adding code, an agent must:

1. Read `docs/product/PRODUCT.md` and the relevant story packet.
2. Identify which layer the change belongs to (see Layers below).
3. Confirm no existing boundary is crossed without a decision record.
4. Check `scripts/bin/harness-cli query matrix` for the proof expectations.

If the codebase does not yet exist for the area in question, scaffolding it is
itself a **high-risk** step (it locks in stack choices) and needs a decision
record.

## Intended Stack

- **Language/Runtime:** Java 21+ (LTS).
- **Framework:** Spring Boot 3.x.
- **Build:** Gradle (Kotlin DSL) — decided per `docs/decisions/` when scaffolded.
- **Persistence:** start stateless; introduce a store only when a story needs
  durable app state (the harness DB is separate and not the app DB).
- **Packaging:** runnable as a local service on macOS where Homebrew lives.

The above is *intended*, not locked. Locking the stack happens through a
decision record at scaffolding time.

## Layers and Boundaries

```text
+-------------------------------------------------------------+
|  Web layer (REST controllers, DTOs, validation)            |
|    - no brew calls here; no business rules                  |
+-------------------------------------------------------------+
|  Service layer (use cases: inventory, outdated, upgrade)   |
|    - orchestration + policy; depends on ports, not adapters |
+-------------------------------------------------------------+
|  Port: BrewClient (interface)                               |
|    - the ONLY contract through which the app reaches brew   |
+-------------------------------------------------------------+
|  Adapter: ShellBrewClient (executes `brew ...`)            |
|    - the ONLY place that runs shell commands                |
+-------------------------------------------------------------+
```

### Hard boundaries

- **All Homebrew interaction goes through one port** (`BrewClient`) and exactly
  one adapter that executes shell. No other class shells out. This keeps the
  dangerous surface auditable and mockable in tests.
- **No destructive `brew` action without an explicit, lane-checked use case.**
  Read operations (`list`, `outdated`, `services list`, `info`) are `normal`;
  mutating operations (`upgrade`, `cleanup`, `uninstall`, `services
  stop/start`, `pin`) are `high-risk` and must be opt-in, never implicit.
- **Controllers never call the adapter directly** — only the service layer.
- **No arbitrary command execution.** The adapter builds `brew` argument lists
  from a fixed allowlist of subcommands; it never interpolates user strings into
  a shell string.
- **Prefer `--json` output** from `brew` and parse structured data; do not
  scrape human-readable output where a JSON form exists.

## Testing Strategy (when code exists)

- Service layer tested against a **mocked `BrewClient`** (no real brew).
- The adapter tested with recorded/fixture `brew --json` outputs.
- Real-`brew` smoke checks belong to the `test:platform` lane and run only on a
  controlled machine, never in unit/CI by default.

## Change Policy

Changing any hard boundary above (the single-port rule, the allowlist, the
destructive-action lane) requires a `high-risk` intake and a decision record.
