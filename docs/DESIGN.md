# Design

API and code design conventions for `brew-mgt`. (The Advanced Pack's
`FRONTEND.md` is intentionally omitted — this is an API-only backend for now. If
a UI is added later, create `FRONTEND.md` then.)

## API design

- **REST, JSON, noun-based resources:** `/api/packages`, `/api/services`.
- **Read with GET, mutate with POST** to an action sub-resource
  (`/api/packages/{name}/upgrade`). No verbs in GET paths.
- **Consistent error envelope** (`docs/product/api-contract.md`): `{ error,
  message, detail }` with a stable `error` code enum.
- **Status codes mean something:** 404 not installed, 409 not in an upgradable
  state, 422 invalid package name, 5xx only for genuine server faults.
- **No pagination in v0** — local machine, bounded lists.

## Code design (when scaffolded)

- **Layering** per `docs/ARCHITECTURE.md`: controller → service → `BrewClient`
  port → `ShellBrewClient` adapter. Controllers never call the adapter.
- **DTOs at the edge.** Controllers expose DTOs derived from the domain read
  models; `brew`/adapter types never leak into the web layer.
- **Constructor injection**, no field injection. Components are testable with a
  mocked `BrewClient`.
- **Immutable read models** (`record` types) for `Package`, `Service`, etc.
- **One responsibility per class.** The adapter only executes + parses; the
  service only orchestrates + applies policy.

## Naming

- Use cases read as actions: `ListInstalledPackages`, `DetectOutdated`,
  `UpgradePackage`.
- Test names state behavior: `returnsEmptyListWhenNothingOutdated`,
  `rejectsInvalidPackageName`.

## Conventions

- Format/lint enforced mechanically (Spotless/Checkstyle) once scaffolded; wired
  into `init.sh` `VERIFY_CMD`.
- Comments and identifiers in English (project language convention).
