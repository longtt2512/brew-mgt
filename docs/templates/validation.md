# Validation Report: <US-XXX> — <date>

- **Story:** US-XXX
- **Lane:** tiny | normal | high-risk
- **Run by:** human | agent

## Commands Run

```bash
# paste the exact commands and their pass/fail result
# e.g. ./gradlew test  -> PASS
```

## Proof Status

| proof        | before | after | evidence (test name / log) |
| ------------ | ------ | ----- | -------------------------- |
| unit         |        |       |                            |
| integration  |        |       |                            |
| e2e          |        |       |                            |
| platform     |        |       |                            |

Update the durable record:

```bash
scripts/bin/harness-cli story update --id US-XXX --unit 1 --integration 0 --e2e 0 --platform 0
```

## Not Attempted

- What was intentionally not validated, and why.

## Trace

```bash
scripts/bin/harness-cli trace --summary "<what changed>" --outcome success --story US-XXX
```
