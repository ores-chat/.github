use serde_json::Value;
use std::{fs, path::Path, process};

fn check_policy(policy: &Value) -> Result<(), String> {
    let eq = |pointer: &str, want: Value| -> Result<(), String> {
        match policy.pointer(pointer) {
            Some(value) if *value == want => Ok(()),
            _ => Err(format!("repository policy mismatch: {pointer}")),
        }
    };

    eq("/schemaVersion", Value::from(1))?;
    eq("/defaultVisibility", Value::from("private"))?;
    eq("/changeFlow/pullRequestRequired", Value::from(true))?;
    eq("/changeFlow/immutableDependencies", Value::from(true))?;
    eq(
        "/changeFlow/requiredFiles",
        serde_json::json!(["README.md", "AGENTS.md", ".github/workflows/ci.yml"]),
    )?;

    for (name, want) in [
        ("sharedAuthOwnsIdentity", true),
        ("oresChatOwnsProductAuthorization", true),
        ("customerAdminFallback", false),
        ("secretsInGit", false),
        ("mcpReadOnly", true),
        ("sidecarLoopbackOnly", true),
    ] {
        eq(&format!("/security/{name}"), Value::from(want))?;
    }

    // Enforce the Rust-first repository automation contract explicitly.
    for (field, expected) in [
        ("scriptsAndValidators", Value::from("rust")),
        ("testHarnesses", Value::from("rust")),
        ("newPythonToolingAllowed", Value::from(false)),
        ("nativeLanguageTestsAllowed", Value::from(true)),
    ] {
        eq(&format!("/tooling/{field}"), expected)?;
    }

    // Shared platform choices are contract constraints, not unchecked metadata.
    for (name, want) in [
        ("flags", "flags-2-env"),
        ("sync", "opto-sync"),
        ("telemetry", "ores-otel"),
        ("packages", "zed-pkg"),
        ("encryptedEnvironment", "env/enc"),
    ] {
        eq(&format!("/platforms/{name}"), Value::from(want))?;
    }
    Ok(())
}

fn check(root: &Path) -> Result<(), String> {
    let data = fs::read_to_string(root.join("repository-policy.json"))
        .map_err(|e| format!("cannot read repository policy: {e}"))?;
    let policy: Value =
        serde_json::from_str(&data).map_err(|e| format!("invalid repository policy JSON: {e}"))?;
    check_policy(&policy)?;

    for document in [
        "README.md",
        "CONTRIBUTING.md",
        "SECURITY.md",
        "PULL_REQUEST_TEMPLATE.md",
        "profile/README.md",
    ] {
        if !root.join(document).is_file() {
            return Err(format!("missing governance document: {document}"));
        }
    }
    Ok(())
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if let Err(error) = check(&root) {
        eprintln!("{error}");
        process::exit(1);
    }
    println!("organization policy: valid");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> Value {
        serde_json::from_str(include_str!("../../../repository-policy.json")).unwrap()
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(serde_json::from_str::<Value>("{").is_err());
    }

    #[test]
    fn current_policy_is_valid() {
        assert_eq!(check_policy(&policy()), Ok(()));
    }

    #[test]
    fn rejects_weakening_customer_admin_isolation() {
        let mut policy = policy();
        policy["security"]["customerAdminFallback"] = Value::from(true);
        assert!(
            check_policy(&policy)
                .unwrap_err()
                .contains("customerAdminFallback")
        );
    }

    #[test]
    fn rejects_platform_authority_substitution() {
        for name in [
            "flags",
            "sync",
            "telemetry",
            "packages",
            "encryptedEnvironment",
        ] {
            let mut policy = policy();
            policy["platforms"][name] = Value::from("unreviewed");
            assert!(
                check_policy(&policy).unwrap_err().contains(name),
                "{name} drift must fail closed"
            );
        }
    }

    #[test]
    fn tooling_must_use_rust_instead_of_new_python() {
        for (field, invalid) in [
            ("scriptsAndValidators", Value::from("python")),
            ("testHarnesses", Value::from("python")),
            ("newPythonToolingAllowed", Value::from(true)),
            ("nativeLanguageTestsAllowed", Value::from(false)),
        ] {
            let mut policy = policy();
            policy["tooling"][field] = invalid;
            assert!(
                check_policy(&policy).unwrap_err().contains(field),
                "{field} drift must fail closed"
            );
        }
    }

    #[test]
    fn rejects_dropped_required_repository_files() {
        let mut policy = policy();
        policy["changeFlow"]["requiredFiles"] =
            serde_json::json!(["README.md", "AGENTS.md"]);
        assert!(
            check_policy(&policy)
                .unwrap_err()
                .contains("requiredFiles")
        );
    }
}
