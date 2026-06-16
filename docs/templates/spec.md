# Spec: <name>

> Use this when a human supplies a new specification. A spec is **input
> material**, not a permanent manual. Decompose it into `docs/product/` docs,
> story candidates, and validation expectations, then stop extending it.

## Goal

One paragraph: what this delivers and for whom.

## Capabilities

- Capability 1 — ...
- Capability 2 — ...

## Constraints

- Platform / runtime constraints.
- Homebrew interaction constraints (read vs destructive).
- Security / host side-effect constraints.

## Out of Scope

- ...

## Candidate Stories

| id     | title | lane |
| ------ | ----- | ---- |
| US-XXX | ...   | ...  |

## Open Decisions

- ... (each becomes a `docs/decisions/` record when resolved)

## Decomposition Checklist

- [ ] Product docs created/updated under `docs/product/`.
- [ ] Candidate stories registered with `harness-cli story add`.
- [ ] Test-matrix rows seeded.
- [ ] High-risk items have decision records.
