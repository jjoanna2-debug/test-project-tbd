# Rust Staging Lab

Documentation snapshot: **2026-08-16**.

A compact Rust-first staging area for learning GitHub, repository automation, defensive tooling, and future CLI ideas. The repository is deliberately small, but its checks are real: protected CI, deterministic repository inspection, bounded filesystem work, contextual GitHub Actions policy checks, and a calibrated secret-signal classifier.

## Current Snapshot

| Item | Current value |
| --- | --- |
| Package | `test-project-tbd` `0.1.0` |
| Rust edition | 2024 |
| Pinned toolchain | Rust `1.97.1`, minimal profile, Clippy and Rustfmt |
| Runtime dependencies | None |
| Default branch | Protected `main` |
| Required check | `Repository smoke test` |
| Doctor text-read limit | 4 MiB per non-binary file |
| Doctor traversal limit | 20,000 visited directory entries |
| Intended use | Learning, testing, and experimentation only |

Public repository: <https://github.com/jjoanna2-debug/test-project-tbd>

```bash
git clone https://github.com/jjoanna2-debug/test-project-tbd.git
cd test-project-tbd
```

## Purpose

This repository is used to practice and improve:

- focused commits, branches, pull requests, protected checks, and merge queues;
- a small Rust 2024 project with an exact reproducible toolchain and strict lint policy;
- dependency-free repository inspection and policy enforcement;
- secret-pattern classification with measurable regression tests;
- safe GitHub Actions and Dependabot configuration;
- documentation, licensing, contribution, and public-repository hygiene.

It is not a production system, commercial product, managed service, professional recommendation, security product, or operational dependency.

## Repository Layout

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
src/main.rs
src/bin/check_repo.rs
src/bin/check_repo/secrets.rs
src/bin/check_repo/secrets/corpus.rs
src/bin/check_repo/workflows.rs
scripts/doctor.sh
docs/
.github/
issue-evidence/
```

The implementation remains intentionally dependency-free. New abstractions have to earn their existence rather than arriving in a motorcade of configuration files.

## Reproducible Rust Baseline

`rust-toolchain.toml` pins Rust `1.97.1` with the minimal rustup profile plus Clippy and Rustfmt. Cargo uses Edition 2024 and the Rust-version-aware resolver implied by that edition. Local commands and GitHub Actions therefore evaluate the same compiler, formatter, linter, and standard library instead of whichever toolchain happens to be installed that week.

The manifest also denies debug macros, unfinished `todo!` paths, and `unimplemented!` placeholders through the repository lint policy.

## Repository Doctor

`src/bin/check_repo.rs` builds one deterministic repository inventory and coordinates focused classifiers. It exits successfully only when the repository satisfies its current structural, security, workflow, and automation invariants.

### Inventory and resource boundaries

The doctor:

- verifies required project, policy, documentation, GitHub, Rust, toolchain, and helper files;
- walks the repository iteratively rather than recursively;
- stops after 20,000 visited directory entries;
- rejects repository symlinks rather than following them outside the scan boundary;
- sorts and deduplicates findings for stable output;
- samples up to 64 KiB from declared binary files and verifies that their content is actually binary;
- reports UTF-8 text hidden behind a binary suffix and then scans the full text;
- rejects undeclared binary data and invalid UTF-8 instead of silently omitting it;
- recognizes common executable, archive, object, image, document, audio, database, and WebAssembly magic prefixes;
- refuses to load non-binary files larger than 4 MiB;
- applies the same 4 MiB ceiling to required-file reference checks;
- reports unreadable directories, entries, metadata, and files instead of silently omitting them.

### Sensitive-file and secret checks

The doctor rejects common credential stores, live environment files, private-key filenames, key containers, and password-manager databases used by cloud CLIs, container tooling, Kubernetes, Terraform, Vault, package managers, and service-account workflows. It detects private-key blocks, AWS access-key shapes, and provider-specific token shapes for GitHub, GitLab, OpenAI, Slack, Stripe, npm, Google, and SendGrid.

Generic assignments are parsed from quoted, unquoted, multiline, escaped, YAML literal-block, and YAML folded-block forms. Colon assignments are accepted only when the left side is a valid configuration key, preventing Rust type annotations, Markdown code, and quoted test literals from being misread as credentials.

Secret values are scored using:

- exact secret-bearing key semantics;
- value length;
- character-class diversity;
- distinct-character diversity;
- provider-token plausibility;
- placeholder, environment-reference, example-host, repeated-pattern, sequential-pattern, prose, redaction, and low-entropy suppression.

Parsing is bounded to 4,096 bytes and 32 lines per candidate value. Only the strongest generic assignment finding per file is emitted, which avoids turning one bad fixture into a choir of identical alarms.

### Calibrated regression corpus

`src/bin/check_repo/secrets/corpus.rs` maintains a dedicated labeled corpus with realistic positives and hard negatives. It covers provider-token probes, multiline values, YAML blocks, escaped strings, comments, placeholders, documentation examples, misleading key names, repeated patterns, sequential patterns, public examples, and environment references.

Provider-shaped probes are assembled at runtime so the repository does not store credential-like literals merely to test that it can detect them.

The protected tests require the maintained corpus to meet all of these floors at the active threshold:

| Metric | Required floor |
| --- | --- |
| Average precision | `0.98` |
| Precision | `0.95` |
| Recall | `0.95` |
| F1 | `0.95` |

These figures are regression guarantees for this repository's maintained internal corpus. They are not claims about an external production dataset.

### GitHub Actions policy

Workflow checks are context-aware rather than blind substring searches. The doctor enforces:

- mandatory top-level permissions;
- read-only or empty token permission maps, with every `write` scope rejected at workflow and job level;
- an event allowlist limited to `push`, `pull_request`, `merge_group`, `workflow_dispatch`, and `schedule`;
- rejection of privileged pull-request context, issue-comment, workflow-completion, repository-dispatch, and other unapproved trigger surfaces;
- rejection of YAML anchors, aliases, and merge keys so authorization cannot be hidden through indirection;
- `persist-credentials: false` on every `actions/checkout` step;
- rejection of `allow-unsafe-pr-checkout: true`;
- full lowercase commit-SHA pins for third-party GitHub Actions;
- lowercase SHA-256 digests for Docker actions;
- checkout-step tracking that remains correct when inputs contain nested YAML lists.

A future workflow needing a different trigger or token permission must update the policy, tests, risk analysis, and current documentation explicitly. The repository does not inherit authority merely because a YAML key exists for it.

## Run Locally

The pinned toolchain is selected automatically when rustup is installed. Run the complete gate:

```bash
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo doc --locked --no-deps --document-private-items
cargo run --quiet --locked --bin check_repo
```

Or run the local wrapper for the repository doctor:

```bash
bash scripts/doctor.sh
```

Passing doctor output:

```text
Repository check passed.
Doctor check passed.
```

## Automated Quality Gate

The required `Repository smoke test` runs formatting, compilation, strict Clippy, locked tests, documentation compilation, and the repository doctor on:

- pull requests into `main`;
- pushes to `main`;
- merge-queue groups;
- manual workflow dispatches.

The workflow disables incremental compilation, treats compiler and rustdoc warnings as failures, forces current JavaScript Action runtime behavior, and uses the exact toolchain declared by the repository.

New commits cancel older in-progress runs for the same pull request or ref. CI therefore validates the current revision rather than financing an archaeological survey of superseded commits.

The doctor also asserts the important toolchain, workflow, and Dependabot settings. Removing merge-queue coverage, stale-run cancellation, all-target checks, documentation compilation, direct doctor execution, read-only workflow boundaries, immutable references, or grouped dependency-update policy causes the repository gate to fail.

## Dependency Automation

Dependabot checks GitHub Actions and Cargo every Monday at **06:00 Europe/Lisbon**.

Routine minor and patch updates are grouped by ecosystem. Major updates remain separate for explicit review. Open version-update pull requests are capped per ecosystem.

## Evidence Asset Releases

The repository currently has two GitHub Release tags:

- `codex-issue-22773-assets`, published 2026-05-15 and explicitly described by its release metadata as a redacted screenshot bundle;
- `codex-issue-23192-assets`, published 2026-05-17 as a public screenshot-evidence bundle.

They are **not software releases**, package versions, supported distributions, production builds, or compatibility promises. The tags point to evidence snapshots and must not be interpreted as semantic-version releases of `test-project-tbd`.

Evidence stored in the repository tree remains subject to the explicit-redaction naming checks under `issue-evidence/`. GitHub Release assets are outside the checked-out file tree. The local doctor does not inspect or attest to their redaction, so every release asset must be reviewed separately before publication.

## Working With the Repository

Start with [START_HERE.md](START_HERE.md) for the beginner map. The current operational guides are:

- [docs/LOCAL_SETUP.md](docs/LOCAL_SETUP.md)
- [docs/GITHUB_WORKFLOW.md](docs/GITHUB_WORKFLOW.md)
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)
- [ROADMAP.md](ROADMAP.md)
- [CHANGELOG.md](CHANGELOG.md)

Use a branch and pull request for changes to `main`. The current workflow is documented in [docs/GITHUB_WORKFLOW.md](docs/GITHUB_WORKFLOW.md).

## Optional Future Direction

The completed baseline does not require another framework or another folder of ceremony. Future work should begin only when a concrete consumer exists. Useful candidates include:

- structured findings for CI annotations or machine consumption;
- benchmarks for large-but-valid repositories within the existing resource limits;
- reusable library boundaries if a second binary or caller appears;
- corpus maintenance when provider formats or realistic hard negatives change;
- dependencies only where measured value exceeds maintenance and supply-chain cost.

See [ROADMAP.md](ROADMAP.md) for the completed phase status and optional direction.

## Important Boundaries

This repository is provided for learning, testing, and experimentation only. Do not use it with production credentials, secrets, private keys, API tokens, personal data, confidential information, customer data, regulated data, business-critical workflows, or security-sensitive systems.

The repository doctor and protected workflow are guardrails, not a security audit, scanner warranty, support promise, or production-readiness certification.

## Policies and Notices

- [LICENSE](LICENSE) — Apache License 2.0 terms
- [NOTICE](NOTICE) — copyright and project attribution
- [LEGAL_NOTICES.md](LEGAL_NOTICES.md) — plain-language license and public-use boundaries
- [DISCLAIMER.md](DISCLAIMER.md) — warranty, liability, reliance, and risk boundaries
- [SECURITY.md](SECURITY.md) — security policy and supported-status statement
- [SUPPORT.md](SUPPORT.md) — support and maintenance boundaries
- [CONTRIBUTING.md](CONTRIBUTING.md) — contribution workflow and restrictions
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — participation and moderation expectations
- [SPONSORS.md](SPONSORS.md) — current funding status and sponsorship boundaries

## Funding Status

No active GitHub funding provider is configured as of 2026-08-16. The placeholder `.github/FUNDING.yml` does not expose a Sponsor button or create sponsorship tiers, paid support, consulting, maintenance, priority, or service obligations.

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for the controlling license text and [LEGAL_NOTICES.md](LEGAL_NOTICES.md) for plain-language context.

Copyright and project attribution are listed in [NOTICE](NOTICE).
