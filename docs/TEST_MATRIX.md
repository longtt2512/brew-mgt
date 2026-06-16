# Test Matrix

The behavior-to-proof control panel. Each product behavior maps to the proof
that shows it works. The **durable source of truth** is the SQLite database —
query it with:

```bash
scripts/bin/harness-cli query matrix            # human-readable yes/no
scripts/bin/harness-cli query matrix --numeric  # 1/0, for copying into story update
```

This markdown table is the human-readable seed. Keep it roughly in sync, but
when they disagree the CLI output (backed by `harness.db`) wins.

## Proof Columns

- **unit** — service-layer logic with a mocked `BrewClient`.
- **integration** — `@SpringBootTest` slice / adapter against fixture `brew
  --json` output.
- **e2e** — MockMvc/WebTestClient through the REST API.
- **platform** — smoke check against real `brew` on a controlled machine.

`1`/`yes` = proof exists and passed. `0`/`no` = not yet.

## Seeded Behaviors

| Story  | Behavior                                   | Lane      | unit | integration | e2e | platform |
| ------ | ------------------------------------------ | --------- | ---- | ----------- | --- | -------- |
| US-001 | List installed formulae & casks            | normal    | no   | no          | no  | no       |
| US-002 | Detect outdated packages                   | normal    | no   | no          | no  | no       |
| US-003 | List running `brew services`               | normal    | no   | no          | no  | no       |
| US-004 | Show package info / dependency detail      | normal    | no   | no          | no  | no       |
| US-005 | Upgrade a selected package (destructive)   | high-risk | no   | no          | no  | no       |

All proofs are `no` because no application code exists yet. As stories are
implemented, update the durable record:

```bash
scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 1 --platform 0
```

Do not mark `platform` proofs as passing in CI — they require a real machine
with Homebrew and belong to the `test:platform` lane.
