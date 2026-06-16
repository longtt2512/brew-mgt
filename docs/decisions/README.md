# Decisions

Durable decision records. Required for `high-risk` work that changes behavior or
architecture. Each record must exist in **both** places: a markdown file here
(from `docs/templates/decision.md`) and a durable row via
`scripts/bin/harness-cli decision add`. A decision mentioned only in a trace
does not count.

Numbering is sequential: `NNNN-slug.md`.

| id                            | title                                          | status   |
| ----------------------------- | ---------------------------------------------- | -------- |
| 0001-brew-execution-boundary  | Single port/adapter for all Homebrew execution | accepted |
