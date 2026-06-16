# Harness Backlog

The harness grows from friction. This file is the **human-readable view** of the
growth backlog; the durable records live in `harness.db`:

```bash
scripts/bin/harness-cli backlog add --title "<short name>" --pain "<what was hard>" \
  [--risk tiny|normal|high-risk] [--predicted "<expected impact>"]
scripts/bin/harness-cli query backlog --open
scripts/bin/harness-cli query backlog --closed
```

## How To Use

When an agent is confused, repeats manual reasoning, needs a missing validation
command, discovers a missing rule, or sees a recurring failure pattern, it must
either fix the harness directly or add a backlog item. For items expected to
change agent behavior or proof results, fill `--predicted` on creation and the
outcome on close, so predictions can be compared to results later.

Risk uses the shared lane vocabulary: `tiny`, `normal`, `high-risk`.

## Seed Items

These are known gaps in this v0 harness, recorded so they are not forgotten:

1. **No validation scripts exist.** `validate:quick` and the rest of the ladder
   in `docs/HARNESS.md` are aspirational until the Spring Boot project is
   scaffolded. *Pain: agents cannot prove anything mechanically yet.*
   *Risk: normal.*
2. **No CI workflow.** Story verification is manual. *Pain: no automated gate on
   merge.* *Risk: normal.*
3. **Phase 5 CLI commands not implemented.** `score-context`, `audit`,
   `propose`, `intervention`, and `tool register` from the reference harness are
   not in this CLI yet. *Pain: no automated drift/intervention tracking.*
   *Risk: normal.*

Register these in the durable layer after `harness-cli init`:

```bash
scripts/bin/harness-cli backlog add --title "Add validate:quick" --pain "No mechanical proof exists" --risk normal
```
