# Tech Debt Tracker

Known shortcuts, their cost, and the repayment plan. Add an entry whenever a
decision trades long-term cleanliness for short-term progress. Review before
benchmark runs and cleanup passes.

| id | item | why taken | cost / risk | repayment plan | status |
| -- | ---- | --------- | ----------- | -------------- | ------ |
| TD-001 | No app code; validation ladder is aspirational | Harness-first bootstrap | Cannot mechanically prove product behavior yet | Scaffold Spring Boot, wire `./gradlew test` into `init.sh` `VERIFY_CMD` | open |
| TD-002 | Harness CLI not built/compiled in CI | No Rust toolchain at authoring time | CLI could fail to build | Run `scripts/build-cli.sh`; add a CI job | open |
| TD-003 | Phase-5 CLI commands (audit/propose/intervention) not implemented | Scope of v0 | No automated drift/intervention tracking | Implement when friction justifies it (see HARNESS_BACKLOG) | open |

Close an item by setting status to `done` and noting the commit/plan that repaid
it.
