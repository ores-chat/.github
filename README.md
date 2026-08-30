# ORES Chat organization governance

Shared workflows, contribution policy, security policy, and the public organization profile.

This repository now supplies the public organization profile, contribution workflow, private vulnerability-reporting policy, pull-request checklist, and a machine-validated repository/security platform contract.

The policy defaults repositories to private, requires pull requests and immutable dependencies, separates Shared Auth identity from ORES Chat product authorization, prohibits customer/admin fallback and secrets in Git, and fixes MCP/sidecar boundaries as read-only/loopback-only.

No CODEOWNERS file is invented because the organization currently has no teams. Add one after real ownership teams exist. Run `python3 scripts/validate.py` and the JSON syntax gate.
