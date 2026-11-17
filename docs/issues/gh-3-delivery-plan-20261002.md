# Delivery plan for #3: DEN-3954: enforce repository/test-fleet audit across ORES Chat

Tracks https://github.com/ores-chat/.github/issues/3

## Scope

Translate the issue into bounded, independently reviewable delivery slices while keeping shared contracts and repository ownership intact.

## Rules

- Public, customer, and admin surfaces remain distinct in API, theme, audience, and authorization.
- Shared contracts/clients are reused rather than reimplemented in application or catalog repositories.
- Generated artifacts are reproducible projections with immutable source provenance.
- Browser/native surfaces must keep credentials and provider details out of distributable assets.
- Monorepo composition advances reviewed immutable child revisions rather than duplicating source.
- Fleet/test evidence records exact source heads and never calls skipped/zero-step jobs green.

## Verification

- Add deterministic fixtures for the issue's user-visible/runtime contract.
- Add negative coverage for realm confusion, stale generated output, missing dependency evidence, and unsupported platform capability.
- Run repository-local checks on the exact head.
- Keep documentation/catalog claims tied to executable evidence.

This draft advances #3; executable work and exact-head proof are still required before closure.
