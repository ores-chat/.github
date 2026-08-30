#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
policy = json.loads((root / "repository-policy.json").read_text(encoding="utf-8"))

assert policy["schemaVersion"] == 1
assert policy["defaultVisibility"] == "private"
assert policy["changeFlow"]["pullRequestRequired"] is True
assert policy["changeFlow"]["immutableDependencies"] is True

security = policy["security"]
assert security["sharedAuthOwnsIdentity"] is True
assert security["oresChatOwnsProductAuthorization"] is True
assert security["customerAdminFallback"] is False
assert security["secretsInGit"] is False
assert security["mcpReadOnly"] is True
assert security["sidecarLoopbackOnly"] is True

for document in ["README.md", "CONTRIBUTING.md", "SECURITY.md", "PULL_REQUEST_TEMPLATE.md", "profile/README.md"]:
    assert (root / document).is_file(), f"missing governance document: {document}"

print("organization policy: valid")

