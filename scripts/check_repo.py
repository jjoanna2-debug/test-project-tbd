#!/usr/bin/env python3
"""Small repository structure check for the learning lab."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

REQUIRED_FILES = [
    "README.md",
    "START_HERE.md",
    "index.html",
    "src/styles.css",
    "src/main.js",
    "docs/PROJECT_STRUCTURE.md",
    "docs/LOCAL_SETUP.md",
    "docs/GITHUB_WORKFLOW.md",
    "LICENSE",
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
    ".github/FUNDING.yml",
    ".github/dependabot.yml",
    ".github/CODEOWNERS",
    ".github/PULL_REQUEST_TEMPLATE.md",
    ".github/ISSUE_TEMPLATE/config.yml",
    ".github/ISSUE_TEMPLATE/bug_report.yml",
    ".github/ISSUE_TEMPLATE/feature_request.yml",
    ".github/ISSUE_TEMPLATE/documentation_task.yml",
]

STARTER_REFERENCES = {
    "index.html": ["src/styles.css", "src/main.js", "practice-button"],
    "src/main.js": ["practice-button"],
}


def main() -> int:
    missing = [path for path in REQUIRED_FILES if not (ROOT / path).is_file()]

    for path, expected_values in STARTER_REFERENCES.items():
        content = (ROOT / path).read_text(encoding="utf-8")
        for expected in expected_values:
            if expected not in content:
                missing.append(f"{path} reference: {expected}")

    if missing:
        print("Repository check failed:")
        for item in missing:
            print(f"- {item}")
        return 1

    print("Repository check passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
