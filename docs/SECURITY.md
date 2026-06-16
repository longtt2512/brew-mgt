# Security Policy

`brew-mgt` executes a privileged local tool (`brew`) that can mutate the user's
machine. Security is therefore a first-class product concern, not an
afterthought. Changes to anything in this file are `high-risk` and require a
decision record.

## Threat model (v0)

- **Command injection** — the highest risk. Untrusted input (package names from
  API callers) must never be interpolated into a shell string.
- **Unintended mutation** — a read path accidentally triggering a destructive
  `brew` action.
- **Privilege / scope** — running `brew` with more authority than needed, or on
  an unexpected host.
- **Information disclosure** — leaking host paths, env, or `brew` internals in
  error responses beyond what is useful.

## Controls

1. **Single execution surface.** All shell execution lives in one adapter
   (`ShellBrewClient`, decision 0001). Security review focuses there.
2. **No shell string interpolation.** The adapter invokes `brew` with an
   **argument list** (e.g. `ProcessBuilder("brew", "info", name)`), never
   `sh -c "brew info " + name`. The OS does not re-parse arguments.
3. **Subcommand allowlist.** Only a fixed set of `brew` subcommands may be
   invoked. Mutating subcommands (`upgrade`, `cleanup`, `uninstall`, `services
   start/stop`, `pin`) are gated behind explicit high-risk use cases.
4. **Input validation.** Package names are validated against a conservative
   pattern (alphanumerics, `-`, `_`, `@`, `.`, `+`) before reaching the adapter;
   reject anything else.
5. **No implicit mutation.** Read endpoints can never reach a mutating
   subcommand.
6. **Bounded error detail.** The API error envelope surfaces `brew`'s message
   but the service decides how much host detail to expose.
7. **Local-first, unauthenticated by assumption.** v0 assumes a single local
   user. **Exposing the service over a network is a high-risk decision** that
   must add authentication/authorization first.

## Review triggers

Require a security review (and decision record) when changing: the adapter, the
allowlist, input validation, the error envelope, or anything that would make the
service reachable beyond localhost.
