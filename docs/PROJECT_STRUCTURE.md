# Project Structure

This document explains the current repository layout.

## Root Files

| Path | Purpose |
| --- | --- |
| `README.md` | Main project overview and entry point. |
| `START_HERE.md` | Beginner guide for understanding the repository. |
| `Cargo.toml` | Rust package manifest, project metadata, toolchain floor, and lint policy. |
| `Cargo.lock` | Rust dependency lockfile. |
| `LICENSE` | Apache License 2.0 text. |
| `NOTICE` | Copyright and project attribution notice. |
| `LEGAL_NOTICES.md` | Plain-language legal and public-use context. |
| `DISCLAIMER.md` | Warranty, liability, reliance, and use-at-your-own-risk notice. |
| `SECURITY.md` | Security expectations and unsupported status. |
| `SUPPORT.md` | Support and maintenance boundaries. |
| `CONTRIBUTING.md` | Contribution rules and sensitive-information restrictions. |
| `CODE_OF_CONDUCT.md` | Participation and moderation expectations. |
| `SPONSORS.md` | Temporary sponsorship notice. |
| `CHANGELOG.md` | Chronological change notes. |
| `ROADMAP.md` | Learning roadmap and future ideas. |
| `.gitignore` | Files Git should ignore. |
| `.gitattributes` | Git text and binary handling rules. |
| `.editorconfig` | Editor formatting defaults. |
| `.markdownlint.json` | Markdown linting defaults for repository documentation. |

## Documentation

| Path | Purpose |
| --- | --- |
| `docs/PROJECT_STRUCTURE.md` | Current repository layout. |
| `docs/LOCAL_SETUP.md` | How to clone, open, edit, and commit locally. |
| `docs/GITHUB_WORKFLOW.md` | Branch, commit, push, pull request, and merge workflow. |

## Source Files

| Path | Purpose |
| --- | --- |
| `src/main.rs` | Small Rust starter program. |

## Scripts

| Path | Purpose |
| --- | --- |
| `scripts/check_repo.py` | Python check for required files, Rust starter references, redacted evidence artifact names, common secret patterns, sensitive filenames, and workflow hardening. |
| `scripts/doctor.sh` | Shell wrapper for the local repository check. |

## GitHub Configuration

| Path | Purpose |
| --- | --- |
| `.github/FUNDING.yml` | Placeholder funding configuration. |
| `.github/dependabot.yml` | Weekly Dependabot checks for GitHub Actions and Cargo. |
| `.github/CODEOWNERS` | Default review visibility for repository changes. |
| `.github/PULL_REQUEST_TEMPLATE.md` | Pull request checklist. |
| `.github/ISSUE_TEMPLATE/config.yml` | Issue template configuration. |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | Bug report form. |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | Feature request form. |
| `.github/ISSUE_TEMPLATE/documentation_task.yml` | Documentation task form. |
| `.github/workflows/basic-checks.yml` | Simple repository smoke test. |

## Current Design Principle

Keep everything small, readable, and beginner-friendly. This repository is for learning GitHub workflows, not for production use.
