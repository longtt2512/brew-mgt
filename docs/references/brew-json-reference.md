# Reference: `brew --json=v2` output shapes

Source: Homebrew CLI (`brew help`, `man brew`). Captured 2026-06-15. Verify
against the installed `brew` version on the target machine before relying on
exact fields — Homebrew evolves.

The adapter (`ShellBrewClient`) parses these structured outputs rather than
scraping human-readable text.

## `brew info --json=v2 [name]`

Top level has two arrays: `formulae` and `casks` (different shapes).

```jsonc
{
  "formulae": [
    {
      "name": "git",
      "versions": { "stable": "2.45.0" },
      "installed": [ { "version": "2.45.0" } ],
      "outdated": false,
      "pinned": false,
      "desc": "Distributed revision control system",
      "homepage": "https://git-scm.com",
      "dependencies": ["gettext", "pcre2"]
    }
  ],
  "casks": [
    {
      "token": "visual-studio-code",
      "version": "1.90.0",
      "installed": "1.89.0",
      "outdated": true,
      "name": ["Visual Studio Code"],
      "desc": "Code editor",
      "homepage": "https://code.visualstudio.com/"
    }
  ]
}
```

Key normalization notes for `Package`:

- Formula identity is `name`; cask identity is `token`. Normalize both to
  `name`.
- Formula installed version: `installed[].version`; cask installed version:
  `installed` (string).
- `outdated` and `pinned` are present on formulae; casks expose `outdated`.

## `brew outdated --json=v2`

```jsonc
{
  "formulae": [
    { "name": "node", "installed_versions": ["20.10.0"], "current_version": "22.3.0", "pinned": false }
  ],
  "casks": [
    { "name": "visual-studio-code", "installed_versions": ["1.89.0"], "current_version": "1.90.0" }
  ]
}
```

## `brew services list --json`

```jsonc
[
  { "name": "postgresql@16", "status": "started", "user": "long", "file": "/opt/homebrew/..." }
]
```

`status` values seen: `started`, `stopped`, `error`, `none`/`unknown`.

## Friction

The differing formula/cask shapes are recorded as known friction; the adapter
must map both into the single `Package` read model (see
`docs/product/domain-model.md`).
