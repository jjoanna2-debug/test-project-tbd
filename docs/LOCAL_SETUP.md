# Local Setup

Verified against the repository on **2026-08-16**.

This guide covers the current local workflow: clone, create a branch, run the same validation used by CI, commit, push, and open a pull request.

## Requirements

You need:

- Git for cloning, branching, committing, pulling, and pushing;
- Rust `1.85` or newer with Cargo and Rustfmt;
- Clippy for the repository's strict lint gate;
- a code editor.

No framework, server, database, external service, runtime dependency, or production credential is required.

Confirm the tools are available:

```bash
git --version
rustc --version
cargo --version
cargo clippy --version
cargo fmt --version
```

## Clone the Repository

```bash
git clone https://github.com/jjoanna2-debug/test-project-tbd.git
cd test-project-tbd
```

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
src/main.rs
src/bin/check_repo.rs
src/bin/check_repo/secrets.rs
src/bin/check_repo/workflows.rs
README.md
START_HERE.md
docs/
.github/
```

`scripts/doctor.sh` is only the local wrapper for the Rust doctor. CI invokes the Rust binary directly.

## Run the Complete Local Gate

Run the same four checks used by the protected workflow:

```bash
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo run --quiet --locked --bin check_repo
```

The commands stop at the first failure when run individually from a shell script or CI step. Fix the first reported defect, rerun the failed command, and then rerun the full set.

## Run the Doctor Wrapper

```bash
bash scripts/doctor.sh
```

The doctor currently checks:

- required repository and source invariants;
- one deterministic, sorted repository inventory;
- iterative traversal capped at 20,000 visited entries;
- rejection of symlinks and explicit reporting of unreadable filesystem state;
- a 4 MiB ceiling on every non-binary text read, including required-file reference checks;
- sensitive filenames, credential stores, key containers, and non-redacted evidence paths;
- private-key blocks, provider token shapes, AWS access-key shapes, and scored generic secret assignments;
- contextual GitHub Actions permissions, triggers, checkout credentials, full action SHA pins, and Docker digests;
- durable CI and Dependabot configuration invariants.

Passing output:

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

Open a pull request into `main`, complete the repository template, and wait for `Repository smoke test` to pass. The required check validates formatting, strict all-target/all-feature Clippy, locked all-target/all-feature tests, and the repository doctor.

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
