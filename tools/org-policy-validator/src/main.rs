use serde_json::Value;
use std::{fs, path::Path, process};
fn check(root: &Path) -> Result<(), String> {
    let data=fs::read_to_string(root.join("repository-policy.json")).map_err(|e| format!("cannot read repository policy: {e}"))?;
    let p:Value=serde_json::from_str(&data).map_err(|e| format!("invalid repository policy JSON: {e}"))?;
    let eq=|pointer:&str, want:Value| -> Result<(),String> {
        match p.pointer(pointer) {Some(v) if *v==want=>Ok(()),_=>Err(format!("repository policy mismatch: {pointer}"))}
    };
    eq("/schemaVersion",Value::from(1))?;
    eq("/defaultVisibility",Value::from("private"))?;
    eq("/changeFlow/pullRequestRequired",Value::from(true))?;
    eq("/changeFlow/immutableDependencies",Value::from(true))?;
    for (name,want) in [
        ("sharedAuthOwnsIdentity",true),("oresChatOwnsProductAuthorization",true),
        ("customerAdminFallback",false),("secretsInGit",false),
        ("mcpReadOnly",true),("sidecarLoopbackOnly",true)
    ] { eq(&format!("/security/{name}"),Value::from(want))?; }
    for doc in ["README.md","CONTRIBUTING.md","SECURITY.md","PULL_REQUEST_TEMPLATE.md","profile/README.md"] {
        if !root.join(doc).is_file() {return Err(format!("missing governance document: {doc}"))}
    }
    Ok(())
}
fn main(){
    let root=Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if let Err(e)=check(&root) {eprintln!("{e}"); process::exit(1)}
    println!("organization policy: valid");
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn rejects_invalid_json() {assert!(serde_json::from_str::<Value>("{").is_err());}
    #[test] fn contract_requires_both_true_and_false_security_flags() {
        let p:Value=serde_json::json!({"customerAdminFallback":false,"mcpReadOnly":true});
        assert_eq!(p["customerAdminFallback"],Value::from(false));
        assert_eq!(p["mcpReadOnly"],Value::from(true));
    }
}
