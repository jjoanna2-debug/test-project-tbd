// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

const ALLOWED_TRIGGERS: &[&str] = &[
    "merge_group",
    "pull_request",
    "push",
    "schedule",
    "workflow_dispatch",
];

#[derive(Default)]
struct WorkflowState {
    permissions_indent: Option<usize>,
    trigger_indent: Option<usize>,
    trigger_event_indent: Option<usize>,
    steps_indent: Option<usize>,
    step_item_indent: Option<usize>,
    has_top_level_permissions: bool,
    checkout_step: Option<CheckoutStep>,
}

struct CheckoutStep {
    line_number: usize,
    step_indent: usize,
    credentials_disabled: bool,
    unsafe_checkout_enabled: bool,
}

pub(crate) fn check_workflow_content(
    relative_path: &str,
    content: &str,
    failures: &mut Vec<String>,
) {
    let mut state = WorkflowState::default();

    for (line_number, raw_line) in content.lines().enumerate() {
        let line = strip_yaml_comment(raw_line);
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let line_number = line_number + 1;
        let indent = line.len() - line.trim_start().len();
        update_block_state(
            relative_path,
            indent,
            &mut state,
            failures,
        );
        start_step_if_needed(
            relative_path,
            trimmed_line,
            indent,
            &mut state,
            failures,
        );

        if contains_yaml_indirection(trimmed_line) {
            failures.push(format!(
                "{relative_path}:{line_number} must not use YAML anchors, aliases, or merge keys"
            ));
        }

        check_yaml_policy(
            relative_path,
            trimmed_line,
            indent,
            line_number,
            &mut state,
            failures,
        );
        check_action_reference(
            relative_path,
            trimmed_line,
            indent,
            line_number,
            &mut state,
            failures,
        );
    }

    finish_checkout_step(relative_path, &mut state.checkout_step, failures);
    if !state.has_top_level_permissions {
        failures.push(format!(
            "{relative_path} must declare top-level workflow permissions"
        ));
    }
}

fn update_block_state(
    relative_path: &str,
    indent: usize,
    state: &mut WorkflowState,
    failures: &mut Vec<String>,
) {
    if state
        .permissions_indent
        .is_some_and(|block_indent| indent <= block_indent)
    {
        state.permissions_indent = None;
    }
    if state
        .trigger_indent
        .is_some_and(|block_indent| indent <= block_indent)
    {
        state.trigger_indent = None;
        state.trigger_event_indent = None;
    }
    if state
        .steps_indent
        .is_some_and(|block_indent| indent <= block_indent)
    {
        finish_checkout_step(relative_path, &mut state.checkout_step, failures);
        state.steps_indent = None;
        state.step_item_indent = None;
    }
}

fn start_step_if_needed(
    relative_path: &str,
    trimmed_line: &str,
    indent: usize,
    state: &mut WorkflowState,
    failures: &mut Vec<String>,
) {
    if !trimmed_line.starts_with("- ") {
        return;
    }

    let Some(steps_indent) = state.steps_indent else {
        return;
    };
    if indent <= steps_indent {
        return;
    }

    let step_item_indent = state.step_item_indent.get_or_insert(indent);
    if indent == *step_item_indent {
        finish_checkout_step(relative_path, &mut state.checkout_step, failures);
    }
}

fn check_yaml_policy(
    relative_path: &str,
    trimmed_line: &str,
    indent: usize,
    line_number: usize,
    state: &mut WorkflowState,
    failures: &mut Vec<String>,
) {
    let step_line = trimmed_line.strip_prefix("- ").unwrap_or(trimmed_line);
    let Some((key, value)) = yaml_key_value(step_line) else {
        return;
    };
    let scalar_value = trim_yaml_scalar(value);

    check_trigger(
        relative_path,
        key,
        value,
        indent,
        line_number,
        state,
        failures,
    );
    check_permissions(
        relative_path,
        key,
        value,
        scalar_value,
        indent,
        line_number,
        state,
        failures,
    );

    if key == "steps" && value.is_empty() {
        state.steps_indent = Some(indent);
        state.step_item_indent = None;
    }

    check_checkout_option(
        relative_path,
        key,
        scalar_value,
        indent,
        line_number,
        &mut state.checkout_step,
        failures,
    );
}

#[allow(clippy::too_many_arguments)]
fn check_trigger(
    relative_path: &str,
    key: &str,
    value: &str,
    indent: usize,
    line_number: usize,
    state: &mut WorkflowState,
    failures: &mut Vec<String>,
) {
    if key == "on" {
        state.trigger_event_indent = None;
        if value.is_empty() {
            state.trigger_indent = Some(indent);
        } else {
            for trigger in inline_trigger_names(value) {
                reject_disallowed_trigger(relative_path, trigger, line_number, failures);
            }
        }
        return;
    }

    let Some(trigger_indent) = state.trigger_indent else {
        return;
    };
    if indent <= trigger_indent {
        return;
    }

    let event_indent = state.trigger_event_indent.get_or_insert(indent);
    if indent == *event_indent {
        reject_disallowed_trigger(relative_path, key, line_number, failures);
    }
}

fn inline_trigger_names(value: &str) -> impl Iterator<Item = &str> {
    value
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(trim_yaml_scalar)
        .filter(|trigger| !trigger.is_empty())
}

fn reject_disallowed_trigger(
    relative_path: &str,
    trigger: &str,
    line_number: usize,
    failures: &mut Vec<String>,
) {
    if !ALLOWED_TRIGGERS.contains(&trigger) {
        failures.push(format!(
            "{relative_path}:{line_number} trigger {trigger} is not allowed"
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn check_permissions(
    relative_path: &str,
    key: &str,
    value: &str,
    scalar_value: &str,
    indent: usize,
    line_number: usize,
    state: &mut WorkflowState,
    failures: &mut Vec<String>,
) {
    if key == "permissions" {
        state.has_top_level_permissions |= indent == 0;

        if value.is_empty() {
            state.permissions_indent = Some(indent);
            return;
        }

        if scalar_value == "write-all" {
            failures.push(format!(
                "{relative_path}:{line_number} must not use write-all permissions"
            ));
            return;
        }

        if let Some(scope) = inline_write_permission_scope(value) {
            failures.push(format!(
                "{relative_path}:{line_number} must not grant {scope}: write"
            ));
            return;
        }

        if !matches!(scalar_value, "read-all" | "{}") && !value.trim_start().starts_with('{') {
            failures.push(format!(
                "{relative_path}:{line_number} permissions must be an explicit read-only map, read-all, or an empty map"
            ));
        }
        return;
    }

    if scalar_value == "write"
        && state
            .permissions_indent
            .is_some_and(|block_indent| indent > block_indent)
    {
        failures.push(format!(
            "{relative_path}:{line_number} must not grant {key}: write"
        ));
    }
}

fn inline_write_permission_scope(value: &str) -> Option<&str> {
    let inner = value
        .trim()
        .strip_prefix('{')?
        .strip_suffix('}')?;

    inner.split(',').find_map(|entry| {
        yaml_key_value(entry).and_then(|(key, value)| {
            (trim_yaml_scalar(value) == "write").then_some(key)
        })
    })
}

fn check_checkout_option(
    relative_path: &str,
    key: &str,
    scalar_value: &str,
    indent: usize,
    line_number: usize,
    checkout_step: &mut Option<CheckoutStep>,
    failures: &mut Vec<String>,
) {
    let Some(checkout) = checkout_step.as_mut() else {
        return;
    };
    if indent <= checkout.step_indent {
        return;
    }

    if key == "persist-credentials" {
        if scalar_value == "false" {
            checkout.credentials_disabled = true;
        } else {
            failures.push(format!(
                "{relative_path}:{line_number} actions/checkout persist-credentials must be false"
            ));
        }
    } else if key == "allow-unsafe-pr-checkout" && scalar_value == "true" {
        checkout.unsafe_checkout_enabled = true;
        failures.push(format!(
            "{relative_path}:{line_number} actions/checkout must not enable allow-unsafe-pr-checkout"
        ));
    }
}

fn check_action_reference(
    relative_path: &str,
    trimmed_line: &str,
    indent: usize,
    line_number: usize,
    state: &mut WorkflowState,
    failures: &mut Vec<String>,
) {
    let step_line = trimmed_line.strip_prefix("- ").unwrap_or(trimmed_line);
    let Some((key, value)) = yaml_key_value(step_line) else {
        return;
    };
    if key != "uses" {
        return;
    }

    let action_ref = trim_yaml_scalar(value)
        .split_whitespace()
        .next()
        .unwrap_or("");
    let action_name = action_ref
        .split_once('@')
        .map_or(action_ref, |(name, _)| name);
    if action_name == "actions/checkout" {
        let step_indent = state.step_item_indent.unwrap_or(indent);
        state.checkout_step = Some(CheckoutStep {
            line_number,
            step_indent,
            credentials_disabled: false,
            unsafe_checkout_enabled: false,
        });
    }

    if action_ref.starts_with("./") {
        return;
    }

    if action_ref.starts_with("docker://") {
        if !is_pinned_docker_reference(action_ref) {
            failures.push(format!(
                "{relative_path}:{line_number} Docker action must use a lowercase sha256 digest"
            ));
        }
        return;
    }

    let Some((_, ref_name)) = action_ref.rsplit_once('@') else {
        failures.push(format!("{relative_path}:{line_number} action is unpinned"));
        return;
    };

    if !is_full_sha(ref_name) {
        failures.push(format!(
            "{relative_path}:{line_number} action must be pinned to a full lowercase SHA"
        ));
    }
}

fn finish_checkout_step(
    relative_path: &str,
    checkout_step: &mut Option<CheckoutStep>,
    failures: &mut Vec<String>,
) {
    let Some(checkout) = checkout_step.take() else {
        return;
    };

    if !checkout.credentials_disabled {
        failures.push(format!(
            "{relative_path}:{} actions/checkout must set persist-credentials: false",
            checkout.line_number
        ));
    }
    if checkout.unsafe_checkout_enabled {
        return;
    }
}

fn contains_yaml_indirection(line: &str) -> bool {
    let step_line = line.strip_prefix("- ").unwrap_or(line).trim_start();
    if step_line.starts_with('*') || step_line.starts_with('&') {
        return true;
    }

    let Some((key, value)) = yaml_key_value(step_line) else {
        return false;
    };
    if key == "<<" {
        return true;
    }

    let raw_value = value.trim_start();
    if raw_value.starts_with(['\'', '"']) {
        return false;
    }

    raw_value
        .trim_start_matches(['[', '{', ','])
        .trim_start()
        .starts_with(['*', '&'])
}

fn strip_yaml_comment(line: &str) -> &str {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;

    for (index, character) in line.char_indices() {
        if character == '\\' && in_double_quotes {
            escaped = !escaped;
            continue;
        }

        if character == '"' && !in_single_quotes && !escaped {
            in_double_quotes = !in_double_quotes;
        } else if character == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;
        } else if character == '#'
            && !in_single_quotes
            && !in_double_quotes
            && (index == 0
                || line[..index]
                    .chars()
                    .next_back()
                    .is_some_and(char::is_whitespace))
        {
            return &line[..index];
        }

        if character != '\\' {
            escaped = false;
        }
    }

    line
}

fn yaml_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((trim_yaml_scalar(key), value.trim()))
}

fn trim_yaml_scalar(value: &str) -> &str {
    value
        .trim()
        .trim_matches(|character| character == '"' || character == '\'')
}

fn is_pinned_docker_reference(reference: &str) -> bool {
    let Some((image, digest)) = reference.rsplit_once("@sha256:") else {
        return false;
    };

    image.len() > "docker://".len()
        && digest.len() == 64
        && digest
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
}

fn is_full_sha(ref_name: &str) -> bool {
    ref_name.len() == 40
        && ref_name
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
}

#[cfg(test)]
mod tests {
    use super::{check_workflow_content, is_full_sha, is_pinned_docker_reference};

    const PINNED_CHECKOUT: &str = "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1";

    #[test]
    fn accepts_read_only_permissions_and_nested_step_lists() {
        let workflow = format!(
            "on:\n  pull_request:\npermissions:\n  contents: read\njobs:\n  smoke:\n    steps:\n      - uses: {PINNED_CHECKOUT}\n        with:\n          sparse-checkout: |\n            src\n            docs\n          persist-credentials: false\n      - run: cargo test\n"
        );
        let mut failures = Vec::new();
        check_workflow_content("safe.yml", &workflow, &mut failures);
        assert!(failures.is_empty(), "{failures:?}");
    }

    #[test]
    fn rejects_every_write_permission_scope() {
        let workflow = r#"
on: push
permissions:
  contents: read
  id-token: write
jobs:
  smoke:
    permissions: { contents: read, issues: write }
"#;
        let mut failures = Vec::new();
        check_workflow_content("permissions.yml", workflow, &mut failures);
        assert_eq!(failures.len(), 2, "{failures:?}");
    }

    #[test]
    fn rejects_non_allowlisted_triggers() {
        let workflow = r#"
on:
  pull_request_target:
  workflow_run:
  issue_comment:
permissions:
  contents: read
"#;
        let mut failures = Vec::new();
        check_workflow_content("triggers.yml", workflow, &mut failures);
        assert_eq!(failures.len(), 3, "{failures:?}");
    }

    #[test]
    fn rejects_yaml_indirection() {
        let workflow = r#"
on: push
permissions: &read_permissions
  contents: read
jobs:
  smoke:
    permissions: *read_permissions
    strategy:
      <<: *defaults
"#;
        let mut failures = Vec::new();
        check_workflow_content("anchors.yml", workflow, &mut failures);
        assert_eq!(failures.len(), 3, "{failures:?}");
    }

    #[test]
    fn rejects_unsafe_checkout_mode() {
        let workflow = format!(
            "on: pull_request\npermissions: read-all\njobs:\n  smoke:\n    steps:\n      - uses: {PINNED_CHECKOUT}\n        with:\n          persist-credentials: false\n          allow-unsafe-pr-checkout: true\n"
        );
        let mut failures = Vec::new();
        check_workflow_content("unsafe-checkout.yml", &workflow, &mut failures);
        assert_eq!(failures.len(), 1, "{failures:?}");
    }

    #[test]
    fn requires_explicit_top_level_permissions() {
        let mut failures = Vec::new();
        check_workflow_content("missing.yml", "on: push\n", &mut failures);
        assert_eq!(failures.len(), 1);
    }

    #[test]
    fn requires_docker_actions_to_use_lowercase_sha256_digests() {
        let digest = "a".repeat(64);
        assert!(is_pinned_docker_reference(&format!(
            "docker://alpine@sha256:{digest}"
        )));
        assert!(!is_pinned_docker_reference("docker://alpine:latest"));
        assert!(!is_pinned_docker_reference(&format!(
            "docker://alpine@sha256:{}",
            "A".repeat(64)
        )));
    }

    #[test]
    fn accepts_only_full_lowercase_action_shas() {
        assert!(is_full_sha("9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0"));
        assert!(!is_full_sha("9C091BB21B7C1C1D1991BB908D89E4E9DDDFE3E0"));
        assert!(!is_full_sha("v4"));
    }
}
