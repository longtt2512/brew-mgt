# Harness

The goal of this harness is to let humans and agents turn product intent into
safe, validated changes to `brew-mgt`, with the repository (not chat history) as
the system of record.

> The app is what users touch. The harness is what agents touch.

## Mental Model

```text
+------------------+
| Human intent     |
+------------------+
         |
         v
+------------------+
| Feature intake   |
+------------------+
         |
         v
+------------------+
| Story packet     |
+------------------+
         |
         v
+------------------+
| Agent work loop  |
+------------------+
         |
         v
+------------------+
| Product delta    |
+------------------+
         |
         v
+------------------+
| Validation proof |
+------------------+
         |
         v
+------------------+
| Harness delta    |
+------------------+
         |
         v
+------------------+
| Next intent      |
+------------------+
```

Every task has two possible outputs:

1. **Product delta** — app code, tests, API shape, data model, or product docs.
2. **Harness delta** — docs, templates, validation expectations, backlog items,
   or decision records that make the next task easier.

## Harness v0 Scope (this repo)

Included:

- Agent entrypoints (`AGENTS.md`, `CLAUDE.md`).
- Product documentation for the Homebrew manager (`docs/product/`).
- Feature intake and risk lanes.
- Story, decision, and validation templates.
- Test matrix with seeded rows.
- Harness growth backlog.
- Durable layer: SQLite database + Rust CLI for operational records.

Deliberately excluded until a selected story needs it:

- Spring Boot application source code.
- Build config (Gradle/Maven), test runner config, CI workflows.
- A locked runtime/deployment target.

## Durable Layer

Policy documents describe *how to work*. The durable layer stores *what
happened*. Operational data — intake classifications, story status, decisions,
backlog items, and execution traces — lives in a SQLite database (`harness.db`)
managed by the Rust Harness CLI at `scripts/bin/harness-cli`.

The database is local to each clone and `.gitignore`d. The schema is
version-controlled under `scripts/schema/`.

Build and initialize:

```bash
scripts/build-cli.sh             # cargo build --release + copy to scripts/bin/
scripts/bin/harness-cli init     # apply scripts/schema/*.sql into harness.db
```

### Command reference

```bash
# Intake
scripts/bin/harness-cli intake --type <type> --summary <text> --lane <lane>

# Features (feature_list.json is the source of truth; mirror it into the DB)
scripts/bin/harness-cli feature sync                 # reads ./feature_list.json
scripts/bin/harness-cli feature sync --file <path>

# Stories
scripts/bin/harness-cli story add    --id US-001 --title <text> --lane <lane> [--verify <cmd>]
scripts/bin/harness-cli story update --id US-001 --status <status>
scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 0 --platform 0
scripts/bin/harness-cli story verify US-001
scripts/bin/harness-cli story verify-all

# Decisions
scripts/bin/harness-cli decision add --id 0001-slug --title <text> --doc docs/decisions/0001-slug.md [--notes <text>]

# Traces
scripts/bin/harness-cli trace --summary <text> --outcome <outcome> [--story US-001] [--friction <text>]

# Backlog (harness growth)
scripts/bin/harness-cli backlog add --title <text> --pain <text> [--risk tiny|normal|high-risk] [--predicted <text>]

# Queries
scripts/bin/harness-cli query matrix [--numeric]
scripts/bin/harness-cli query backlog [--open|--closed]
scripts/bin/harness-cli query stats
scripts/bin/harness-cli --version
```

Lane vocabulary is shared across intake, stories, and backlog:
`tiny`, `normal`, `high-risk`. (`low` is not valid.)

### Feature tracking

`feature_list.json` (repo root) is the **human/agent-editable source of truth**
for what to build and its status (`not_started` | `in_progress` | `blocked` |
`passing`). Only one feature may be `in_progress` at a time, and `passing`
requires recorded `evidence`. `harness-cli feature sync` mirrors it into the
`stories` table so `query matrix` and verification work off the durable copy.
Edit the JSON, then sync — do not hand-edit the DB. `init.sh` runs the sync on
every startup.

## Source Hierarchy

```text
User-provided spec or prompt        -> input material, not a permanent manual
docs/product/*                      -> current product contract
docs/stories/*                      -> story-sized work packets + evidence
scripts/bin/harness-cli query matrix-> behavior-to-proof control panel
docs/decisions/*                    -> why the contract changed
```

Before implementation, product docs describe intent. After implementation,
product docs **plus executable tests** become the living contract.

## Task Loop

For every task:

1. Classify the request with `docs/FEATURE_INTAKE.md`.
2. Record it: `harness-cli intake --type <type> --summary <text> --lane <lane>`.
3. Locate the affected product docs and story files.
4. Check proof status: `harness-cli query matrix`.
5. Work only inside the selected lane.
6. Before finishing, ask whether product truth, validation expectations,
   architecture rules, failure patterns, or next-agent instructions changed.
7. Record a trace: `harness-cli trace ...` (see `docs/TRACE_SPEC.md`).
8. If harness friction was found, fix it or record it with `backlog add`.

## Story Verification

Stories may carry a mechanical proof command. `story verify <id>` runs the
configured command from the repo root, records the result, and exits 0 on pass
or 1 on fail. Run `story verify-all` before merges and maturity claims. Record
proof booleans with numeric `1`/`0` (not `yes`/`no`).

## Decision Records

High-risk work that changes behavior or architecture needs a durable decision in
**both** places: a markdown file under `docs/decisions/` (from
`docs/templates/decision.md`) and a durable record via
`harness-cli decision add`. A decision mentioned only inside a trace does not
satisfy this requirement.

## Growth Rule

The harness grows from friction. When an agent is confused, repeats manual
reasoning, needs a missing validation command, or sees a recurring failure, it
must either improve the harness directly or record the friction:

```bash
scripts/bin/harness-cli backlog add --title "<short name>" --pain "<what was hard>"
```

## Harness Change Policy

Update directly: story status/evidence, test-matrix rows, story↔product links,
validation notes, intake records, traces, and backlog items.

Ask a human first: changing architecture direction, removing validation
requirements, changing the source-of-truth hierarchy, changing risk
classification, or replacing the feature workflow.

## Done Definition

A task is done only when:

- The requested change is completed or the blocker is documented.
- Relevant docs, stories, and test-matrix entries remain current.
- Validation commands were run when they exist.
- A trace has been recorded.
- Missing harness capabilities were recorded in the backlog.
- The final response says what changed and what was not attempted.

## Future Validation Ladder

No validation scripts exist yet (no app code). When implementation begins, the
expected ladder for this Spring Boot project is:

```text
validate:quick    -> spotless/format, checkstyle, compile, unit tests (./gradlew test)
test:integration  -> @SpringBootTest, repository/service slices, Testcontainers if used
test:e2e          -> MockMvc / WebTestClient end-to-end API flows
test:platform     -> real `brew` smoke checks on a controlled machine
test:release       -> full suite + log checks + performance smoke
```

Agents must not claim these commands pass until they exist and have been run.
