# Evaluator Rubric

A scorecard for reviewing agent output after a session or at a milestone. Score
each dimension **0–2** (0 = fails, 1 = partial, 2 = meets bar). Total out of 12.

> **The evaluator needs tuning.** Out of the box, agents are poor self-judges —
> they spot issues then talk themselves into approving. Run this on a completed
> sprint, compare to human judgment, and make pass/fail criteria more specific
> where they diverge. Plan for 3–5 tuning rounds and record each change.

## Dimensions

### 1. Correctness (0–2)

Does the implementation match the target behavior in `feature_list.json` /
`docs/product/`? 2 = behavior matches and edge cases handled; 1 = main path
only; 0 = does not match.

### 2. Verification (0–2)

Were the required checks actually run, with evidence? 2 = all matrix proofs for
the feature ran and evidence is recorded; 1 = some ran; 0 = claimed without
evidence. (See lecture: agents declare victory too early.)

### 3. Scope discipline (0–2)

Did the agent stay within the selected feature and lane? 2 = stayed in lane,
intake recorded; 1 = minor drift; 0 = unrelated changes / lane violation.

### 4. Reliability (0–2)

Does the result survive a restart or re-run? 2 = `./init.sh` + verification pass
clean on a fresh run; 1 = passes with a manual nudge; 0 = flaky/breaks.

### 5. Maintainability (0–2)

Are code and docs clear enough for the next session? 2 = boundaries respected
(single BrewClient port), docs current; 1 = readable but undocumented; 0 =
opaque.

### 6. Handoff readiness (0–2)

Can a new session continue using repo artifacts only? 2 = progress log, handoff,
feature list, traces all current and sufficient; 1 = partial; 0 = needs chat
history.

## Scoring sheet

| dimension          | score (0–2) | notes |
| ------------------ | ----------- | ----- |
| Correctness        |             |       |
| Verification       |             |       |
| Scope discipline   |             |       |
| Reliability        |             |       |
| Maintainability    |             |       |
| Handoff readiness  |             |       |
| **Total / 12**     |             |       |

## Conclusion

- **Accept** — meets the bar (typically 10–12 with no zeros).
- **Revise** — needs fixes before accepting (any single 0, or total < 10).
- **Block** — fundamental issues (correctness or verification = 0).
