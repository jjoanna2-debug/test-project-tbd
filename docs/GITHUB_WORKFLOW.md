# GitHub Workflow

Verified against the repository on **2026-08-16**.

This guide defines the current branch-to-merge workflow. The default branch is protected, the required check is named `Repository smoke test`, and the same gate also validates merge-queue groups and pushes to `main`.

## 1. Synchronize `main`

```bash
git switch main
git pull --ff-only origin main
```

`--ff-only` prevents an accidental local merge commit while updating the protected branch.

## 2. Create a Focused Branch

```bash
git switch -c feature/describe-the-change
```

Use one branch for one coherent change. Good branch names describe the intent rather than the emotional journey required to implement it.

## 3. Make the Change

Edit only the files needed for the task. Review the working tree before staging:

```bash
git status
git diff --check
git diff
```

## 4. Run the Current Local Gate

The repository selects Rust `1.97.1`, Clippy, and Rustfmt through `rust-toolchain.toml`.

```bash
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo doc --locked --no-deps --document-private-items
cargo run --quiet --locked --bin check_repo
```

The local wrapper remains available when only the doctor needs to be rerun:

```bash
bash scripts/doctor.sh
```

## 5. Commit a Reviewable Unit

```bash
git add path/to/changed-file
git commit -m "Describe the focused change"
```

Do not stage unrelated files merely because Git has noticed them. Computers are excellent witnesses and terrible editors.

## 6. Push the Branch

```bash
git push -u origin feature/describe-the-change
```

Later commits on the same branch can use plain `git push`.

## 7. Open the Pull Request

Open a pull request from the branch into `main` and complete the repository template. The description should state:

- what changed;
- why the change is needed;
- how it was verified;
- security, data, compatibility, and operational impact;
- documentation changes;
- any deliberate exclusions or follow-up work.

## 8. Protected Quality Gate

The required `Repository smoke test` contains six direct validation steps:

| Step | Command |
| --- | --- |
| Formatting | `cargo fmt --check` |
| Compilation | `cargo check --locked --all-targets --all-features` |
| Lint | `cargo clippy --locked --all-targets --all-features -- -D warnings` |
| Tests | `cargo test --locked --all-targets --all-features` |
| Documentation | `cargo doc --locked --no-deps --document-private-items` |
| Repository policy | `cargo run --quiet --locked --bin check_repo` |

The workflow currently runs on:

- pull requests targeting `main`;
- pushes to `main`;
- merge-queue `checks_requested` groups;
- manual `workflow_dispatch` runs.

The job has read-only repository contents permission, uses a full commit-SHA pin for `actions/checkout`, disables persisted checkout credentials, disables incremental compilation, and treats compiler and rustdoc warnings as failures. The repository toolchain file keeps local and hosted validation on the same exact Rust release.

## 9. Superseded Runs

The workflow concurrency key is based on the workflow plus the pull request number or Git ref. A newer commit cancels an older in-progress run for the same pull request or ref.

A canceled obsolete run is not a quality failure. It means a newer revision has replaced it and will receive the authoritative result.

## 10. Review Before Merge

Before merging:

1. Read the final diff, not merely the commit messages.
2. Confirm the required check passed on the current head commit.
3. Resolve review comments and stale conversations.
4. Confirm documentation matches the implemented behavior.
5. Confirm no secrets, credentials, personal data, confidential information, or unredacted evidence entered the branch.

## 11. Merge Queue

When the merge queue is used, GitHub creates a proposed merge-group commit and runs the same `Repository smoke test` against that combined state. This catches failures caused by interaction with newer `main` changes instead of trusting an earlier pull-request result.

## 12. After Merge

```bash
git switch main
git pull --ff-only origin main
git branch -d feature/describe-the-change
```

Delete the remote branch after confirming the merge completed.

## Dependency Updates

Dependabot checks both GitHub Actions and Cargo every Monday at **06:00 Europe/Lisbon**.

- Minor and patch updates are grouped by ecosystem.
- Major updates remain separate for explicit review.
- Open version-update pull requests are capped per ecosystem.
- Dependabot changes still pass through the protected repository gate.

## Toolchain Updates

The Rust toolchain is intentionally exact rather than floating. A toolchain update must change `rust-toolchain.toml`, the matching `rust-version` in `Cargo.toml`, repository-doctor invariants, and current documentation in one pull request. The protected gate then proves that formatting, compilation, linting, tests, documentation, and repository policy all survive the update.

## Documentation Freshness

Update documentation in the same pull request whenever behavior, commands, file layout, policy, supported tooling, automation, or roadmap status changes. Add or update the review date on current-state documents.

A green build beside obsolete instructions is merely a well-tested lie.

## Safety Rules

Do not commit:

- passwords, API keys, tokens, private keys, or production credentials;
- live `.env` files or root credential stores;
- personal, customer, confidential, or regulated data;
- unredacted evidence artifacts;
- unrelated backup, build, or delegated-work directories.

This repository remains a public learning and testing project, not a production service.
