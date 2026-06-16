# API Contract (intended)

Draft REST contract for `brew-mgt`. **Not yet locked** — it becomes binding when
the Spring Boot project is scaffolded, at which point any change here is
`high-risk` and needs a decision record. Until then, treat it as the target
shape that stories implement against.

All responses are JSON. Errors use a consistent envelope.

## GET /api/packages  (US-001)

List installed formulae and casks.

```jsonc
// 200 OK
{
  "formulae": [
    { "name": "git", "version": "2.45.0", "installedOn": "2026-01-12", "outdated": false }
  ],
  "casks": [
    { "name": "visual-studio-code", "version": "1.90.0", "installedOn": "2026-02-01", "outdated": true }
  ]
}
```

Query params: `?type=formula|cask` (optional filter).

## GET /api/packages/outdated  (US-002)

```jsonc
// 200 OK
[
  { "name": "node", "current": "20.10.0", "latest": "22.3.0", "type": "formula", "pinned": false }
]
```

## GET /api/packages/{name}  (US-004)

```jsonc
// 200 OK
{
  "name": "git",
  "version": "2.45.0",
  "type": "formula",
  "description": "Distributed revision control system",
  "homepage": "https://git-scm.com",
  "dependencies": ["gettext", "pcre2"],
  "outdated": false
}
// 404 if not installed
```

## GET /api/services  (US-003)

```jsonc
// 200 OK
[
  { "name": "postgresql@16", "status": "started", "user": "long", "file": "/opt/homebrew/..." }
]
```

`status` ∈ `started | stopped | error | unknown`.

## POST /api/packages/{name}/upgrade  (US-005, high-risk)

Mutating. Upgrades one named package. Must be explicit; never bulk-upgrades
implicitly.

```jsonc
// Request body (optional)
{ "dryRun": true }

// 200 OK
{ "name": "node", "from": "20.10.0", "to": "22.3.0", "status": "upgraded", "dryRun": false }
// 409 if not outdated; 404 if not installed
```

## Error Envelope

```jsonc
// 4xx / 5xx
{
  "error": "BREW_COMMAND_FAILED",
  "message": "brew exited with code 1",
  "detail": "Error: No such keg: /opt/homebrew/Cellar/foo"
}
```

## Notes

- Pagination is out of scope for v0 (local machine, bounded list sizes).
- Long-running `brew` operations (upgrade) may later move to async/job status;
  that would be a new story + decision.
