# Story: US-002 Detect outdated packages

- **Id:** US-002
- **Lane:** normal
- **Status:** proposed
- **Intake type:** spec-slice
- **Product docs affected:** docs/product/PRODUCT.md, docs/product/api-contract.md
- **Decision record:** n/a

## User Story

As a developer, I want to see which installed packages have a newer version
available, so that I can decide what to upgrade.

## Scope

In scope:
- `GET /api/packages/outdated` returning name, current, latest, type, pinned.

Out of scope:
- Performing upgrades (US-005).

## Behavior / Acceptance Criteria

1. Given outdated packages exist, when I call the endpoint, then each entry has
   current and latest versions.
2. Given nothing is outdated, then a 200 with an empty array is returned.
3. Pinned formulae are flagged `pinned: true`.

## Validation (proof)

| proof        | expected | how                                            |
| ------------ | -------- | ---------------------------------------------- |
| unit         | yes      | service maps `brew outdated --json=v2`         |
| integration  | yes      | adapter against fixture output                 |
| e2e          | yes      | MockMvc `GET /api/packages/outdated`           |
| platform     | no       | manual real-brew smoke later                   |

## Register

```bash
scripts/bin/harness-cli story add --id US-002 --title "Detect outdated packages" --lane normal
```
