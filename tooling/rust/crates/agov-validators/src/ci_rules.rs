//! Small, fail-closed evaluator for the subset of GitLab `rules:` used by
//! this repository's top-level workflow and governance-validation job.
//!
//! This is structural local-model evidence, not a replacement for GitLab CI
//! Lint or a live pipeline. Unsupported clauses return an error instead of
//! being guessed as admitted or selected.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default)]
pub struct CiContext {
    variables: BTreeMap<String, String>,
    changed_paths: BTreeSet<String>,
}

impl CiContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variable(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(name.into(), value.into());
        self
    }

    pub fn with_changed_path(mut self, path: impl Into<String>) -> Self {
        self.changed_paths.insert(path.into());
        self
    }

    fn variable(&self, name: &str) -> &str {
        self.variables.get(name).map(String::as_str).unwrap_or("")
    }
}

pub fn pipeline_admitted(workflow_rules: &Value, context: &CiContext) -> Result<bool, String> {
    evaluate_rules(workflow_rules, context, "workflow")
}

pub fn job_selected(job_rules: &Value, context: &CiContext) -> Result<bool, String> {
    evaluate_rules(job_rules, context, "job")
}

fn evaluate_rules(rules: &Value, context: &CiContext, label: &str) -> Result<bool, String> {
    let rules = rules
        .as_array()
        .ok_or_else(|| format!("{label} rules must be an array"))?;

    for (index, rule) in rules.iter().enumerate() {
        let rule = rule
            .as_object()
            .ok_or_else(|| format!("{label} rules[{index}] must be an object"))?;
        for key in rule.keys() {
            if !matches!(key.as_str(), "if" | "when" | "changes" | "variables") {
                return Err(format!(
                    "{label} rules[{index}] uses unsupported key `{key}`"
                ));
            }
        }

        let if_matches = match rule.get("if") {
            Some(Value::String(expression)) => evaluate_expression(expression, context)
                .map_err(|error| format!("{label} rules[{index}]: {error}"))?,
            Some(_) => {
                return Err(format!(
                    "{label} rules[{index}].if must be a string expression"
                ))
            }
            None => true,
        };
        let changes_match = match rule.get("changes") {
            Some(changes) => evaluate_changes(changes, context)
                .map_err(|error| format!("{label} rules[{index}]: {error}"))?,
            None => true,
        };

        if if_matches && changes_match {
            return match rule.get("when").and_then(Value::as_str) {
                None | Some("always") | Some("on_success") => Ok(true),
                Some("never") => Ok(false),
                Some(value) => Err(format!(
                    "{label} rules[{index}] uses unsupported `when: {value}`"
                )),
            };
        }
    }

    Ok(false)
}

fn evaluate_expression(expression: &str, context: &CiContext) -> Result<bool, String> {
    if expression.contains("||")
        || expression.contains("=~")
        || expression.contains("!~")
        || expression.contains('(')
        || expression.contains(')')
    {
        return Err(format!("unsupported rules expression `{expression}`"));
    }

    expression
        .split("&&")
        .map(str::trim)
        .try_fold(true, |matches, clause| {
            if clause.is_empty() {
                return Err(format!("empty clause in rules expression `{expression}`"));
            }
            let clause_matches = evaluate_clause(clause, context)?;
            Ok(matches && clause_matches)
        })
}

fn evaluate_clause(clause: &str, context: &CiContext) -> Result<bool, String> {
    if let Some((left, right)) = clause.split_once("!=") {
        return Ok(resolve_operand(left, context)? != resolve_operand(right, context)?);
    }
    if let Some((left, right)) = clause.split_once("==") {
        return Ok(resolve_operand(left, context)? == resolve_operand(right, context)?);
    }

    let variable = clause
        .trim()
        .strip_prefix('$')
        .filter(|name| valid_variable_name(name))
        .ok_or_else(|| format!("unsupported rules clause `{clause}`"))?;
    Ok(!context.variable(variable).is_empty())
}

fn resolve_operand<'a>(operand: &'a str, context: &'a CiContext) -> Result<&'a str, String> {
    let operand = operand.trim();
    if let Some(variable) = operand.strip_prefix('$') {
        if valid_variable_name(variable) {
            return Ok(context.variable(variable));
        }
        return Err(format!("invalid CI variable operand `{operand}`"));
    }
    if operand.len() >= 2 {
        let bytes = operand.as_bytes();
        let quoted = (bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'');
        if quoted {
            return Ok(&operand[1..operand.len() - 1]);
        }
    }
    Err(format!("unsupported rules operand `{operand}`"))
}

fn valid_variable_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn evaluate_changes(changes: &Value, context: &CiContext) -> Result<bool, String> {
    let patterns = match changes {
        Value::Array(patterns) => patterns,
        Value::Object(object) => {
            for key in object.keys() {
                if !matches!(key.as_str(), "paths" | "compare_to") {
                    return Err(format!("unsupported changes key `{key}`"));
                }
            }
            object
                .get("paths")
                .and_then(Value::as_array)
                .ok_or_else(|| "changes object must contain a paths array".to_string())?
        }
        _ => return Err("changes must be an array or a paths object".to_string()),
    };

    let patterns = patterns
        .iter()
        .map(|pattern| {
            let pattern = pattern
                .as_str()
                .ok_or_else(|| "every changes pattern must be a string".to_string())?;
            if pattern
                .chars()
                .any(|character| matches!(character, '?' | '[' | ']' | '{' | '}' | '\\'))
            {
                return Err(format!("unsupported changes glob `{pattern}`"));
            }
            Ok(pattern)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(patterns.iter().any(|pattern| {
        context
            .changed_paths
            .iter()
            .any(|path| glob_matches(pattern, path))
    }))
}

fn glob_matches(pattern: &str, path: &str) -> bool {
    let pattern = pattern.as_bytes();
    let path = path.as_bytes();
    let mut memo = BTreeMap::new();

    fn matches_from(
        pattern: &[u8],
        path: &[u8],
        pattern_index: usize,
        path_index: usize,
        memo: &mut BTreeMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(result) = memo.get(&(pattern_index, path_index)) {
            return *result;
        }
        let result = if pattern_index == pattern.len() {
            path_index == path.len()
        } else if pattern[pattern_index] == b'*' {
            let double_star = pattern.get(pattern_index + 1) == Some(&b'*');
            let next_pattern = pattern_index + if double_star { 2 } else { 1 };
            let zero_directories = double_star
                && pattern.get(next_pattern) == Some(&b'/')
                && matches_from(pattern, path, next_pattern + 1, path_index, memo);
            zero_directories
                || matches_from(pattern, path, next_pattern, path_index, memo)
                || (path_index < path.len()
                    && (double_star || path[path_index] != b'/')
                    && matches_from(pattern, path, pattern_index, path_index + 1, memo))
        } else {
            path_index < path.len()
                && pattern[pattern_index] == path[path_index]
                && matches_from(pattern, path, pattern_index + 1, path_index + 1, memo)
        };
        memo.insert((pattern_index, path_index), result);
        result
    }

    matches_from(pattern, path, 0, 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_rules() -> (Value, Value) {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../");
        let content = std::fs::read_to_string(root.join(".gitlab-ci.yml")).unwrap();
        let workflow_start = content.find("workflow:\n").unwrap();
        let workflow_end = content[workflow_start..]
            .find("\n.rust-base:")
            .map(|offset| workflow_start + offset)
            .unwrap();
        let job_start = content.find("agov:governance:validate:\n").unwrap();
        let job_end = content[job_start..]
            .find("\n# Validate AGF framework content")
            .map(|offset| job_start + offset)
            .unwrap();
        let workflow: Value =
            serde_norway::from_str(&content[workflow_start..workflow_end]).unwrap();
        let job: Value = serde_norway::from_str(&content[job_start..job_end]).unwrap();
        (
            workflow.pointer("/workflow/rules").unwrap().clone(),
            job.pointer("/agov:governance:validate/rules")
                .unwrap()
                .clone(),
        )
    }

    fn context(source: &str) -> CiContext {
        CiContext::new()
            .with_variable("CI_PIPELINE_SOURCE", source)
            .with_variable("CI_DEFAULT_BRANCH", "main")
            .with_changed_path("agf/rules/workflow/new-example-rule.md")
    }

    #[test]
    fn live_workflow_and_governance_job_match_expected_contexts() {
        let (workflow, job) = repository_rules();

        let admitted = [
            context("merge_request_event")
                .with_variable("CI_COMMIT_BRANCH", "feature")
                .with_variable("CI_OPEN_MERGE_REQUESTS", "1"),
            context("push").with_variable("CI_COMMIT_BRANCH", "feature"),
            context("push").with_variable("CI_COMMIT_BRANCH", "main"),
            context("push").with_variable("CI_COMMIT_TAG", "v2.6.1"),
        ];
        for candidate in admitted {
            assert!(pipeline_admitted(&workflow, &candidate).unwrap());
            assert!(job_selected(&job, &candidate).unwrap());
        }

        let duplicate_push = context("push")
            .with_variable("CI_COMMIT_BRANCH", "feature")
            .with_variable("CI_OPEN_MERGE_REQUESTS", "1");
        assert!(!pipeline_admitted(&workflow, &duplicate_push).unwrap());
    }

    #[test]
    fn first_matching_rule_controls_the_result() {
        let rules = serde_json::json!([
            {"if": "$CI_COMMIT_BRANCH", "when": "never"},
            {"when": "always"}
        ]);
        let context = CiContext::new().with_variable("CI_COMMIT_BRANCH", "main");
        assert!(!job_selected(&rules, &context).unwrap());
    }

    #[test]
    fn unsupported_expression_fails_closed() {
        let rules = serde_json::json!([
            {"if": "$CI_COMMIT_BRANCH =~ /main/"},
            {"when": "always"}
        ]);
        assert!(job_selected(&rules, &CiContext::new()).is_err());

        let hidden_unsupported = serde_json::json!([
            {"if": "$CI_COMMIT_BRANCH && $CI_COMMIT_TAG =~ /v.*/"}
        ]);
        assert!(job_selected(&hidden_unsupported, &CiContext::new()).is_err());
    }

    #[test]
    fn changes_globs_distinguish_single_and_double_star() {
        let one_segment = serde_json::json!([{"changes": ["agf/rules/*.md"]}]);
        let recursive = serde_json::json!([{"changes": ["agf/rules/**/*.md"]}]);
        let context = CiContext::new().with_changed_path("agf/rules/workflow/example.md");
        assert!(!job_selected(&one_segment, &context).unwrap());
        assert!(job_selected(&recursive, &context).unwrap());

        let direct_child = CiContext::new().with_changed_path("agf/rules/example.md");
        assert!(job_selected(&recursive, &direct_child).unwrap());
    }
}
