# Project Structure

Verified against `main` on **2026-08-16**.

This document maps the current repository layout and the responsibility of each maintained path.

## Root Files

| Path | Current responsibility |
| --- | --- |
| `README.md` | Current project snapshot, architecture, checks, automation, priorities, evidence-release status, and boundaries. |
| `START_HERE.md` | Beginner map and safe first branch-to-PR exercise. |
| `Cargo.toml` | Package metadata, version `0.1.0`, Edition 2024, Rust `1.97.1`, repository metadata, publish setting, and lint policy. |
| `Cargo.lock` | Locked dependency state for reproducible local and CI commands. |
| `rust-toolchain.toml` | Exact Rust `1.97.1` toolchain with the minimal profile, Clippy, and Rustfmt. |
| `LICENSE` | Standard Apache License 2.0 text. |
| `NOTICE` | Current copyright and repository attribution. |
| `LEGAL_NOTICES.md` | Plain-language license and public-use context. |
| `DISCLAIMER.md` | Warranty, liability, reliance, and use-at-your-own-risk boundaries. |
| `SECURITY.md` | Supported-status statement, current guardrails, and security-reporting boundaries. |
| `SUPPORT.md` | Support and maintenance status. |
| `CONTRIBUTING.md` | Current branch, validation, contribution, and sensitive-information requirements. |
| `CODE_OF_CONDUCT.md` | Participation, scope, moderation, and enforcement expectations. |
| `SPONSORS.md` | Current funding status and sponsorship boundaries. |
| `CHANGELOG.md` | Chronological record of meaningful repository changes and documentation reviews. |
| `ROADMAP.md` | Completed phases and optional future direction. |
| `.gitignore` | Root-local build, backup, worktree, dependency, and environment exclusions. |
| `.gitattributes` | Text normalization and binary handling rules. |
| `.editorconfig` | Editor formatting defaults. |
| `.markdownlint.json` | Markdown lint settings used for repository documentation. |

## Source Files

| Path | Current responsibility |
| --- | --- |
| `src/main.rs` | Minimal Rust starter binary and its unit test. |
| `src/bin/check_repo.rs` | Main repository-doctor binary: required-file policy, source invariants, iterative bounded inventory, text-read limits, sensitive paths, redacted evidence, and classifier orchestration. |
| `src/bin/check_repo/secrets.rs` | Private-key, provider-token, AWS-key, bounded assignment parsing, calibrated generic secret scoring, and metric enforcement. |
| `src/bin/check_repo/secrets/corpus.rs` | Maintained labeled calibration corpus with realistic positives, hard negatives, provider probes, multiline values, YAML blocks, escaped strings, placeholders, and misleading key names. |
| `src/bin/check_repo/workflows.rs` | Context-aware GitHub Actions permissions, trigger, checkout-credential, SHA-pin, and Docker-digest enforcement. |

The package has no runtime dependencies. `unsafe_code` is forbidden. Clippy `all` and `pedantic` are enabled, while debug macros and unfinished `todo!` or `unimplemented!` paths are denied. CI promotes warnings to failures.

The classifier corpus is test-only. Provider-shaped probes are constructed at runtime so source control does not contain credential-like literals merely to exercise detection.

## Classifier Calibration Contract

The maintained corpus covers:

- quoted, unquoted, multiline, escaped, YAML literal-block, and YAML folded-block assignments;
- GitHub, GitLab, OpenAI, Slack, Stripe, npm, Google, SendGrid, AWS access-key, and private-key signals;
- placeholders, environment references, repeated and sequential patterns, documentation examples, prose, misleading key names, comments, and example hosts;
- valid configuration-key grammar that excludes Rust type annotations, Markdown code, and unbalanced quoted literals.

The test harness requires at least `0.98` average precision and `0.95` precision, recall, and F1 at the active threshold. These are internal regression floors, not external production-performance claims.

## Documentation

| Path | Current responsibility |
| --- | --- |
| `docs/LOCAL_SETUP.md` | Tool requirements, clone, branch, complete local gate, push, PR, and post-merge cleanup. |
| `docs/GITHUB_WORKFLOW.md` | Protected branch-to-merge contract, exact toolchain, current CI steps, events, concurrency, merge queue, and dependency automation. |
| `docs/PROJECT_STRUCTURE.md` | This current layout map. |

## Local Helper

| Path | Current responsibility |
| --- | --- |
| `scripts/doctor.sh` | Strict shell wrapper that changes to the repository root, runs the locked Rust doctor binary, and prints a passing confirmation. |

CI runs the Rust doctor directly rather than placing a shell wrapper between GitHub Actions and the actual check.

## GitHub Configuration

| Path | Current responsibility |
| --- | --- |
| `.github/workflows/basic-checks.yml` | Required `Repository smoke test` for pull requests, pushes to `main`, merge groups, and manual dispatch; validates formatting, compilation, linting, tests, documentation, and repository policy with stale-run cancellation and read-only checkout. |
| `.github/dependabot.yml` | Monday 06:00 Europe/Lisbon GitHub Actions and Cargo checks, grouped minor/patch updates, separate major updates, and PR limits. |
| `.github/CODEOWNERS` | Default review visibility. |
| `.github/PULL_REQUEST_TEMPLATE.md` | Pull request scope, verification, risk, documentation, and safety checklist. |
| `.github/ISSUE_TEMPLATE/config.yml` | Disables blank issues and links to contribution, support, and security policies. |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | Structured defect report form. |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | Structured improvement proposal form. |
| `.github/ISSUE_TEMPLATE/documentation_task.yml` | Structured documentation correction form. |
| `.github/FUNDING.yml` | Inactive funding-provider configuration; no Sponsor button is currently exposed. |

## Evidence Artifacts

| Location | Current responsibility |
| --- | --- |
| `issue-evidence/` | Public issue-supporting artifacts committed to the repository tree and required to use explicit redaction naming. |
| `issue-evidence/codex-23192-redacted/` | Current in-tree redacted evidence set for Codex issue 23192. |
| Release tag `codex-issue-22773-assets` | Screenshot-evidence bundle published 2026-05-15 and explicitly described by its release metadata as redacted. |
| Release tag `codex-issue-23192-assets` | Public screenshot-evidence bundle published 2026-05-17 for Codex issue 23192. |

The two GitHub Releases are evidence bundles, not software releases or package versions. Their tags must not be interpreted as supported builds or semantic versions of `test-project-tbd`.

The doctor rejects in-tree evidence paths that are not explicitly redacted or that use names such as `unredacted` or `nonredacted`. GitHub Release assets are outside the checked-out repository tree. The local doctor neither inspects nor attests to their redaction, so every release asset requires separate review before publication.

## Generated and Local-Only Areas

The repository doctor and `.gitignore` deliberately exclude root-local build, cache, backup, temporary, dependency, and delegated-work directories such as `target/`, `node_modules/`, `backups/`, and `worktrees/`.

A nested documentation directory with the same name is not automatically excluded. Skip rules apply only to reserved top-level local-work areas.

## Design Contract

The current design is:

- Rust 2024 with an exact reproducible toolchain;
- dependency-free at runtime;
- deterministic;
- resource-bounded;
- calibrated against a maintained labeled corpus;
- fail-closed on unreadable or ambiguous repository state;
- protected by CI;
- self-enforcing for critical toolchain, workflow, classifier, and Dependabot invariants;
- explicit about the difference between internal regression metrics and external production claims;
- explicit about the difference between evidence-asset tags and software releases.

Update this file whenever a maintained path, toolchain, calibration contract, release-asset bundle, or material responsibility is added, removed, renamed, or changed.
