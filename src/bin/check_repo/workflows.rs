// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

#[derive(Default)]
struct WorkflowState {
    permissions_indent: Option<usize>,
    trigger_indent: Option<usize>,
    has_top_level_permissions: bool,
    checkout_step: Option<(usize, bool)>,
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
        update_block_state(&mut state, indent);

        if trimmed_line.starts_with("- ") {
            finish_checkout_step(relative_path, &mut state.checkout_step, failures);
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
            line_number,
            &mut state.checkout_step,
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

fn update_block_state(state: &mut WorkflowState, indent: usize) {
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
    let Some((key, value)) = yaml_key_value(trimmed_line) else {
        return;
    };
    let scalar_value = trim_yaml_scalar(value);

    if key == "on" && value.is_empty() {
        state.trigger_indent = Some(indent);
    } else if key == "pull_request_target"
        && state
            .trigger_indent
            .is_some_and(|block_indent| indent > block_indent)
    {
        failures.push(format!(
            "{relative_path}:{line_number} must not use pull_request_target"
        ));
    }

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

    if key == "persist-credentials"
        && scalar_value == "false"
        && let Some((_, credentials_disabled)) = state.checkout_step.as_mut()
    {
        *credentials_disabled = true;
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

        if scalar_value == "write-all" {
            failures.push(format!(
                "{relative_path}:{line_number} must not use write-all permissions"
            ));
        }
        if inline_permissions_grant_contents_write(value) {
            failures.push(format!(
                "{relative_path}:{line_number} must not grant contents: write"
            ));
        }
        if value.is_empty() {
            state.permissions_indent = Some(indent);
        }
    } else if key == "contents"
        && scalar_value == "write"
        && state
            .permissions_indent
            .is_some_and(|block_indent| indent > block_indent)
    {
        failures.push(format!(
            "{relative_path}:{line_number} must not grant contents: write"
        ));
    }
}

fn check_action_reference(
    relative_path: &str,
    trimmed_line: &str,
    line_number: usize,
    checkout_step: &mut Option<(usize, bool)>,
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
        *checkout_step = Some((line_number, false));
    }

    if action_ref.starts_with("./") {
        return;
    }

    if action_ref.starts_with("docker://") {
        if !is_pinned_docker_reference(action_ref) {
            failures.push(format!(
                "{relative_path}:{line_number} Docker action must use a sha256 digest"
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
            "{relative_path}:{line_number} action must be pinned to a full SHA"
        ));
    }
}

fn finish_checkout_step(
    relative_path: &str,
    checkout_step: &mut Option<(usize, bool)>,
    failures: &mut Vec<String>,
) {
    if let Some((line_number, false)) = checkout_step.take() {
        failures.push(format!(
            "{relative_path}:{line_number} actions/checkout must set persist-credentials: false"
        ));
    }
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

fn inline_permissions_grant_contents_write(value: &str) -> bool {
    let Some(inner) = value
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
    else {
        return false;
    };

    inner.split(',').any(|entry| {
        yaml_key_value(entry)
            .is_some_and(|(key, value)| key == "contents" && trim_yaml_scalar(value) == "write")
    })
}

fn is_pinned_docker_reference(reference: &str) -> bool {
    let Some((image, digest)) = reference.rsplit_once("@sha256:") else {
        return false;
    };

    image.len() > "docker://".len()
        && digest.len() == 64
        && digest
            .chars()
            .all(|character| character.is_ascii_hexdigit())
}

fn is_full_sha(ref_name: &str) -> bool {
    ref_name.len() == 40
        && ref_name
            .chars()
            .all(|character| character.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::{check_workflow_content, is_full_sha, is_pinned_docker_reference};

    const PINNED_CHECKOUT: &str = "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1";

    #[test]
    fn parses_workflow_permissions_in_context() {
        let safe = r"
permissions:
  contents: read
jobs:
  smoke:
    steps:
      - name: Example input
        with:
          contents: write
# permissions: write-all
";
        let unsafe_workflow = r#"
permissions: "write-all"
jobs:
  smoke:
    permissions:
      contents: 'write'
"#;

        let mut safe_failures = Vec::new();
        check_workflow_content("safe.yml", safe, &mut safe_failures);
        assert!(safe_failures.is_empty());

        let mut unsafe_failures = Vec::new();
        check_workflow_content("unsafe.yml", unsafe_workflow, &mut unsafe_failures);
        assert_eq!(unsafe_failures.len(), 2);
    }

    #[test]
    fn enforces_workflow_trust_boundaries() {
        let safe = format!(
            "on:\n  pull_request:\npermissions:\n  contents: read\njobs:\n  smoke:\n    steps:\n      - uses: {PINNED_CHECKOUT}\n        with:\n          persist-credentials: false\n"
        );
        let unsafe_workflow = format!(
            "on:\n  pull_request_target:\njobs:\n  smoke:\n    steps:\n      - uses: {PINNED_CHECKOUT}\n"
        );

        let mut safe_failures = Vec::new();
        check_workflow_content("safe.yml", &safe, &mut safe_failures);
        assert!(safe_failures.is_empty());

        let mut unsafe_failures = Vec::new();
        check_workflow_content("unsafe.yml", &unsafe_workflow, &mut unsafe_failures);
        assert_eq!(unsafe_failures.len(), 3);
    }

    #[test]
    fn detects_inline_write_permissions() {
        let workflow = "permissions: { contents: 'write', issues: read }";
        let mut failures = Vec::new();
        check_workflow_content("inline.yml", workflow, &mut failures);
        assert_eq!(failures.len(), 1);
    }

    #[test]
    fn requires_docker_actions_to_use_sha256_digests() {
        let digest = "a".repeat(64);
        assert!(is_pinned_docker_reference(&format!(
            "docker://alpine@sha256:{digest}"
        )));
        assert!(!is_pinned_docker_reference("docker://alpine:latest"));
    }

    #[test]
    fn accepts_full_hexadecimal_action_shas() {
        assert!(is_full_sha("9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0"));
        assert!(is_full_sha("9C091BB21B7C1C1D1991BB908D89E4E9DDDFE3E0"));
        assert!(!is_full_sha("v4"));
    }
}
