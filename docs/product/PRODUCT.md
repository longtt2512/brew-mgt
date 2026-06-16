# Product Contract: brew-mgt

The current product contract for the Homebrew management service. This is the
living description of *what the product does*. Before code exists it states
intent; once code exists, this doc **plus executable tests** is the contract.

## Purpose

`brew-mgt` gives a developer one place to see and safely manage everything
Homebrew has installed on their machine: formulae, casks, and background
services. It wraps the `brew` CLI behind a small Spring Boot service with a REST
API, turning ad-hoc terminal commands into an inspectable, auditable workflow.

## Users

- **Primary:** a developer on macOS (or Linuxbrew) who installs many tools via
  Homebrew and wants visibility and controlled upgrades.

## Capabilities

### Read (safe, `normal` lane)

- **Inventory** — list installed formulae and casks with version and install
  date. *(US-001)*
- **Outdated detection** — list packages with a newer version available, current
  vs. latest. *(US-002)*
- **Services** — list `brew services` and their state (started/stopped/error).
  *(US-003)*
- **Package detail** — show info and dependencies for a named package. *(US-004)*

### Mutating (destructive, `high-risk` lane)

- **Upgrade** — upgrade a selected package. Side-effects on the host; explicit,
  never implicit; requires confirmation. *(US-005)*
- *(Future)* cleanup, uninstall, pin/unpin, service start/stop — each its own
  high-risk story with a decision record.

## Product Rules / Invariants

1. **Read operations are always safe** and must never trigger a mutation.
2. **No destructive Homebrew action happens implicitly.** Every mutation is an
   explicit, named use case requested by the user.
3. **The app reaches Homebrew through exactly one port/adapter** (see
   `docs/ARCHITECTURE.md`); no arbitrary shell execution.
4. **Prefer structured `brew --json` output**; surface raw `brew` errors to the
   caller rather than swallowing them.
5. **The service is local-first** — it assumes `brew` is installed on the host
   it runs on.

## Public Surface (intended)

REST endpoints, to be locked when scaffolded (`docs/product/api-contract.md`):

```text
GET  /api/packages              -> installed formulae + casks   (US-001)
GET  /api/packages/outdated     -> outdated packages            (US-002)
GET  /api/packages/{name}       -> package detail + deps        (US-004)
GET  /api/services              -> brew services state          (US-003)
POST /api/packages/{name}/upgrade -> upgrade (high-risk)        (US-005)
```

## Out of Scope (v0)

- Managing non-Homebrew package managers.
- Remote/multi-host management.
- A web UI (API only for now).
- Authentication (local single-user assumption; revisit before any network
  exposure — that would be a high-risk decision).

## Source of Truth

This doc is intent. The behavior-to-proof mapping is in `docs/TEST_MATRIX.md`
(durable copy in `harness.db`). Changes to the public surface or the product
rules above are `high-risk` and require a decision record.
