# Changelog

Documentation reconciled through **2026-08-16**.

All notable changes to this repository are recorded here. The project follows a simple chronological changelog rather than a formal release-management process.

## [Unreleased]

### Added

- Starter public repository structure.
- Beginner onboarding documentation.
- Rust starter project.
- Rust and shell checks for repository structure.
- Apache-2.0 `NOTICE` file for copyright attribution.
- Project structure documentation in `docs/PROJECT_STRUCTURE.md`.
- Local setup guide in `docs/LOCAL_SETUP.md`.
- GitHub workflow guide in `docs/GITHUB_WORKFLOW.md`.
- License, legal notices, disclaimer, security, support, contribution, conduct, and sponsorship policy files.
- Structured issue forms and a pull request template.
- Protected GitHub Actions repository quality gate.
- Scheduled Dependabot checks for GitHub Actions and Cargo.
- `CODEOWNERS`, `.editorconfig`, and `.gitattributes` for repository hygiene.
- Roadmap for current and future learning phases.
- Repository doctor checks for secret signals, sensitive paths, workflow policy, immutable action references, and explicitly redacted evidence artifacts.
- Rust-native repository doctor binary so the project remains Rust-first.
- Exact `rust-toolchain.toml` selection for Rust `1.97.1`, Clippy, and Rustfmt.
- Dedicated test-only classifier corpus with realistic positives, hard negatives, provider probes, multiline assignments, YAML blocks, escaped values, documentation examples, placeholders, and misleading key names.
- Calibration gates for average precision, precision, recall, and F1 at the active classifier threshold.
- Workflow-policy regression tests for read-only permissions, trigger allowlisting, YAML indirection, checkout safety, nested inputs, and immutable references.

### Changed

- Improved repository documentation and public-use boundaries.
- Replaced the earlier static starter with a Rust-first staging lab.
- Migrated the package from Edition 2021 and Rust `1.85` to Edition 2024 and exact Rust `1.97.1`.
- Added repository and README package metadata while keeping the package non-publishable.
- Strengthened the lint policy by denying debug macros, `todo!`, and `unimplemented!` paths.
- Expanded the protected gate to validate compilation and documentation in addition to formatting, strict Clippy, tests, and repository policy.
- Made compiler and rustdoc warnings fatal in CI, disabled incremental compilation, and selected current JavaScript Action runtime behavior explicitly.
- Clarified that funding metadata is inactive until a valid provider is configured.
- Kept root-local backup, build, cache, dependency, temporary, and delegated-work areas outside the public scan boundary.
- Expanded sensitive-filename checks to environment variants, credential manifests, private-key files, key containers, password-manager databases, and common root credential stores.
- Collapsed artifact, secret, and workflow checks into one deterministic file inventory so the doctor does not repeatedly traverse or reread the repository.
- Replaced recursive filesystem walking with iterative traversal capped at 20,000 visited entries.
- Rejected repository symlinks instead of silently omitting them from the security boundary.
- Normalized binary-extension handling across letter case.
- Refused non-binary text files over 4 MiB before loading them into memory.
- Applied the same 4 MiB ceiling to required-file reference checks and surfaced metadata failures explicitly.
- Split secret and workflow classifiers into focused Rust modules.
- Replaced broad secret-key substring matching with exact key semantics, bounded multiline assignment parsing, provider-token plausibility checks, placeholder suppression, and per-file deduplication.
- Added quoted, unquoted, escaped, multiline, YAML literal-block, and YAML folded-block parsing with explicit 4,096-byte and 32-line candidate limits.
- Expanded provider coverage to current GitHub, GitLab, OpenAI, Slack, Stripe, npm, Google, SendGrid, AWS access-key, and private-key signals.
- Rejected repeated, sequential, low-diversity, placeholder, example-host, environment-reference, redacted, and prose fixtures before they can inflate classifier confidence.
- Restricted colon assignment parsing to valid configuration keys so Rust type annotations, Markdown code, and unbalanced quoted literals do not produce false findings.
- Raised the maintained calibration floors to `0.98` average precision and `0.95` precision, recall, and F1.
- Made workflow permission checks context-aware and required immutable Docker action digests.
- Replaced the `contents: write` special case with rejection of every workflow or job permission scope set to `write`.
- Added an explicit workflow-trigger allowlist for `push`, `pull_request`, `merge_group`, `workflow_dispatch`, and `schedule`.
- Rejected privileged pull-request context, issue-comment, workflow-completion, repository-dispatch, and other unapproved trigger surfaces.
- Rejected YAML anchors, aliases, and merge keys so workflow authorization cannot be hidden through indirection.
- Required checkout credentials to remain disabled and rejected unsafe pull-request checkout mode.
- Required full lowercase commit SHAs for third-party Actions and lowercase SHA-256 digests for Docker actions.
- Corrected checkout-step tracking so nested input lists do not terminate checkout policy validation.
- Expanded the protected Rust gate to all targets and features, direct doctor execution, merge-queue groups, pushes to `main`, and manual dispatches.
- Added concurrency that cancels superseded runs for the same pull request or ref.
- Grouped routine Dependabot minor and patch updates by ecosystem on a fixed Europe/Lisbon schedule while leaving major updates isolated for review.
- Made the repository doctor assert durable toolchain, CI, classifier, workflow, and Dependabot guarantees so later edits cannot silently remove the automation baseline.
- Reconciled the roadmap with completed repository, workflow, automation, doctor, and classifier-calibration phases.
- Distinguished the `codex-issue-22773-assets` and `codex-issue-23192-assets` evidence bundles from software releases, package versions, supported builds, and compatibility commitments.
- Reconciled security, support, contribution, conduct, funding, legal, disclaimer, and attribution documents with the implemented repository and evidence surfaces.
- Replaced the lightweight pull request checklist with scope, verification, impact, documentation, rollback, and follow-up sections.
- Reworked issue forms to collect affected revisions, reproducible steps, acceptance criteria, measurements, verified dates, evidence handling, and current sources of truth.
- Added current contribution, support, and security links to the issue chooser.
- Dated the inactive funding configuration and tied future provider activation to a documentation update.

### Documentation

- Reviewed every maintained current-state document against the repository as implemented on 2026-08-16.
- Corrected beginner instructions to use a branch and pull request rather than pushing routine work directly to protected `main`.
- Replaced stale “basic check” descriptions with the current modular doctor, protected gate, merge-queue, concurrency, resource-bound, and dependency-automation behavior.
- Added explicit review dates to current technical and policy documentation.
- Replaced vague “temporary” and “for now” status wording with dated, testable statements.
- Documented that GitHub Release assets are outside the checked-out tree and therefore require separate redaction review.
- Updated the structure map and README to include current evidence tags and their May 2026 publication dates.
- Tightened release-asset wording so redaction is claimed only where release metadata verifies it; the local doctor does not attest assets outside the checked-out tree.
- Updated the README, beginner map, local setup, workflow guide, and structure map for the exact Rust 2024 toolchain and six-step quality gate.
- Replaced the classifier roadmap's “in progress” status with the implemented calibration contract and optional future maintenance direction.
- Documented the workflow event allowlist, universal write-permission rejection, indirection ban, checkout safety rules, and lowercase immutable-reference contract.

### Notes

- This repository remains experimental and is provided for learning, testing, and GitHub workflow practice.
- Earlier closed issues may mention the previous static-page starter. The current repository is a Rust-first staging lab with a modular Rust-native repository doctor.
- Classifier metrics are regression guarantees for the maintained internal corpus, not claims about external production performance.
- The two GitHub Release tags are evidence-asset bundles, not software releases.
