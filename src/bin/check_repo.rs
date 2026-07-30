// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const REQUIRED_FILES: &[&str] = &[
    "README.md",
    "START_HERE.md",
    "Cargo.toml",
    "Cargo.lock",
    "src/main.rs",
    "src/bin/check_repo.rs",
    "scripts/doctor.sh",
    "docs/PROJECT_STRUCTURE.md",
    "docs/LOCAL_SETUP.md",
    "docs/GITHUB_WORKFLOW.md",
    "LICENSE",
    "NOTICE",
    "LEGAL_NOTICES.md",
    "DISCLAIMER.md",
    "SECURITY.md",
    "SUPPORT.md",
    "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md",
    "SPONSORS.md",
    "CHANGELOG.md",
    "ROADMAP.md",
    ".editorconfig",
    ".gitattributes",
    ".gitignore",
    ".markdownlint.json",
    ".github/FUNDING.yml",
    ".github/dependabot.yml",
    ".github/CODEOWNERS",
    ".github/PULL_REQUEST_TEMPLATE.md",
    ".github/ISSUE_TEMPLATE/config.yml",
    ".github/ISSUE_TEMPLATE/bug_report.yml",
    ".github/ISSUE_TEMPLATE/feature_request.yml",
    ".github/ISSUE_TEMPLATE/documentation_task.yml",
];

const SOURCE_REFERENCES: &[(&str, &[&str])] = &[
    (
        "Cargo.toml",
        &[
            "edition = \"2021\"",
            "license = \"Apache-2.0\"",
            "unsafe_code = \"forbid\"",
        ],
    ),
    ("Cargo.lock", &["name = \"test-project-tbd\""]),
    (
        "src/main.rs",
        &["#![forbid(unsafe_code)]", "GitHub staging lab ready."],
    ),
    (
        "src/bin/check_repo.rs",
        &["#![forbid(unsafe_code)]", "fn check_workflows"],
    ),
];

const SKIPPED_DIRS: &[&str] = &[
    ".git",
    ".venv",
    "__pycache__",
    "backups",
    "build",
    "coverage",
    "dist",
    "node_modules",
    "out",
    "target",
    "tmp",
    "temp",
    "worktrees",
];

const BINARY_SUFFIXES: &[&str] = &["gif", "ico", "jpeg", "jpg", "pdf", "png", "webp"];

const BLOCKED_FILENAMES: &[&str] = &[
    ".env",
    ".env.local",
    ".npmrc",
    ".pypirc",
    ".netrc",
    "credentials.json",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    "id_rsa",
    "service-account.json",
];

const BLOCKED_SECRET_SUFFIXES: &[&str] = &[".jks", ".key", ".keystore", ".p12", ".pfx"];

const PRIVATE_KEY_PREFIX: &str = "-----BEGIN ";
const PRIVATE_KEY_WORDS: [&str; 2] = ["PRIVATE ", "KEY-----"];

fn main() -> ExitCode {
    let root = match env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Repository check failed:");
            eprintln!("- could not read current directory: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut missing = Vec::new();
    let mut failures = Vec::new();

    check_required_files(&root, &mut missing);
    check_source_references(&root, &mut missing);
    check_evidence_artifacts(&root, &mut failures);
    check_sensitive_files(&root, &mut failures);
    check_workflows(&root, &mut failures);

    if missing.is_empty() && failures.is_empty() {
        println!("Repository check passed.");
        return ExitCode::SUCCESS;
    }

    println!("Repository check failed:");
    for item in missing.iter().chain(failures.iter()) {
        println!("- {item}");
    }
    ExitCode::FAILURE
}

fn check_required_files(root: &Path, missing: &mut Vec<String>) {
    for required_file in REQUIRED_FILES {
        if !root.join(required_file).is_file() {
            missing.push((*required_file).to_owned());
        }
    }
}

fn check_source_references(root: &Path, missing: &mut Vec<String>) {
    for (path, expected_values) in SOURCE_REFERENCES {
        let source_path = root.join(path);
        if !source_path.is_file() {
            continue;
        }

        let Ok(content) = fs::read_to_string(&source_path) else {
            missing.push(format!("{path} must be readable as UTF-8"));
            continue;
        };

        for expected in *expected_values {
            if !content.contains(expected) {
                missing.push(format!("{path} reference: {expected}"));
            }
        }
    }
}

fn check_evidence_artifacts(root: &Path, failures: &mut Vec<String>) {
    let evidence_root = root.join("issue-evidence");
    if !evidence_root.is_dir() {
        return;
    }

    for file_path in repo_files_under(root, &evidence_root, failures) {
        let relative_path = relative_path(root, &file_path);
        let lower_path = relative_path.to_ascii_lowercase();
        if !lower_path.contains("redacted")
            || lower_path.contains("unredacted")
            || lower_path.contains("nonredacted")
        {
            failures.push(format!("{relative_path} must be explicitly redacted"));
        }
    }
}

fn check_sensitive_files(root: &Path, failures: &mut Vec<String>) {
    for file_path in repo_files_under(root, root, failures) {
        let relative_path = relative_path(root, &file_path);

        if let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) {
            if is_blocked_filename(file_name) {
                failures.push(format!("{relative_path} is a blocked sensitive filename"));
            }
        }

        if is_binary_file(&file_path) {
            continue;
        }

        match fs::read_to_string(&file_path) {
            Ok(content) => check_secret_content(&relative_path, &content, failures),
            Err(error) if error.kind() == io::ErrorKind::InvalidData => {}
            Err(error) => failures.push(format!("{relative_path} could not be read: {error}")),
        }
    }
}

fn check_secret_content(relative_path: &str, content: &str, failures: &mut Vec<String>) {
    let private_key_suffix = PRIVATE_KEY_WORDS.concat();
    if content.contains(PRIVATE_KEY_PREFIX) && content.contains(&private_key_suffix) {
        failures.push(format!(
            "{relative_path} appears to contain private key block"
        ));
    }

    if contains_prefixed_token(content, &["ghp_", "gho_", "ghu_", "ghs_", "ghr_"], 30) {
        failures.push(format!("{relative_path} appears to contain GitHub token"));
    }

    if contains_prefixed_token(content, &["github_pat_"], 20) {
        failures.push(format!(
            "{relative_path} appears to contain GitHub fine-grained token"
        ));
    }

    if contains_aws_access_key(content) {
        failures.push(format!(
            "{relative_path} appears to contain AWS access key id"
        ));
    }

    if has_generic_secret_assignment(content) {
        failures.push(format!(
            "{relative_path} appears to contain generic secret assignment"
        ));
    }
}

fn check_workflows(root: &Path, failures: &mut Vec<String>) {
    let workflow_root = root.join(".github/workflows");
    if !workflow_root.is_dir() {
        return;
    }

    for workflow_path in repo_files_under(root, &workflow_root, failures) {
        if !matches!(
            workflow_path
                .extension()
                .and_then(|extension| extension.to_str()),
            Some("yml" | "yaml")
        ) {
            continue;
        }

        let relative_path = relative_path(root, &workflow_path);
        let Ok(content) = fs::read_to_string(&workflow_path) else {
            failures.push(format!("{relative_path} must be readable as UTF-8"));
            continue;
        };

        if content.contains("write-all") {
            failures.push(format!(
                "{relative_path} must not use write-all permissions"
            ));
        }

        for (line_number, line) in content.lines().enumerate() {
            let trimmed_line = line.trim();
            if trimmed_line == "contents: write" {
                failures.push(format!("{relative_path} must not grant contents: write"));
            }

            let step_line = trimmed_line.strip_prefix("- ").unwrap_or(trimmed_line);
            let Some(action_ref) = step_line.strip_prefix("uses:") else {
                continue;
            };

            let action_ref = action_ref.split_whitespace().next().unwrap_or("");
            if action_ref.starts_with("./") || action_ref.starts_with("docker://") {
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
}

fn repo_files_under(root: &Path, directory: &Path, failures: &mut Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_repo_files(root, directory, failures, &mut files);
    files
}

fn collect_repo_files(
    root: &Path,
    directory: &Path,
    failures: &mut Vec<String>,
    files: &mut Vec<PathBuf>,
) {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) => {
            failures.push(format!(
                "{} could not be read: {error}",
                relative_path(root, directory)
            ));
            return;
        }
    };

    for entry in entries {
        let Ok(entry) = entry else {
            failures.push(format!(
                "{} contains an unreadable directory entry",
                relative_path(root, directory)
            ));
            continue;
        };

        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            failures.push(format!(
                "{} file type could not be read",
                relative_path(root, &path)
            ));
            continue;
        };

        if file_type.is_dir() {
            if should_skip_dir(root, &path) {
                continue;
            }
            collect_repo_files(root, &path, failures, files);
        } else if file_type.is_file() {
            files.push(path);
        }
    }
}

fn should_skip_dir(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .and_then(|relative_path| relative_path.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .is_some_and(|first_part| SKIPPED_DIRS.contains(&first_part))
}

fn is_blocked_filename(file_name: &str) -> bool {
    let lower_name = file_name.to_ascii_lowercase();
    BLOCKED_FILENAMES.contains(&lower_name.as_str())
        || (lower_name.starts_with(".env.") && lower_name != ".env.example")
        || BLOCKED_SECRET_SUFFIXES
            .iter()
            .any(|suffix| lower_name.ends_with(suffix))
}

fn is_binary_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| BINARY_SUFFIXES.contains(&extension))
}

fn contains_prefixed_token(content: &str, prefixes: &[&str], minimum_tail_len: usize) -> bool {
    content
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .any(|token| {
            prefixes.iter().any(|prefix| {
                token.starts_with(prefix) && token.len() >= prefix.len() + minimum_tail_len
            })
        })
}

fn contains_aws_access_key(content: &str) -> bool {
    content
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| {
            token.len() == 20
                && (token.starts_with("AKIA") || token.starts_with("ASIA"))
                && token
                    .chars()
                    .all(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
        })
}

fn has_generic_secret_assignment(content: &str) -> bool {
    content.lines().any(|line| {
        let lower_line = line.to_ascii_lowercase();
        let has_secret_key = ["api_key", "api-key", "secret", "token", "password"]
            .iter()
            .any(|keyword| lower_line.contains(keyword));

        if !has_secret_key {
            return false;
        }

        let Some(separator_index) = line.find(['=', ':']) else {
            return false;
        };

        let assignment_value = line[separator_index + 1..].trim_start();
        let Some(quote) = assignment_value.chars().next() else {
            return false;
        };

        if quote != '"' && quote != '\'' {
            return false;
        }

        let Some(end_index) = assignment_value[1..].find(quote) else {
            return false;
        };

        let value = &assignment_value[1..=end_index];
        value.len() >= 20
            && value
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || "_./+=:-".contains(character))
    })
}

fn is_full_sha(ref_name: &str) -> bool {
    ref_name.len() == 40
        && ref_name
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::{
        contains_aws_access_key, contains_prefixed_token, has_generic_secret_assignment,
        is_blocked_filename, is_full_sha, should_skip_dir,
    };
    use std::path::Path;

    #[test]
    fn detects_github_token_shape() {
        let probe = format!("token = 'ghp_{}'", "a".repeat(36));
        assert!(contains_prefixed_token(&probe, &["ghp_"], 30));
    }

    #[test]
    fn detects_aws_access_key_shape() {
        let probe = ["AKIA", "ABCDEFGHIJKLMNOP"].concat();
        assert!(contains_aws_access_key(&probe));
    }

    #[test]
    fn detects_generic_secret_assignment() {
        let keyword = ["pass", "word"].concat();
        let probe = format!("{keyword} = '{}'", "abcdefghijklmnopqrstuvwxyz");
        assert!(has_generic_secret_assignment(&probe));
    }

    #[test]
    fn requires_full_lowercase_sha() {
        assert!(is_full_sha("9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0"));
        assert!(!is_full_sha("v4"));
    }

    #[test]
    fn blocks_environment_and_key_container_filenames() {
        assert!(is_blocked_filename(".env.production"));
        assert!(is_blocked_filename("service-account.json"));
        assert!(is_blocked_filename("signing.P12"));
        assert!(!is_blocked_filename(".env.example"));
        assert!(!is_blocked_filename("public-certificate.pem"));
    }

    #[test]
    fn skips_only_reserved_top_level_local_work_areas() {
        let root = Path::new("/repo");
        assert!(should_skip_dir(root, &root.join("worktrees/transmission")));
        assert!(should_skip_dir(root, &root.join("backups/archive")));
        assert!(!should_skip_dir(root, &root.join("docs/worktrees")));
    }
}
