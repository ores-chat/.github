# ORES Chat

ORES Chat is a private, context-agnostic conversation platform for embedding vertically separated public, signed-in user, and administrator chat into independent applications.

The platform orchestrates Anthropic, Gemini, and OpenAI over host-supplied, bounded, read-only context. Shared Auth verifies identity, ORES Chat owns product authorization, opto-sync carries credential-free product data, ores-otel supplies redacted telemetry, flags2env defines audited runtime configuration, and zed-pkg coordinates immutable dependencies.

## Audience boundaries

- Public website chat uses allowlisted marketing context and the external theme.
- Authenticated user chat uses the customer realm, tenant-scoped context, and user theme.
- Administrator chat uses a separate admin realm, MFA, private ingress, separate data authority, and admin theme.

Customer and administrator authority never fall back to one another.

Most repositories are private. Security reports must use GitHub private vulnerability reporting in the affected repository rather than public issues.

