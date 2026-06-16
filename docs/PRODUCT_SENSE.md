# Product Sense

Guidance for judgment calls the contract doesn't spell out. When an agent faces
a "what would the user actually want here?" decision, default to these.

## Who the user is

A developer who installs a lot via Homebrew and wants **visibility and control
without leaving a tool they trust**. They value safety over cleverness: a tool
that touches their machine must be predictable and never surprising.

## Defaults for ambiguous decisions

- **Safety beats convenience.** When unsure, prefer the non-destructive option
  and make the destructive one explicit and opt-in.
- **Honest state beats optimistic state.** Show real `brew` status, including
  errors, rather than a tidy fiction.
- **Single-package over bulk.** Prefer operating on one named package; never add
  an implicit "upgrade everything".
- **Transparency.** It is good to show the user exactly which `brew` command
  would run (especially for mutations / `dryRun`).
- **Small surface.** Resist feature sprawl. Each new capability is a story with a
  clear user-visible behavior, not a grab-bag flag.

## Good "done-when" examples

- *List installed*: "I can see every formula and cask with its version in one
  call, and a `brew` failure shows up as a clear error, not an empty list."
- *Upgrade*: "I can upgrade exactly the package I named, preview it with
  `dryRun`, and nothing else on my machine changes."

## Anti-goals

- A general remote package-management platform.
- Managing non-Homebrew package managers.
- Silent background mutation of the user's machine.
