# Contributing

All changes use a focused feature branch and pull request. State the owning repository, affected audience/realm, migration or compatibility impact, tests run, and security/telemetry implications.

## Required checks

- Keep commits explicit and reviewable.
- Pin source dependencies to immutable commits or locked versions.
- Run repository-local format, lint, test, contract, and smoke gates.
- Keep credentials out of code, fixtures, logs, telemetry, and CLI defaults.
- Preserve public/customer/admin realm, endpoint, data, and theme separation.
- Update interfaces/docs/E2E when a cross-repository contract changes.

Do not use production systems as test fixtures. Deployed tests require complete disposable endpoints and short-lived credentials.

