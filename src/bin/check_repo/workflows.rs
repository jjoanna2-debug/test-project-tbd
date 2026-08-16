// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

pub(crate) fn check_workflow_content(
    relative_path: &str,
    content: &str,
    failures: &mut Vec<String>,
) {
    let mut permissions_indent = None;

    for (line_number, raw_line) in content.lines().enumerate() {
        let line = strip_yaml_comment(raw_line);
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        if permissions_indent.is_some_and(|block_indent| indent <= block_indent) {
            permissions_indent = None;
        }

        if let Some((key, value)) = yaml_key_value(trimmed_line) {
            let scalar_value = trim_yaml_scalar(value);
            if key == "permissions" {
                if scalar_value == "write-all" {
                    failures.push(format!(
                        "{relative_path}:{} must not use write-all permissions",
                        line_number + 1
                    ));
                }

                if inline_permissions_grant_contents_write(value) {
                    failures.push(format!(
                        "{relative_path}:{} must not grant contents: write",
                        line_number + 1
                    ));
                }

                if value.is_empty() {
                    permissions_indent = Some(indent);
                }
            } else if key == "contents"
                && scalar_value == "write"
                && permissions_indent.is_some_and(|block_indent| indent > block_indent)
            {
                failures.push(format!(
                    "{relative_path}:{} must not grant contents: write",
                    line_number + 1
                ));
            }
        }

        let step_line = trimmed_line.strip_prefix("- ").unwrap_or(trimmed_line);
        let Some((key, value)) = yaml_key_value(step_line) else {
            continue;
        };
        if key != "uses" {
            continue;
        }

        let action_ref = trim_yaml_scalar(value)
            .split_whitespace()
            .next()
            .unwrap_or("");

        if action_ref.starts_with("./") {
            continue;
        }

        if action_ref.starts_with("docker://") {
            if !is_pinned_docker_reference(action_ref) {
                failures.push(format!(
                    "{relative_path}:{} Docker action must use a sha256 digest",
                    line_number + 1
                ));
            }
            continue;
        }

        let Some((_, ref_name)) = action_ref.rsplit_once('@') else {
            failures.push(format!(
                "{relative_path}:{} action is unpinned",
                line_number + 1
            ));
            continue;
        };

        if !is_full_sha(ref_name) {
            failures.push(format!(
                "{relative_path}:{} action must be pinned to a full SHA",
                line_number + 1
            ));
        }
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

    #[test]
    fn parses_workflow_permissions_in_context() {
        let safe = r#"
permissions:
  contents: read
jobs:
  smoke:
    steps:
      - name: Example input
        with:
          contents: write
# permissions: write-all
"#;
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
