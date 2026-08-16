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
| `docs/GITHUB_WORKFLOW.md` | Branch, commit, protected-check, merge-queue, and dependency-update workflow. |

## Source Files

| Path | Purpose |
| --- | --- |
| `src/main.rs` | Small Rust starter program. |
| `src/bin/check_repo.rs` | Repository inventory, bounded traversal and text reads, required-file policy, sensitive-path checks, and classifier orchestration. |
| `src/bin/check_repo/secrets.rs` | Provider signatures and scored secret-assignment classification with precision/recall regression tests. |
| `src/bin/check_repo/workflows.rs` | Context-aware GitHub Actions permission and immutable-reference checks. |

## Scripts

| Path | Purpose |
| --- | --- |
| `scripts/doctor.sh` | Local shell wrapper for the Rust repository doctor. CI invokes the Rust binary directly. |

## GitHub Configuration

| Path | Purpose |
| --- | --- |
| `.github/FUNDING.yml` | Placeholder funding configuration. |
| `.github/dependabot.yml` | Scheduled and grouped GitHub Actions and Cargo version updates. |
| `.github/CODEOWNERS` | Default review visibility for repository changes. |
| `.github/PULL_REQUEST_TEMPLATE.md` | Pull request checklist. |
| `.github/ISSUE_TEMPLATE/config.yml` | Issue template configuration. |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | Bug report form. |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | Feature request form. |
| `.github/ISSUE_TEMPLATE/documentation_task.yml` | Documentation task form. |
| `.github/workflows/basic-checks.yml` | Protected Rust quality and repository-policy gate with stale-run cancellation, merge-queue support, and manual dispatch. |

## Current Design Principle

Keep the implementation small and readable while making every new safety or automation rule self-enforcing. This repository is for learning GitHub workflows, not for production use.
