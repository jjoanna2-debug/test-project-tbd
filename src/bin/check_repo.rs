// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

#[path = "check_repo/secrets.rs"]
mod secrets;
#[path = "check_repo/workflows.rs"]
mod workflows;

use secrets::check_secret_content;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use workflows::check_workflow_content;

const REQUIRED_FILES: &[&str] = &[
    "README.md",
    "START_HERE.md",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "src/main.rs",
    "src/bin/check_repo.rs",
    "src/bin/check_repo/secrets.rs",
    "src/bin/check_repo/workflows.rs",
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
            "edition = \"2024\"",
            "rust-version = \"1.97.1\"",
            "license = \"Apache-2.0\"",
            "publish = false",
            "unsafe_code = \"forbid\"",
            "dbg_macro = \"deny\"",
            "todo = \"deny\"",
            "unimplemented = \"deny\"",
        ],
    ),
    ("Cargo.lock", &["name = \"test-project-tbd\""]),
    (
        "rust-toolchain.toml",
        &[
            "channel = \"1.97.1\"",
            "profile = \"minimal\"",
            "components = [\"clippy\", \"rustfmt\"]",
        ],
    ),
    (
        ".github/workflows/basic-checks.yml",
        &[
            "merge_group:",
            "workflow_dispatch:",
            "cancel-in-progress: true",
            "FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: \"true\"",
            "cargo check --locked --all-targets --all-features",
            "cargo clippy --locked --all-targets --all-features -- -D warnings",
            "cargo test --locked --all-targets --all-features",
            "cargo doc --locked --no-deps --document-private-items",
            "cargo run --quiet --locked --bin check_repo",
        ],
    ),
    (
        ".github/dependabot.yml",
        &[
            "routine-actions:",
            "routine-rust:",
            "applies-to: \"version-updates\"",
            "timezone: \"Europe/Lisbon\"",
        ],
    ),
    (
        "src/main.rs",
        &["#![forbid(unsafe_code)]", "GitHub staging lab ready."],
    ),
    (
        "src/bin/check_repo.rs",
        &[
            "#![forbid(unsafe_code)]",
            "check_repo/secrets.rs",
            "check_repo/workflows.rs",
        ],
    ),
    (
        "src/bin/check_repo/secrets.rs",
        &[
            "pub(crate) fn check_secret_content",
            "fn secret_assignment_score",
        ],
    ),
    (
        "src/bin/check_repo/workflows.rs",
        &[
            "pub(crate) fn check_workflow_content",
            "fn is_pinned_docker_reference",
        ],
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

const BINARY_SUFFIXES: &[&str] = &[
    "7z", "a", "avi", "bmp", "class", "dmg", "doc", "docx", "dylib", "eot", "exe", "flac", "gif",
    "gz", "ico", "jar", "jpeg", "jpg", "m4a", "mov", "mp3", "mp4", "o", "otf", "pdf", "png", "ppt",
    "pptx", "so", "tar", "tiff", "ttf", "wav", "webm", "webp", "woff", "woff2", "xls", "xlsx",
    "zip",
];
const MAX_TEXT_FILE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_SCAN_ENTRIES: usize = 20_000;

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

const BLOCKED_SECRET_SUFFIXES: &[&str] = &[".jks", ".key", ".kdbx", ".keystore", ".p12", ".pfx"];

const BLOCKED_SENSITIVE_PATHS: &[&str] = &[
    ".aws/credentials",
    ".config/gcloud/application_default_credentials.json",
    ".docker/config.json",
    ".kube/config",
];

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

    let repository_files = repo_files_under(&root, &root, &mut failures);
    check_repository_files(&root, &repository_files, &mut failures);

    missing.sort_unstable();
    missing.dedup();
    failures.sort_unstable();
    failures.dedup();

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

        match fs::metadata(&source_path) {
            Ok(metadata) if is_oversized_text_file(metadata.len()) => {
                missing.push(format!("{path} exceeds the 4 MiB text scan limit"));
                continue;
            }
            Ok(_) => {}
            Err(error) => {
                missing.push(format!("{path} metadata could not be read: {error}"));
                continue;
            }
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

fn check_repository_files(root: &Path, files: &[PathBuf], failures: &mut Vec<String>) {
    for file_path in files {
        let relative_path = relative_path(root, file_path);

        check_evidence_artifact(&relative_path, failures);

        if is_blocked_sensitive_path(&relative_path) {
            failures.push(format!("{relative_path} is a blocked sensitive path"));
        }

        if let Some(file_name) = file_path.file_name().and_then(|name| name.to_str())
            && is_blocked_filename(file_name)
        {
            failures.push(format!("{relative_path} is a blocked sensitive filename"));
        }

        if is_binary_file(file_path) {
            continue;
        }

        match fs::metadata(file_path) {
            Ok(metadata) if is_oversized_text_file(metadata.len()) => {
                failures.push(format!("{relative_path} exceeds the 4 MiB text scan limit"));
                continue;
            }
            Ok(_) => {}
            Err(error) => {
                failures.push(format!(
                    "{relative_path} metadata could not be read: {error}"
                ));
                continue;
            }
        }

        match fs::read_to_string(file_path) {
            Ok(content) => {
                check_secret_content(&relative_path, &content, failures);
                if is_workflow_file(&relative_path, file_path) {
                    check_workflow_content(&relative_path, &content, failures);
                }
            }
            Err(error) if error.kind() == io::ErrorKind::InvalidData => {}
            Err(error) => {
                failures.push(format!("{relative_path} could not be read: {error}"));
            }
        }
    }
}

fn check_evidence_artifact(relative_path: &str, failures: &mut Vec<String>) {
    if !relative_path.starts_with("issue-evidence/") {
        return;
    }

    let lower_path = relative_path.to_ascii_lowercase();
    if !lower_path.contains("redacted")
        || lower_path.contains("unredacted")
        || lower_path.contains("nonredacted")
    {
        failures.push(format!("{relative_path} must be explicitly redacted"));
    }
}

fn repo_files_under(root: &Path, directory: &Path, failures: &mut Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut directories = vec![directory.to_path_buf()];
    let mut scanned_entries = 0usize;

    while let Some(current_directory) = directories.pop() {
        let entries = match fs::read_dir(&current_directory) {
            Ok(entries) => entries,
            Err(error) => {
                failures.push(format!(
                    "{} could not be read: {error}",
                    relative_path(root, &current_directory)
                ));
                continue;
            }
        };

        for entry in entries {
            scanned_entries += 1;
            if scanned_entries > MAX_SCAN_ENTRIES {
                failures.push(format!(
                    "repository scan exceeds the {MAX_SCAN_ENTRIES} entry limit"
                ));
                directories.clear();
                break;
            }

            let Ok(entry) = entry else {
                failures.push(format!(
                    "{} contains an unreadable directory entry",
                    relative_path(root, &current_directory)
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
                if !should_skip_dir(root, &path) {
                    directories.push(path);
                }
            } else if file_type.is_file() {
                files.push(path);
            } else if file_type.is_symlink() {
                failures.push(format!(
                    "{} is a symlink and will not be followed",
                    relative_path(root, &path)
                ));
            }
        }
    }

    files.sort_unstable();
    files
}

fn should_skip_dir(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .and_then(|relative_path| relative_path.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .is_some_and(|first_part| SKIPPED_DIRS.contains(&first_part))
}

fn is_blocked_sensitive_path(relative_path: &str) -> bool {
    let lower_path = relative_path.to_ascii_lowercase();
    BLOCKED_SENSITIVE_PATHS.contains(&lower_path.as_str())
}

fn is_blocked_filename(file_name: &str) -> bool {
    let lower_name = file_name.to_ascii_lowercase();
    BLOCKED_FILENAMES.contains(&lower_name.as_str())
        || (lower_name.starts_with(".env.") && !is_environment_template(&lower_name))
        || BLOCKED_SECRET_SUFFIXES
            .iter()
            .any(|suffix| lower_name.ends_with(suffix))
}

fn is_environment_template(file_name: &str) -> bool {
    matches!(
        file_name.rsplit('.').next(),
        Some("dist" | "example" | "sample" | "template")
    )
}

fn is_binary_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .is_some_and(|extension| BINARY_SUFFIXES.contains(&extension.as_str()))
}

fn is_oversized_text_file(size: u64) -> bool {
    size > MAX_TEXT_FILE_BYTES
}

fn is_workflow_file(relative_path: &str, path: &Path) -> bool {
    relative_path.starts_with(".github/workflows/")
        && matches!(
            path.extension()
                .and_then(|extension| extension.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref(),
            Some("yml" | "yaml")
        )
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
        MAX_TEXT_FILE_BYTES, is_binary_file, is_blocked_filename, is_blocked_sensitive_path,
        is_oversized_text_file, should_skip_dir,
    };
    use std::path::Path;

    #[test]
    fn distinguishes_sensitive_files_from_templates() {
        assert!(is_blocked_filename(".env.production"));
        assert!(is_blocked_filename("service-account.json"));
        assert!(is_blocked_filename("signing.P12"));
        assert!(is_blocked_filename("vault.kdbx"));
        assert!(!is_blocked_filename(".env.example"));
        assert!(!is_blocked_filename(".env.production.sample"));
        assert!(!is_blocked_filename("public-certificate.pem"));
    }

    #[test]
    fn blocks_root_credential_paths() {
        assert!(is_blocked_sensitive_path(".aws/credentials"));
        assert!(is_blocked_sensitive_path(".KUBE/config"));
        assert!(!is_blocked_sensitive_path("docs/.aws/credentials"));
    }

    #[test]
    fn recognizes_binary_extensions_case_insensitively() {
        assert!(is_binary_file(Path::new("archive.ZIP")));
        assert!(is_binary_file(Path::new("fixture.DOCX")));
        assert!(!is_binary_file(Path::new("README.md")));
    }

    #[test]
    fn bounds_text_file_scans() {
        assert!(!is_oversized_text_file(MAX_TEXT_FILE_BYTES));
        assert!(is_oversized_text_file(MAX_TEXT_FILE_BYTES + 1));
    }

    #[test]
    fn skips_only_reserved_top_level_local_work_areas() {
        let root = Path::new("/repo");
        assert!(should_skip_dir(root, &root.join("worktrees/transmission")));
        assert!(should_skip_dir(root, &root.join("backups/archive")));
        assert!(!is_blocked_sensitive_path("docs/worktrees"));
        assert!(!should_skip_dir(root, &root.join("docs/worktrees")));
    }
}
