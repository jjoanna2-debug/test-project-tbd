# Codex Instructions

## Repository Scope

This repository is a Rust-first staging lab for learning GitHub, tooling,
automation, repository hygiene, and future app or CLI ideas.

Keep changes small, readable, and beginner-friendly. Do not turn this repository
into a production service, security product, or credential-bearing workspace.

## Working Loop

Before editing, inspect the relevant live files and prefer the existing Rust,
shell, documentation, and GitHub workflow patterns.

For implementation work:

1. Check the current state with `git status --short`.
2. Read the files you are about to edit.
3. Keep the change scoped to the requested behavior or documentation.
4. Run the verification commands below before calling the work done.
5. If any verification fails, fix the failure and rerun the relevant check.

## Required Verification

Run these commands after repository code, docs, workflow, policy, Codex
instruction, skill, hook, or script changes:

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
bash scripts/doctor.sh
```

Do not report the work as ready while any required verification is failing.

## Safety Rules

Do not commit or introduce production credentials, secrets, private keys, API
tokens, personal data, customer data, confidential information, regulated data,
or unredacted issue evidence.

Evidence files under `issue-evidence/` must be explicitly redacted in the file
or directory name.

GitHub Actions must keep least-privilege permissions and third-party actions
must be pinned to full commit SHAs.

Rust code must keep `#![forbid(unsafe_code)]`.

## Codex Workflow Surfaces

Use this file for durable repository rules.

Use `.agents/skills/repo-check/SKILL.md` when a task asks for the reusable
inspect, change, verify, and summarize workflow for this repository.

Project-local hooks live under `.codex/`. They are mechanical enforcement only;
do not put long workflow doctrine there.
