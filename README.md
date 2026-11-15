# ORES Chat organization governance

Shared workflows, contribution policy, security policy, and the public organization profile.

This repository now supplies the public organization profile, contribution workflow, private vulnerability-reporting policy, pull-request checklist, and a machine-validated repository/security platform contract.

The policy defaults repositories to private, requires pull requests and immutable dependencies, separates Shared Auth identity from ORES Chat product authorization, prohibits customer/admin fallback and secrets in Git, and fixes MCP/sidecar boundaries as read-only/loopback-only.

No CODEOWNERS file is invented because the organization currently has no teams. Add one after real ownership teams exist.

## Rust-first scripts and tests

ORES Chat repository automation, CI validators, security checks, generators, and portable regression harnesses must use **Rust rather than Python**. Do not add Python scripts or Python-based CI invocations. Preserve native-language product tests (such as Flutter/Dart tests). When migrating legacy Python, retain golden and adversarial fixture parity, switch CI to the Rust replacement, then remove the old Python implementation; a vendor-mandated exception must be documented and reviewed.

Run the actual organization-policy gates:

```sh
cargo fmt --manifest-path tools/org-policy-validator/Cargo.toml -- --check
cargo test --manifest-path tools/org-policy-validator/Cargo.toml --all-targets
cargo run --quiet --manifest-path tools/org-policy-validator/Cargo.toml --
```
