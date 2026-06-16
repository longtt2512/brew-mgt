# Decision <NNNN>: <title>

- **Id:** NNNN-slug
- **Status:** proposed | accepted | superseded
- **Date:** YYYY-MM-DD
- **Lane:** high-risk (decisions are for boundary/contract/security changes)
- **Related stories:** US-XXX

## Context

What problem or change forced a decision? What constraints apply (Homebrew
behavior, security, host side effects)?

## Decision

What was decided, stated plainly.

## Alternatives Considered

- Option A — why not.
- Option B — why not.

## Consequences

- Positive: ...
- Negative / trade-offs: ...
- Follow-up needed: ...

## Register in the durable layer

```bash
scripts/bin/harness-cli decision add \
  --id NNNN-slug \
  --title "<title>" \
  --doc docs/decisions/NNNN-slug.md \
  --notes "Accepted during US-XXX."
```
