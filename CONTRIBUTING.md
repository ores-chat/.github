# Contributing

All changes use a focused feature branch and pull request. State the owning repository, affected audience/realm, migration or compatibility impact, tests run, and security/telemetry implications.

## Required checks

- Keep commits explicit and reviewable.
- Pin source dependencies to immutable commits or locked versions.
- Run repository-local format, lint, test, contract, and smoke gates.
- Implement new CI checks, repo scripts, generators, auditors, and test harnesses in Rust, **not Python**. Do not add `.py` files or `python`/`python3`/`pip`/`pytest` steps to workflows. Keep native-language product tests where required (e.g., Flutter/Dart).
- For existing Python tooling, preserve positive/negative fixture coverage and error semantics while porting to Rust; remove the original Python scripts and invocations in the same proven-parity change. Document any required vendor-only exception.
- Keep credentials out of code, fixtures, logs, telemetry, and CLI defaults.
- Preserve public/customer/admin realm, endpoint, data, and theme separation.
- Update interfaces/docs/E2E when a cross-repository contract changes.

Do not use production systems as test fixtures. Deployed tests require complete disposable endpoints and short-lived credentials.

