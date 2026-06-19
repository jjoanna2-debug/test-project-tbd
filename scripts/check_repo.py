#!/usr/bin/env python3
# Copyright 2026 Jean-Claude Joanna
# SPDX-License-Identifier: Apache-2.0
"""Small repository structure check for the staging lab."""

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

REQUIRED_FILES = [
    "README.md",
    "START_HERE.md",
    "Cargo.toml",
    "Cargo.lock",
    "src/main.rs",
    "scripts/check_repo.py",
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
]

SOURCE_REFERENCES = {
    "Cargo.toml": [
        'edition = "2021"',
        'license = "Apache-2.0"',
        'unsafe_code = "forbid"',
    ],
    "Cargo.lock": ['name = "test-project-tbd"'],
    "src/main.rs": ["#![forbid(unsafe_code)]", "GitHub staging lab ready."],
}

EVIDENCE_ROOT = ROOT / "issue-evidence"
WORKFLOW_ROOT = ROOT / ".github" / "workflows"

SKIPPED_DIRS = {
    ".git",
    ".venv",
    "__pycache__",
    "build",
    "coverage",
    "dist",
    "node_modules",
    "out",
    "target",
    "tmp",
    "temp",
}

BINARY_SUFFIXES = {
    ".gif",
    ".ico",
    ".jpeg",
    ".jpg",
    ".pdf",
    ".png",
    ".webp",
}

BLOCKED_FILENAMES = {
    ".env",
    ".env.local",
    ".npmrc",
    ".pypirc",
    ".netrc",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    "id_rsa",
}

SECRET_PATTERNS = [
    ("private key block", re.compile(r"-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----")),
    ("GitHub token", re.compile(r"\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9_]{30,}\b")),
    ("GitHub fine-grained token", re.compile(r"\bgithub_pat_[A-Za-z0-9_]{20,}\b")),
    ("AWS access key id", re.compile(r"\b(?:AKIA|ASIA)[A-Z0-9]{16}\b")),
    (
        "generic secret assignment",
        re.compile(
            r"(?i)\b(?:api[_-]?key|secret|token|password)\b"
            r"\s*[:=]\s*[\"'][A-Za-z0-9_./+=:-]{20,}[\"']"
        ),
    ),
]

PINNED_ACTION_REF = re.compile(r"^[a-f0-9]{40}$")
USES_LINE = re.compile(r"^\s*-?\s*uses:\s*([^#\s]+)")


def iter_repo_files():
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if any(part in SKIPPED_DIRS for part in path.relative_to(ROOT).parts):
            continue
        yield path


def text_or_none(path: Path) -> str | None:
    if path.suffix.lower() in BINARY_SUFFIXES:
        return None
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return None


def check_sensitive_files() -> list[str]:
    failures = []
    for path in iter_repo_files():
        relative_path = path.relative_to(ROOT).as_posix()
        if path.name in BLOCKED_FILENAMES:
            failures.append(f"{relative_path} is a blocked sensitive filename")

        content = text_or_none(path)
        if content is None:
            continue

        for label, pattern in SECRET_PATTERNS:
            if pattern.search(content):
                failures.append(f"{relative_path} appears to contain {label}")

    return failures


def check_workflows() -> list[str]:
    failures = []
    if not WORKFLOW_ROOT.is_dir():
        return failures

    for workflow_path in sorted(WORKFLOW_ROOT.glob("*.yml")) + sorted(
        WORKFLOW_ROOT.glob("*.yaml")
    ):
        relative_path = workflow_path.relative_to(ROOT).as_posix()
        content = workflow_path.read_text(encoding="utf-8")

        if "write-all" in content:
            failures.append(f"{relative_path} must not use write-all permissions")

        if re.search(r"(?m)^\s*contents:\s*write\s*$", content):
            failures.append(f"{relative_path} must not grant contents: write")

        for line_number, line in enumerate(content.splitlines(), start=1):
            match = USES_LINE.match(line)
            if not match:
                continue

            action_ref = match.group(1)
            if action_ref.startswith(("./", "docker://")):
                continue
            if "@" not in action_ref:
                failures.append(f"{relative_path}:{line_number} action is unpinned")
                continue

            _, ref = action_ref.rsplit("@", 1)
            if not PINNED_ACTION_REF.fullmatch(ref):
                failures.append(
                    f"{relative_path}:{line_number} action must be pinned to a full SHA"
                )

    return failures


def main() -> int:
    missing = [path for path in REQUIRED_FILES if not (ROOT / path).is_file()]
    failures = []

    for path, expected_values in SOURCE_REFERENCES.items():
        source_path = ROOT / path
        if not source_path.is_file():
            continue

        content = source_path.read_text(encoding="utf-8")
        for expected in expected_values:
            if expected not in content:
                missing.append(f"{path} reference: {expected}")

    if EVIDENCE_ROOT.is_dir():
        for evidence_path in EVIDENCE_ROOT.rglob("*"):
            if not evidence_path.is_file():
                continue
            relative_path = evidence_path.relative_to(ROOT).as_posix()
            lower_path = relative_path.lower()
            if (
                "redacted" not in lower_path
                or "unredacted" in lower_path
                or "nonredacted" in lower_path
            ):
                failures.append(f"{relative_path} must be explicitly redacted")

    failures.extend(check_sensitive_files())
    failures.extend(check_workflows())

    if missing or failures:
        print("Repository check failed:")
        for item in missing:
            print(f"- {item}")
        for item in failures:
            print(f"- {item}")
        return 1

    print("Repository check passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
