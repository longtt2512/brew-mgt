# Story: US-005 Upgrade a selected package (destructive)

- **Id:** US-005
- **Lane:** high-risk
- **Status:** proposed
- **Intake type:** spec-slice
- **Product docs affected:** docs/product/PRODUCT.md, docs/product/api-contract.md
- **Decision record:** docs/decisions/0001-brew-execution-boundary.md (and a new
  record for upgrade safety before implementation)

## User Story

As a developer, I want to upgrade one named package through the service, so that
I can update tools without leaving the app — safely and explicitly.

## Scope

In scope:
- `POST /api/packages/{name}/upgrade` for a single named package.
- `dryRun` support that resolves the action without mutating the host.

Out of scope:
- Bulk `brew upgrade` of everything (explicitly disallowed — see product rules).
- cleanup / uninstall.

## Behavior / Acceptance Criteria

1. Given an outdated package, when I POST upgrade, then it is upgraded and the
   result reports `from`/`to` versions.
2. Given `dryRun: true`, then no mutation occurs and the planned action is
   returned.
3. Given a package that is not installed, then 404; if not outdated, then 409.
4. The action is never triggered implicitly by any read endpoint.

## Why high-risk

This mutates the user's machine via `brew upgrade`. It needs a decision record
covering: confirmation/dry-run policy, timeout handling, and how partial
failures are reported. Requires human confirmation before merge.

## Validation (proof)

| proof        | expected | how                                              |
| ------------ | -------- | ------------------------------------------------ |
| unit         | yes      | service with mocked BrewClient, incl. dryRun     |
| integration  | yes      | adapter command construction (no real exec)      |
| e2e          | yes      | MockMvc happy/404/409 paths                       |
| platform     | yes      | real-brew smoke on a controlled machine ONLY      |

## Register

```bash
scripts/bin/harness-cli story add --id US-005 --title "Upgrade a selected package" --lane high-risk
```
