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

## Start here

If you are new to GitHub, read [START_HERE.md](START_HERE.md) first. It explains the files in this repository, the basic GitHub words, and a tiny practice plan.

For hands-on setup and workflow notes, read:

- [docs/LOCAL_SETUP.md](docs/LOCAL_SETUP.md)
- [docs/GITHUB_WORKFLOW.md](docs/GITHUB_WORKFLOW.md)
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)

## Starter project

This repository now uses a small Rust starter instead of a static web page:

```text
AGENTS.md
Cargo.toml
Cargo.lock
src/main.rs
src/bin/check_repo.rs
scripts/doctor.sh
.agents/skills/repo-check/SKILL.md
.codex/hooks.json
.codex/hooks/run_repo_doctor.sh
```

Edit the Rust or shell files to practice commits, branches, pull requests, and checks. The Rust code is intentionally small for now; it is a staging point, not a finished product.

## What The Doctor Checks

`src/bin/check_repo.rs` is the main project-specific tool in this repository. It helps keep the public repository small, readable, and suitable for practice by checking a narrow set of repository hygiene rules:

- required project, policy, documentation, GitHub, Rust, and script files exist;
- expected Rust safety references stay in place;
- issue evidence artifacts are explicitly marked as redacted;
- common sensitive filenames are not committed;
- private-key blocks, GitHub-token shapes, AWS access-key shapes, and generic secret assignments are flagged;
- GitHub Actions workflows avoid broad write permissions;
- third-party GitHub Actions are pinned to full commit SHAs.

The doctor is a local hygiene guard, not a complete security scanner or audit.

Run it locally with:

```bash
bash scripts/doctor.sh
```

Passing output:

```text
Repository check passed.
Doctor check passed.
```

## Codex Workflow

This repository includes Codex-specific workflow surfaces:

- `AGENTS.md` defines durable repository instructions and required checks.
- `.agents/skills/repo-check/SKILL.md` defines the reusable inspect, edit,
  verify, and report workflow for this repo.
- `.codex/hooks.json` wires a project-local stop hook to the repository doctor.

The hook is mechanical enforcement around the existing doctor check. Rich
workflow guidance belongs in `AGENTS.md` and the repo skill.

## Current scope

- Repository setup
- Beginner onboarding
- README structure
- Rust starter project
- Rust-native repository doctor and shell wrapper
- Codex repository instructions, repo-check skill, and project-local doctor hook
- License and disclaimer hygiene
- GitHub workflow practice
- Basic public-repository policy files
- Issue and pull request templates
- Basic GitHub Actions smoke checks
- Weekly Dependabot checks for GitHub Actions and Cargo
- Basic local and CI guards for common secret patterns, sensitive filenames, redacted evidence artifacts, workflow permissions, and pinned GitHub Actions
- Funding status note

## Important notices

This repository is provided for learning, testing, and experimentation only. Use of this repository or its contents is voluntary and entirely at your own risk.

This repository is not production software, not professional advice, not a managed service, not a security product, not audited, not supported, and not guaranteed to be accurate, complete, secure, maintained, or suitable for any purpose.

Do not use this repository with production credentials, secrets, private keys, API tokens, personal data, confidential information, customer data, regulated data, business-critical workflows, or security-sensitive systems.

## Policies and project files

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

## GitHub workflow helpers

This repository includes:

- issue templates for bugs, features, and documentation tasks;
- a pull request template;
- a basic GitHub Actions workflow;
- a small Rust test;
- Rust and shell local checks;
- weekly Dependabot checks for GitHub Actions and Cargo;
- public-repository hardening checks for secrets, redacted evidence artifacts, and GitHub workflow safety;
- CODEOWNERS review visibility;
- `.editorconfig` and `.gitattributes` for cleaner editing and diffs.

## Funding status

This repository does not currently expose active GitHub funding links. If funding is enabled later, any support will remain voluntary and will not create support, maintenance, consulting, service-level, feature, priority, warranty, or commercial obligations. See [SPONSORS.md](SPONSORS.md).

A placeholder `.github/FUNDING.yml` exists for future funding metadata only.

## Next steps

- Edit the Rust starter
- Create a branch
- Make commits
- Open a pull request
- Review the GitHub Actions result
- Merge the pull request
- Read the changelog and roadmap

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for the full license text and [LEGAL_NOTICES.md](LEGAL_NOTICES.md) for plain-language context.

Copyright and project attribution are listed in [NOTICE](NOTICE).

## Disclaimer

This repository is provided on an "as is" and "as available" basis for learning, testing, and experimentation purposes only. See [DISCLAIMER.md](DISCLAIMER.md) for additional clarification.
