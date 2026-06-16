# Story: <US-XXX> <short title>

- **Id:** US-XXX
- **Lane:** tiny | normal | high-risk
- **Status:** proposed | in-progress | done | blocked
- **Intake type:** spec-slice | change-request | ...
- **Product docs affected:** docs/product/<file>.md
- **Decision record:** docs/decisions/<id>.md  (required if high-risk)

## User Story

As a <user>, I want <capability>, so that <value>.

## Scope

In scope:
- ...

Out of scope:
- ...

## Behavior / Acceptance Criteria

1. Given <state>, when <action>, then <observable result>.
2. ...

## Validation (proof)

| proof        | expected | how                                            |
| ------------ | -------- | ---------------------------------------------- |
| unit         | yes/no   | service test with mocked BrewClient            |
| integration  | yes/no   | adapter against fixture `brew --json` output   |
| e2e          | yes/no   | MockMvc/WebTestClient API flow                 |
| platform     | yes/no   | real `brew` smoke on a controlled machine      |

Verify command (optional, for `harness-cli story verify`):

```bash
# e.g. ./gradlew test --tests '*ListInstalled*'
```

## Notes / Risks

- ...

## Register in the durable layer

```bash
scripts/bin/harness-cli story add --id US-XXX --title "<title>" --lane <lane>
```
