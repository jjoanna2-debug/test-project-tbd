# Rust Staging Lab

A small Rust-first staging area for learning GitHub, tooling, automation, repository hygiene, and future app or CLI ideas.

## Repository

Public repository: <https://github.com/jjoanna2-debug/test-project-tbd>

Clone URL:

```bash
git clone https://github.com/jjoanna2-debug/test-project-tbd.git
```

## Purpose

This repository is used for experimenting with GitHub workflows and basic project setup.

It is not a production system, commercial product, managed service, professional recommendation, security tool, or operational dependency.

## Start Here

If you are new to GitHub, read [START_HERE.md](START_HERE.md) first. It explains the files in this repository, the basic GitHub words, and a tiny practice plan.

For hands-on setup and workflow notes, read:

- [docs/LOCAL_SETUP.md](docs/LOCAL_SETUP.md)
- [docs/GITHUB_WORKFLOW.md](docs/GITHUB_WORKFLOW.md)
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)

## Starter Project

This repository uses a small Rust starter and a modular Rust-native repository doctor:

```text
Cargo.toml
Cargo.lock
src/main.rs
src/bin/check_repo.rs
src/bin/check_repo/secrets.rs
src/bin/check_repo/workflows.rs
scripts/doctor.sh
```

Edit the Rust, documentation, or configuration files to practice commits, branches, pull requests, and checks. The code is intentionally compact; it is a staging point, not a finished product.

## What the Doctor Checks

`src/bin/check_repo.rs` builds one deterministic repository inventory and coordinates the project-specific checks. The focused classifier modules keep detection logic testable instead of turning one file into a municipal landfill.

The doctor verifies that:

- required project, policy, documentation, GitHub, Rust, and local-helper files exist;
- expected Rust safety and CI-policy references remain in place;
- issue evidence artifacts are explicitly marked as redacted;
- repository symlinks cannot silently escape the scan boundary;
- traversal is iterative and stops after 20,000 visited entries instead of allowing unbounded filesystem work or recursion depth;
- non-binary text reads, including required-file reference checks, are capped at 4 MiB per file before content is loaded into memory;
- sensitive filenames, key containers, password-manager databases, and common root credential stores are not committed;
- private-key blocks, AWS access-key shapes, and provider-specific token shapes are flagged;
- quoted and unquoted secret assignments are scored using key semantics, length, character classes, and value diversity;
- placeholders, environment references, redacted values, and low-entropy examples are suppressed to reduce false positives;
- GitHub Actions permissions are evaluated in YAML context rather than by blind substring matching;
- third-party GitHub Actions use full commit SHAs and Docker actions use SHA-256 digests.

The secret-assignment classifier includes a labeled positive-and-negative regression corpus with an internal average-precision floor of `0.95`. That protects classifier behavior from silent regression; it is not a claim about performance on an external production dataset.

Run the doctor locally with:

```bash
bash scripts/doctor.sh
```

Passing output:

```text
Repository check passed.
Doctor check passed.
```

## Automated Quality Gate

The required `Repository smoke test` runs formatting, strict Clippy checks, locked tests, and the repository doctor directly against every Rust target and feature.

The same gate covers:

- pull requests into `main`;
- pushes to `main`;
- merge-queue groups;
- manual workflow dispatches.

New commits cancel older in-progress runs for the same pull request or ref, so CI validates the current revision instead of burning runner time on archaeological layers. The doctor also asserts the important workflow and Dependabot settings, making those automation guarantees cumulative.

Dependabot checks GitHub Actions and Cargo every Monday at 06:00 Europe/Lisbon time. Routine minor and patch updates are grouped by ecosystem; major updates remain separate for explicit review.

## Current Scope

- Repository setup
- Beginner onboarding
- README structure
- Rust starter project
- Modular Rust-native repository doctor and local shell wrapper
- Scored secret-signal classification and internal precision/recall regression coverage
- Bounded text reads and iterative repository traversal
- License and disclaimer hygiene
- Protected branch, pull-request, merge-queue, and manual quality gates
- Basic public-repository policy files
- Issue and pull request templates
- Grouped weekly Dependabot checks for GitHub Actions and Cargo
- Local and CI guards for sensitive paths, secret patterns, redacted evidence artifacts, workflow permissions, and immutable action references
- Funding status note

## Important Notices

This repository is provided for learning, testing, and experimentation only. Use of this repository or its contents is voluntary and entirely at your own risk.

This repository is not production software, not professional advice, not a managed service, not a security product, not audited, not supported, and not guaranteed to be accurate, complete, secure, maintained, or suitable for any purpose.

Do not use this repository with production credentials, secrets, private keys, API tokens, personal data, confidential information, customer data, regulated data, business-critical workflows, or security-sensitive systems.

## Policies and Project Files

- [LICENSE](LICENSE) — Apache License 2.0 terms
- [NOTICE](NOTICE) — copyright and project attribution notice
- [LEGAL_NOTICES.md](LEGAL_NOTICES.md) — plain-language license and public-use boundaries
- [DISCLAIMER.md](DISCLAIMER.md) — warranty, liability, professional-advice, and risk disclaimer
- [SECURITY.md](SECURITY.md) — security policy and no-support expectations
- [SUPPORT.md](SUPPORT.md) — support and maintenance boundaries
- [CONTRIBUTING.md](CONTRIBUTING.md) — contribution rules and sensitive-information restrictions
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — participation and moderation expectations
- [SPONSORS.md](SPONSORS.md) — funding status and no-benefits clarification
- [CHANGELOG.md](CHANGELOG.md) — chronological repository change notes
- [ROADMAP.md](ROADMAP.md) — future learning path and project ideas
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — current file layout

## GitHub Workflow Helpers

This repository includes:

- issue templates for bugs, features, and documentation tasks;
- a pull request template;
- a protected GitHub Actions quality gate;
- merge-queue and manual-run coverage;
- stale-run cancellation;
- Rust formatting, linting, tests, and project-specific checks;
- grouped weekly Dependabot checks for GitHub Actions and Cargo;
- public-repository hardening checks for secrets, redacted evidence artifacts, and GitHub workflow safety;
- `CODEOWNERS` review visibility;
- `.editorconfig` and `.gitattributes` for cleaner editing and diffs.

## Funding Status

This repository does not currently expose active GitHub funding links. If funding is enabled later, any support will remain voluntary and will not create support, maintenance, consulting, service-level, feature, priority, warranty, or commercial obligations. See [SPONSORS.md](SPONSORS.md).

A placeholder `.github/FUNDING.yml` exists for future funding metadata only.

## Next Steps

- Edit the Rust starter
- Create a branch
- Make focused commits
- Open a pull request
- Review the protected GitHub Actions result
- Merge the pull request
- Read the changelog and roadmap

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for the full license text and [LEGAL_NOTICES.md](LEGAL_NOTICES.md) for plain-language context.

Copyright and project attribution are listed in [NOTICE](NOTICE).

## Disclaimer

This repository is provided on an "as is" and "as available" basis for learning, testing, and experimentation purposes only. See [DISCLAIMER.md](DISCLAIMER.md) for additional clarification.
