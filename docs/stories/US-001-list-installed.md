# Story: US-001 List installed formulae & casks

- **Id:** US-001
- **Lane:** normal
- **Status:** proposed
- **Intake type:** spec-slice
- **Product docs affected:** docs/product/PRODUCT.md, docs/product/api-contract.md
- **Decision record:** n/a

## User Story

As a developer, I want to see every formula and cask Homebrew has installed with
its version, so that I have a single inventory of my tools.

## Scope

In scope:
- `GET /api/packages` returning formulae and casks with name, version, type,
  outdated flag.
- Optional `?type=formula|cask` filter.

Out of scope:
- Outdated detection detail (US-002), package detail (US-004).

## Behavior / Acceptance Criteria

1. Given Homebrew has packages installed, when I call `GET /api/packages`, then
   I receive formulae and casks with name and version.
2. Given `?type=cask`, when I call the endpoint, then only casks are returned.
3. Given `brew` fails, then the API returns the error envelope (not an empty
   list).

## Validation (proof)

| proof        | expected | how                                          |
| ------------ | -------- | -------------------------------------------- |
| unit         | yes      | service test with mocked BrewClient          |
| integration  | yes      | adapter parses fixture `brew info --json=v2` |
| e2e          | yes      | MockMvc `GET /api/packages`                  |
| platform     | no       | manual real-brew smoke later                 |

## Register

```bash
scripts/bin/harness-cli story add --id US-001 --title "List installed formulae & casks" --lane normal
```
