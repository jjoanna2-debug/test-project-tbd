# Local Setup

Verified against the repository on **2026-08-17**.

This guide covers the current local workflow: clone, create a branch, run the same validation used by CI, inspect repository-doctor diagnostics, commit, push, and open a pull request.

## Requirements

You need:

- Git for cloning, branching, committing, pulling, and pushing;
- rustup, which selects the repository's pinned Rust `1.97.1` toolchain;
- a code editor.

The repository toolchain file installs the minimal Rust profile plus Clippy and Rustfmt. No framework, server, database, external service, runtime dependency, or production credential is required.

Confirm the tools are available:

```bash
git --version
rustup --version
rustc --version
cargo --version
cargo clippy --version
cargo fmt --version
```

The Rust commands should resolve through `rust-toolchain.toml` after entering the repository directory.

## Clone the Repository

```bash
git clone https://github.com/jjoanna2-debug/test-project-tbd.git
cd test-project-tbd
```

The first Rust command may download the exact pinned toolchain if rustup does not already have it.

## Create a Working Branch

Do not make routine changes directly on protected `main`.

```bash
git switch main
git pull --ff-only origin main
git switch -c practice/my-change
```

Use a branch name that describes one coherent change.

## Files You Will Usually Edit

```text
Cargo.toml
rust-toolchain.toml
src/main.rs
src/bin/check_repo.rs
src/bin/check_repo/content.rs
src/bin/check_repo/output.rs
src/bin/check_repo/secrets.rs
src/bin/check_repo/secrets/corpus.rs
src/bin/check_repo/workflows.rs
README.md
START_HERE.md
docs/
.github/
```

`scripts/doctor.sh` is only the local wrapper for the Rust doctor. CI invokes the Rust binary directly.

## Run the Complete Local Gate

Run the same six checks used by the protected workflow:

```bash
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo doc --locked --no-deps --document-private-items
cargo run --quiet --locked --bin check_repo
```

The repository treats compiler, Clippy, and rustdoc warnings as failures in CI. Fix the first reported defect, rerun the failed command, and then rerun the full set.

## Repository Doctor Output

The local default is human-readable text:

```bash
cargo run --quiet --locked --bin check_repo -- --format text
```

For deterministic machine-readable output:

```bash
cargo run --quiet --locked --bin check_repo -- --format json
```

For the same annotation format used in GitHub Actions:

```bash
cargo run --quiet --locked --bin check_repo -- --format github
```

Use `--help` for the supported interface. Unsupported or ambiguous arguments fail rather than being ignored.

Inside GitHub Actions, the doctor automatically defaults to GitHub annotations. Locally it defaults to text.

## Run the Doctor Wrapper

```bash
bash scripts/doctor.sh
```

The doctor currently checks:

- required repository, toolchain, source, content, and output invariants;
- one deterministic, sorted repository inventory;
- iterative traversal capped at 20,000 visited entries;
- rejection of symlinks and explicit reporting of unreadable filesystem state;
- a 4 MiB ceiling on every scannable text read, including required-file reference checks;
- up to 64 KiB of validation for files carrying declared binary extensions;
- binary magic prefixes, UTF-8 validity, misleading binary suffixes, and undeclared binary content;
- sensitive filenames, credential stores, key containers, and non-redacted evidence paths;
- private-key blocks, provider token shapes, AWS access-key shapes, and scored generic secret assignments;
- contextual GitHub Actions permissions, triggers, YAML indirection, checkout credentials, full lowercase action SHA pins, and lowercase Docker digests;
- durable CI, classifier, content, diagnostic, and Dependabot configuration invariants.

Passing wrapper output:

```text
Repository check passed.
Doctor check passed.
```

## Make and Review a Change

After editing:

```bash
git status
git diff --check
git diff
```

Stage only the files that belong to the change:

```bash
git add path/to/file
```

Then commit:

```bash
git commit -m "Describe the focused change"
```

## Push and Open a Pull Request

```bash
git push -u origin practice/my-change
```

Open a pull request into `main`, complete the repository template, and wait for `Repository smoke test` to pass. The required check validates formatting, compilation, strict all-target/all-feature Clippy, locked all-target/all-feature tests, documentation compilation, and the repository doctor.

## Update After Merge

```bash
git switch main
git pull --ff-only origin main
git branch -d practice/my-change
```

Delete the remote branch through GitHub or with Git after confirming the pull request merged.

## Safety Rules

Never commit or paste:

- production credentials;
- API keys, tokens, passwords, or private keys;
- live `.env` files or root credential stores;
- personal, customer, confidential, or regulated data;
- unredacted evidence artifacts;
- unrelated generated or backup directories.

This repository is public and intended for learning, testing, and experimentation only.
