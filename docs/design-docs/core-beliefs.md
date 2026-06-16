# Core Beliefs

Principles that shape `brew-mgt`. An agent should not violate these without an
explicit decision record. They are deliberately few.

1. **The repository is the system of record.** Intent, decisions, proof, and
   next steps live in files (and `harness.db`), never only in chat.

2. **One dangerous door.** All Homebrew/shell interaction goes through a single
   port and adapter (decision 0001). The blast radius of a mistake is bounded by
   keeping that surface tiny and auditable.

3. **Reads are safe; mutations are explicit.** No destructive `brew` action ever
   happens implicitly or as a side effect of a read. Each mutation is a named,
   lane-checked, opt-in use case.

4. **Mechanical checks beat remembered rules.** Prefer a test, a lint, or a CLI
   verification over a rule an agent must remember. When a rule keeps getting
   forgotten, turn it into a check.

5. **Structured over scraped.** Parse `brew --json=v2`; do not scrape
   human-readable CLI output where a JSON form exists.

6. **Finish before you grow.** One feature `in_progress` at a time; evidence
   before `passing`; clean state before ending a session.

7. **The harness is a product too.** It grows from friction and is simplified as
   models improve (see `quality-document.md`). Components that no longer earn
   their keep are removed.
