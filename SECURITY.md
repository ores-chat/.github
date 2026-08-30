# Security policy

Report vulnerabilities through GitHub private vulnerability reporting in the affected repository. Do not open a public issue or include tokens, provider keys, database URLs, message bodies, customer context, or production identifiers.

Reports should identify the affected repository/version, boundary crossed, minimal reproduction with synthetic data, and expected security outcome.

High-priority boundaries include customer/admin authority crossover, public access to privileged context, Shared Auth verification bypass, credential exposure, writable direct-database access, MCP mutation, sidecar non-loopback exposure, and sensitive telemetry.

