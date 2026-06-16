# SOP: Add a feature within the layered architecture

Use this when implementing a story (e.g. US-001) so the layer boundaries in
`docs/ARCHITECTURE.md` stay intact.

## Steps

1. **Classify & record intake.**
   `harness-cli intake --type spec-slice --summary "<feature>" --lane <lane>`.
2. **Mark the feature in_progress.** Edit `feature_list.json` (exactly one
   `in_progress`), then `harness-cli feature sync`.
3. **Define the read model** (or reuse one) in the domain layer — an immutable
   `record`. Reference `docs/product/domain-model.md`.
4. **Extend the port if needed.** Add a method to `BrewClient` describing the
   capability in terms of the read model — not in terms of shell.
5. **Implement the adapter method.** In `ShellBrewClient` only: build the `brew`
   argument list (allowlisted subcommand), execute with a timeout, parse
   `--json=v2` per `docs/references/brew-json-reference.md`. No business logic
   here.
6. **Implement the service use case.** Orchestrate the port; apply product
   rules (e.g. reads never mutate). This is where unit tests with a mocked
   `BrewClient` live.
7. **Expose via controller + DTO.** Map the read model to a DTO; do not leak
   adapter types. Use the status codes from `docs/DESIGN.md`.
8. **Prove it.** Run the matrix proofs for the feature; record evidence in
   `feature_list.json`, then `harness-cli feature sync` and
   `harness-cli story update --id <id> --unit 1 ...`.
9. **Close out.** Update `claude-progress.md` + `session-handoff.md`, run the
   clean-state checklist, record a trace.

## Boundary checks before you commit

- Did any class other than `ShellBrewClient` run a shell command? → revert.
- Did a controller call the adapter directly? → route through the service.
- Did a read path reach a mutating subcommand? → security violation; stop.
