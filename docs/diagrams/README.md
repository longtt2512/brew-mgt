# Diagrams

PlantUML sources for presenting the harness.

- `harness-flow.puml` — the per-session operating flow (Session Start → Task
  Loop → Session End). The "how it runs" view.
- `harness-artifacts.puml` — the system-of-record map: how entrypoints, the
  source-of-truth docs, continuity files, and the durable layer relate. The
  "what's in the repo" view.

## Render

Pick whichever is easiest:

```bash
# Local jar (needs Java; Graphviz/dot for component diagrams)
brew install plantuml graphviz
plantuml docs/diagrams/harness-flow.puml          # -> harness-flow.png
plantuml -tsvg docs/diagrams/harness-artifacts.puml

# VS Code: install the "PlantUML" extension, open the .puml, Alt+D to preview.
# Web: paste the file contents into https://www.plantuml.com/plantuml
```

For slides, export SVG (`-tsvg`) so the diagram stays crisp when scaled.
