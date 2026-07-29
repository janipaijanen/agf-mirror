use agov_core::{schema, yaml};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A parsed and schema-valid Gate 1 plan document (frontmatter only).
#[derive(Debug, Clone)]
pub struct PlanDocument {
    pub raw: serde_json::Value,
}

/// Validate a Gate 1 plan file.
///
/// Mirrors `Agov.PlanValidator.validate_file/1`:
///   - Read file → extract frontmatter → parse YAML → validate schema
///   - Validate semantics: required advisory roles present, no unknown roles,
///     no duplicate roles.
///
/// Reads role statuses from `agf/agents/MANIFEST.yaml` via `$AGOV_MANIFEST_PATH`.
/// Falls back to the two required roles when the manifest is unreadable.
pub fn validate_file(path: &Path) -> Result<PlanDocument, Vec<(String, String)>> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(vec![("file not found".to_string(), String::new())]);
        }
        Err(err) => return Err(vec![(format!("{err:?}"), String::new())]),
    };

    let raw = match yaml::parse_frontmatter_json(&content) {
        Ok(value) => value,
        Err(yaml::YamlError::NoFrontmatter) => {
            return Err(vec![(
                "no YAML front matter found".to_string(),
                String::new(),
            )]);
        }
        Err(err) => return Err(vec![(format!("{err:?}"), String::new())]),
    };

    let schema_path = agov_core::schema_dir().join("gate1-plan.schema.json");
    let compiled = schema::load_and_compile(&schema_path)
        .map_err(|err| vec![(format!("{err:?}"), schema_path.display().to_string())])?;
    let mut errors: Vec<(String, String)> = schema::validate(&compiled, &raw)
        .into_iter()
        .map(|err| (err.message, err.pointer))
        .collect();

    if errors.is_empty() {
        errors.extend(validate_semantics(&raw));
    }

    if errors.is_empty() {
        Ok(PlanDocument { raw })
    } else {
        Err(errors)
    }
}

pub fn required_roles() -> &'static [&'static str] {
    &["infosec-sentinel", "contribution-auditor"]
}

pub fn validate_semantics(data: &serde_json::Value) -> Vec<(String, String)> {
    let manifest = yaml::read_yaml_file_json(&agov_core::manifest_path()).ok();
    validate_semantics_with_manifest(data, manifest.as_ref())
}

fn validate_semantics_with_manifest(
    data: &serde_json::Value,
    manifest: Option<&serde_json::Value>,
) -> Vec<(String, String)> {
    let roles: Vec<String> = data
        .get("advisory_consultation")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("role").and_then(serde_json::Value::as_str))
        .map(ToOwned::to_owned)
        .collect();

    let role_statuses = role_statuses(manifest);
    let mut errors = Vec::new();

    for required in required_roles() {
        if !roles.iter().any(|role| role == required) {
            errors.push((
                format!("missing required advisory role: {required}"),
                "#/advisory_consultation".to_string(),
            ));
        }
    }

    let mut unavailable: Vec<String> = roles
        .iter()
        .filter(|role| {
            role_statuses
                .get(role.as_str())
                .is_none_or(|status| status != "active")
        })
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    unavailable.sort();
    for role in unavailable {
        let message = match role_statuses.get(role.as_str()) {
            Some(status) => format!("advisory role is not active: {role} (status: {status})"),
            None => format!("unknown advisory role: {role}"),
        };
        errors.push((message, "#/advisory_consultation".to_string()));
    }

    let mut frequencies = HashMap::<&str, usize>::new();
    for role in &roles {
        *frequencies.entry(role.as_str()).or_default() += 1;
    }

    let mut duplicates: Vec<&str> = frequencies
        .into_iter()
        .filter_map(|(role, count)| (count > 1).then_some(role))
        .collect();
    duplicates.sort();
    for role in duplicates {
        errors.push((
            format!("duplicate advisory role: {role}"),
            "#/advisory_consultation".to_string(),
        ));
    }

    errors
}

fn role_statuses(manifest: Option<&serde_json::Value>) -> HashMap<String, String> {
    let mut roles: HashMap<String, String> = required_roles()
        .iter()
        .map(|role| (role.to_string(), "active".to_string()))
        .collect();

    let Some(manifest) = manifest else {
        return roles;
    };

    let Some(agents) = manifest
        .get("agents")
        .and_then(serde_json::Value::as_object)
    else {
        return roles;
    };

    for (role, attrs) in agents {
        if let Some(status) = attrs.get("status").and_then(serde_json::Value::as_str) {
            roles.insert(role.clone(), status.to_string());
        }
    }

    roles
}

#[cfg(test)]
mod tests {
    use super::validate_semantics_with_manifest;
    use serde_json::json;

    fn plan(role: &str) -> serde_json::Value {
        json!({
            "advisory_consultation": [
                {"role": "infosec-sentinel", "status": "consulted"},
                {"role": "contribution-auditor", "status": "consulted"},
                {"role": role, "status": "consulted"}
            ]
        })
    }

    fn manifest() -> serde_json::Value {
        json!({"agents": {
            "fact-checker": {"status": "active"},
            "build-engineer": {"status": "suspended"},
            "documentation-writer": {"status": "skipped"}
        }})
    }

    #[test]
    fn accepts_active_manifest_role() {
        assert!(
            validate_semantics_with_manifest(&plan("fact-checker"), Some(&manifest())).is_empty()
        );
    }

    #[test]
    fn distinguishes_suspended_and_skipped_roles_from_unknown_roles() {
        let manifest = manifest();
        for (role, status) in [
            ("build-engineer", "suspended"),
            ("documentation-writer", "skipped"),
        ] {
            let errors = validate_semantics_with_manifest(&plan(role), Some(&manifest));
            assert!(errors.iter().any(|(message, _)| message
                == &format!("advisory role is not active: {role} (status: {status})")));
        }

        let errors = validate_semantics_with_manifest(&plan("not-a-role"), Some(&manifest));
        assert!(errors
            .iter()
            .any(|(message, _)| message == "unknown advisory role: not-a-role"));
    }

    #[test]
    fn missing_manifest_falls_back_to_required_roles_only() {
        let required_only = json!({"advisory_consultation": [
            {"role": "infosec-sentinel", "status": "consulted"},
            {"role": "contribution-auditor", "status": "consulted"}
        ]});
        assert!(validate_semantics_with_manifest(&required_only, None).is_empty());

        let errors = validate_semantics_with_manifest(&plan("fact-checker"), None);
        assert!(errors
            .iter()
            .any(|(message, _)| message == "unknown advisory role: fact-checker"));
    }
}
