// Static topology check for the shared Rust CI cache (Slice B).
//
// Inspects GitLab CI Lint's *merged* YAML (extends/anchors already
// resolved) rather than the source .gitlab-ci.yml, so a job that becomes a
// shared-cache consumer through indirect `extends` (e.g. via
// .rust-tooling-package) is still caught -- a hard-coded job-name list
// missed exactly this case during Slice A (rust:tooling:preview and
// rust:tooling:build:windows:preview silently skipped the policy
// assertion because before_script arrays don't merge under `extends`).
// See docs/plans/ci-rust-cache-policy-slice-b.md.
//
// The check operates on which CI variable each job's `cache.policy`
// references (e.g. "$RUST_CACHE_WRITER_POLICY" vs "$RUST_CACHE_POLICY"),
// not on the runtime-resolved pull/pull-push value -- GitLab CI Lint does
// not evaluate variables, so the resolved value is never visible statically.
// Runtime resolution is covered separately by
// .gitlab/scripts/check-rust-cache-policy.sh's assert-current mode.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobCacheRole {
    pub name: String,
    pub role: Option<String>,
    pub policy_var: Option<String>,
}

/// Inventory every top-level concrete job (not a hidden `.template`) whose
/// merged `cache.key` equals `shared_key`.
pub fn inventory_shared_cache_jobs(doc: &Value, shared_key: &str) -> Vec<JobCacheRole> {
    let mut out = Vec::new();
    let Some(map) = doc.as_object() else {
        return out;
    };
    for (name, job) in map {
        if name.starts_with('.') {
            continue; // hidden/template job, never a concrete pipeline job
        }
        let Some(job_obj) = job.as_object() else {
            continue;
        };
        let Some(cache) = job_obj.get("cache").and_then(Value::as_object) else {
            continue;
        };
        let Some(key) = cache.get("key").and_then(Value::as_str) else {
            continue;
        };
        if key != shared_key {
            continue;
        }
        let role = job_obj
            .get("variables")
            .and_then(Value::as_object)
            .and_then(|v| v.get("RUST_CACHE_ROLE"))
            .and_then(Value::as_str)
            .map(str::to_string);
        let policy_var = cache.get("policy").and_then(Value::as_str).map(str::to_string);
        out.push(JobCacheRole {
            name: name.clone(),
            role,
            policy_var,
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

#[derive(Debug)]
pub struct TopologyReport {
    pub shared_key_jobs: Vec<JobCacheRole>,
}

/// Renders the resolved writer/consumer graph as Graphviz DOT, independent
/// of whether the topology passes `check_topology` -- useful as diagnostic
/// output when a check fails (a second writer, an unrecognised role, or a
/// job on the wrong policy variable are all visible directly in the graph
/// shape rather than only in an error-message list).
pub fn render_dot(jobs: &[JobCacheRole], shared_key: &str) -> String {
    let mut out = String::new();
    out.push_str("digraph rust_cache_topology {\n");
    out.push_str("  rankdir=LR;\n");
    out.push_str("  node [fontname=\"sans-serif\"];\n");
    out.push_str(&format!(
        "  \"{shared_key}\" [shape=cylinder, style=filled, fillcolor=\"#fff2b2\"];\n"
    ));
    for job in jobs {
        let role_label = job.role.as_deref().unwrap_or("(none)");
        let policy_label = job.policy_var.as_deref().unwrap_or("(none)");
        let (fill, edge) = match job.role.as_deref() {
            Some("writer") => (
                "#b7f0ad",
                format!(
                    "  \"{}\" -> \"{shared_key}\" [label=\"write\", color=\"#1a7d1a\", penwidth=2];\n",
                    job.name
                ),
            ),
            Some("consumer") => (
                "#add8ff",
                format!(
                    "  \"{shared_key}\" -> \"{}\" [label=\"read\", color=\"#2a6fbf\", style=dashed];\n",
                    job.name
                ),
            ),
            _ => (
                "#ffb3b3",
                format!(
                    "  \"{}\" -> \"{shared_key}\" [label=\"role?\", color=\"#c0392b\", style=dotted, penwidth=2];\n",
                    job.name
                ),
            ),
        };
        out.push_str(&format!(
            "  \"{}\" [shape=box, style=filled, fillcolor=\"{fill}\", label=\"{}\\nrole={role_label}\\npolicy={policy_label}\"];\n",
            job.name, job.name
        ));
        out.push_str(&edge);
    }
    out.push_str("}\n");
    out
}

/// Fails unless the shared-cache graph contains exactly one writer-role job,
/// named `writer_job`, consuming `writer_policy_var`; every other job in the
/// graph must be consumer-role and consume `consumer_policy_var`. Any job
/// with a missing or unrecognised role fails closed rather than being
/// treated as a safe default.
pub fn check_topology(
    doc: &Value,
    shared_key: &str,
    writer_job: &str,
    writer_policy_var: &str,
    consumer_policy_var: &str,
) -> Result<TopologyReport, Vec<String>> {
    let jobs = inventory_shared_cache_jobs(doc, shared_key);
    let mut errors = Vec::new();
    let writer_var = format!("${writer_policy_var}");
    let consumer_var = format!("${consumer_policy_var}");

    let writers: Vec<&JobCacheRole> = jobs
        .iter()
        .filter(|j| j.role.as_deref() == Some("writer"))
        .collect();
    if writers.len() != 1 {
        let names: Vec<&str> = writers.iter().map(|j| j.name.as_str()).collect();
        errors.push(format!(
            "expected exactly 1 writer-role job in the '{shared_key}' shared-cache graph, found {}: {names:?}",
            writers.len()
        ));
    } else if writers[0].name != writer_job {
        errors.push(format!(
            "writer-role job is '{}', expected '{writer_job}'",
            writers[0].name
        ));
    }

    for job in &jobs {
        match job.role.as_deref() {
            Some("writer") => {
                if job.policy_var.as_deref() != Some(writer_var.as_str()) {
                    errors.push(format!(
                        "writer job '{}' cache.policy is {:?}, expected '{writer_var}'",
                        job.name, job.policy_var
                    ));
                }
            }
            Some("consumer") => {
                if job.policy_var.as_deref() != Some(consumer_var.as_str()) {
                    errors.push(format!(
                        "consumer job '{}' cache.policy is {:?}, expected '{consumer_var}'",
                        job.name, job.policy_var
                    ));
                }
            }
            other => {
                errors.push(format!(
                    "job '{}' has unrecognised RUST_CACHE_ROLE {other:?} (must be 'writer' or 'consumer')",
                    job.name
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(TopologyReport {
            shared_key_jobs: jobs,
        })
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHARED_KEY: &str = "rust-v3-main";
    const WRITER_JOB: &str = "rust:tooling:test";
    const WRITER_VAR: &str = "RUST_CACHE_WRITER_POLICY";
    const CONSUMER_VAR: &str = "RUST_CACHE_POLICY";

    fn job(cache_key: &str, role: Option<&str>, policy_var: &str) -> Value {
        let mut job = serde_json::json!({
            "cache": {
                "key": cache_key,
                "policy": policy_var,
            },
        });
        if let Some(role) = role {
            job["variables"] = serde_json::json!({ "RUST_CACHE_ROLE": role });
        }
        job
    }

    fn doc(jobs: &[(&str, Value)]) -> Value {
        let mut map = serde_json::Map::new();
        map.insert(
            ".rust-base".to_string(),
            job(SHARED_KEY, Some("consumer"), "$RUST_CACHE_POLICY"),
        );
        map.insert(
            "stages".to_string(),
            serde_json::json!(["test", "build"]),
        );
        for (name, value) in jobs {
            map.insert((*name).to_string(), value.clone());
        }
        Value::Object(map)
    }

    fn valid_doc() -> Value {
        doc(&[
            (
                WRITER_JOB,
                job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
            (
                "rust:tooling:clippy",
                job(SHARED_KEY, Some("consumer"), "$RUST_CACHE_POLICY"),
            ),
            (
                "rust:tooling:oci-pull-test",
                job("rust-oci-pull-test", None, "pull-push"),
            ),
        ])
    }

    #[test]
    fn positive_valid_topology_passes() {
        let report = check_topology(&valid_doc(), SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect("valid topology must pass");
        // The hidden .rust-base template is excluded; only concrete jobs
        // count, and oci-pull-test's separate key excludes it too.
        assert_eq!(report.shared_key_jobs.len(), 2);
    }

    #[test]
    fn negative_hidden_template_excluded() {
        // .rust-base itself is never counted as a job even though it has
        // the shared key -- confirms the leading-dot filter actually runs,
        // not just that the happy path incidentally passes.
        let d = doc(&[(
            WRITER_JOB,
            job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
        )]);
        let report =
            check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR).expect("must pass");
        assert!(!report
            .shared_key_jobs
            .iter()
            .any(|j| j.name == ".rust-base"));
    }

    #[test]
    fn negative_second_writer_rejected() {
        let d = doc(&[
            (
                WRITER_JOB,
                job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
            (
                "rust:tooling:clippy",
                job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
        ]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("two writers must be rejected");
        assert!(errors.iter().any(|e| e.contains("exactly 1 writer-role job")));
    }

    #[test]
    fn negative_wrong_named_writer_rejected() {
        let d = doc(&[(
            "rust:tooling:clippy",
            job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
        )]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("wrong-named writer must be rejected");
        assert!(errors.iter().any(|e| e.contains("expected 'rust:tooling:test'")));
    }

    #[test]
    fn negative_writer_using_consumer_policy_var_rejected() {
        let d = doc(&[(
            WRITER_JOB,
            job(SHARED_KEY, Some("writer"), "$RUST_CACHE_POLICY"),
        )]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("writer on the consumer policy var must be rejected");
        assert!(errors
            .iter()
            .any(|e| e.contains("writer job 'rust:tooling:test' cache.policy")));
    }

    #[test]
    fn negative_consumer_using_writer_policy_var_rejected() {
        let d = doc(&[
            (
                WRITER_JOB,
                job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
            (
                "rust:tooling:clippy",
                job(SHARED_KEY, Some("consumer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
        ]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("a second job resolving the writer policy var must be rejected");
        assert!(errors
            .iter()
            .any(|e| e.contains("consumer job 'rust:tooling:clippy' cache.policy")));
    }

    #[test]
    fn negative_missing_role_fails_closed() {
        let d = doc(&[
            (
                WRITER_JOB,
                job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
            (
                "rust:tooling:clippy",
                job(SHARED_KEY, None, "$RUST_CACHE_POLICY"),
            ),
        ]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("a job with no role must fail closed, not default to consumer");
        assert!(errors.iter().any(|e| e.contains("unrecognised RUST_CACHE_ROLE")));
    }

    #[test]
    fn negative_unknown_role_fails_closed() {
        let d = doc(&[
            (
                WRITER_JOB,
                job(SHARED_KEY, Some("writer"), "$RUST_CACHE_WRITER_POLICY"),
            ),
            (
                "rust:tooling:clippy",
                job(SHARED_KEY, Some("bystander"), "$RUST_CACHE_POLICY"),
            ),
        ]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("an unrecognised role string must fail closed");
        assert!(errors
            .iter()
            .any(|e| e.contains("unrecognised RUST_CACHE_ROLE Some(\"bystander\")")));
    }

    #[test]
    fn negative_no_writer_at_all_rejected() {
        let d = doc(&[(
            "rust:tooling:clippy",
            job(SHARED_KEY, Some("consumer"), "$RUST_CACHE_POLICY"),
        )]);
        let errors = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect_err("zero writers must be rejected, not treated as vacuously fine");
        assert!(errors.iter().any(|e| e.contains("found 0")));
    }

    #[test]
    fn dot_render_marks_writer_consumer_and_unknown_role_distinctly() {
        let jobs = vec![
            JobCacheRole {
                name: WRITER_JOB.to_string(),
                role: Some("writer".to_string()),
                policy_var: Some("$RUST_CACHE_WRITER_POLICY".to_string()),
            },
            JobCacheRole {
                name: "rust:tooling:clippy".to_string(),
                role: Some("consumer".to_string()),
                policy_var: Some("$RUST_CACHE_POLICY".to_string()),
            },
            JobCacheRole {
                name: "rust:tooling:mystery".to_string(),
                role: None,
                policy_var: None,
            },
        ];
        let dot = render_dot(&jobs, SHARED_KEY);
        assert!(dot.starts_with("digraph rust_cache_topology {"));
        assert!(dot.contains(&format!("\"{WRITER_JOB}\" -> \"{SHARED_KEY}\" [label=\"write\"")));
        assert!(dot.contains(&format!(
            "\"{SHARED_KEY}\" -> \"rust:tooling:clippy\" [label=\"read\""
        )));
        assert!(dot.contains("\"rust:tooling:mystery\" -> \"rust-v3-main\" [label=\"role?\""));
    }

    #[test]
    fn separately_keyed_oci_pull_test_job_is_excluded() {
        // rust:tooling:oci-pull-test never extends .rust-base and carries no
        // RUST_CACHE_ROLE; confirm it is excluded by key, not accidentally
        // counted as a role-less shared-cache job.
        let d = valid_doc();
        let report = check_topology(&d, SHARED_KEY, WRITER_JOB, WRITER_VAR, CONSUMER_VAR)
            .expect("valid doc must pass");
        assert!(!report
            .shared_key_jobs
            .iter()
            .any(|j| j.name == "rust:tooling:oci-pull-test"));
    }
}
