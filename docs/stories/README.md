# Stories

Story packets — story-sized units of work derived from `docs/product/`. Create
new ones from `docs/templates/story.md` and register them in the durable layer
with `scripts/bin/harness-cli story add`.

## Seeded backlog

| id     | title                              | lane      | status   |
| ------ | ---------------------------------- | --------- | -------- |
| US-001 | List installed formulae & casks    | normal    | proposed |
| US-002 | Detect outdated packages           | normal    | proposed |
| US-003 | List running brew services         | normal    | proposed |
| US-004 | Show package info / dependencies   | normal    | proposed |
| US-005 | Upgrade a selected package         | high-risk | proposed |

US-003 and US-004 are listed in the matrix but do not yet have full packets —
write them from the template when picked up. Status here is a convenience view;
the durable status lives in `harness.db` (`harness-cli query matrix` / `story`).
