# Domain Model (intended)

Core concepts for `brew-mgt`. Language-neutral here; mapped to Java types when
scaffolded. No persistence is implied — these are read models derived from
`brew --json` unless a story says otherwise.

## Entities

### Package

A thing Homebrew installs.

| field        | type            | notes                                  |
| ------------ | --------------- | -------------------------------------- |
| name         | string          | e.g. `git`, `visual-studio-code`       |
| type         | enum            | `FORMULA` \| `CASK`                    |
| version      | string          | currently installed version           |
| latest       | string?         | latest available (null if up to date) |
| outdated     | boolean         | derived: `latest != null`             |
| installedOn  | date?           | install timestamp if known            |
| pinned       | boolean         | formula pinned against upgrade         |

### Dependency

Edge between packages (for US-004 detail).

| field   | type   | notes                          |
| ------- | ------ | ------------------------------ |
| name    | string | dependency package name        |
| build   | bool   | build-time only dependency     |

### Service

A `brew services` managed background process.

| field   | type   | notes                                       |
| ------- | ------ | ------------------------------------------- |
| name    | string | e.g. `postgresql@16`                        |
| status  | enum   | `STARTED \| STOPPED \| ERROR \| UNKNOWN`    |
| user    | string?| user the service runs as                   |
| file    | string?| plist / service file path                   |

## Value Objects

- **UpgradeResult** — `{ name, from, to, status, dryRun }` for US-005.
- **BrewError** — `{ error, message, detail }`, mapped to the API error envelope.

## Mapping Notes

- `Package.outdated` and `latest` come from `brew outdated --json=v2`.
- `Package.dependencies` come from `brew info --json=v2 <name>`.
- `Service` comes from `brew services list --json`.
- Casks and formulae have **different JSON shapes** in `brew --json=v2`; the
  adapter normalizes both into `Package`. (Recorded as known friction.)

## Boundaries

These are read models assembled by the **service layer** from data the
**adapter** returns. Controllers expose DTOs derived from these; they do not
expose adapter/`brew` types directly (see `docs/ARCHITECTURE.md`).
