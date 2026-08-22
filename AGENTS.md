# Repository Instructions

This file governs `jjoanna2-debug/test-project-tbd`. It supplements the global
Codex instructions and the parent GitHub workspace `AGENTS.md`.

## Repository Contract

- This is a small Rust-first public staging lab, not a production application,
  managed service, security product, or professional-advice repository.
- Preserve the Apache-2.0 license and keep `README.md`, `SECURITY.md`,
  `SUPPORT.md`, `CONTRIBUTING.md`, and `DISCLAIMER.md` consistent when their
  shared public contract changes.
- The Rust package uses edition 2024 and the exact Rust 1.97.1 toolchain pinned
  in `rust-toolchain.toml`. It has a committed `Cargo.lock`, no runtime
  dependencies, `unsafe_code = "forbid"`, and strict Rust and Clippy lints. Do
  not weaken those controls without an explicit request.
- `src/bin/check_repo.rs` and `scripts/doctor.sh` define the repository-specific
  safety contract. Keep their behavior, documentation, tests, and CI usage
  aligned.
- Preserve ignored local `backups/` and `worktrees/`, local audit material, and
  unrelated evidence. Exclude them from broad repository scans and never add
  them to public history merely to satisfy a check.

## Required Verification

For Rust, script, workflow, or repository-doctor changes, run the same checks as
the required `Repository smoke test`:

```bash
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo doc --locked --no-deps --document-private-items
cargo run --quiet --locked --bin check_repo
```

For Markdown-only changes, run the configured Markdown lint on the changed
files and run the repository doctor. If a documentation edit changes a public
policy or a doctor-enforced claim, run the full smoke-test sequence above.

## Editing and GitHub Boundaries

- Inspect `git status` before editing and preserve unrelated or ignored local
  work.
- Patch narrowly. Do not add dependencies, generated frameworks, or broad test
  infrastructure for a small documentation or policy change.
- Read the live pull request, checks, branch protection, and public file after
  any GitHub action. A local command or push alone is not proof of the public
  result.
- Do not edit the workspace ledgers or public profile merely because this
  checkout changed; reconcile them only when the verified public contribution
  state materially changes.
