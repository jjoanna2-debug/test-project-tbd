# GitHub Workflow

This guide explains the branch-to-merge workflow used by this repository.

## 1. Create a Branch

A branch is a safe place to make changes before merging them into `main`.

```bash
git checkout -b practice/my-first-change
```

## 2. Make a Small Change

Edit a focused set of files, such as:

```text
Cargo.toml
src/main.rs
README.md
```

Keep each change coherent so it is easy to review, test, and reverse.

## 3. Check What Changed

```bash
git status
git diff
```

## 4. Run Local Checks

Run the same project-specific doctor used in CI:

```bash
bash scripts/doctor.sh
```

For the complete Rust gate, also run:

```bash
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

## 5. Commit the Change

```bash
git add src/main.rs
git commit -m "Practice first branch change"
```

Replace `src/main.rs` with the files you actually changed.

## 6. Push the Branch

```bash
git push -u origin practice/my-first-change
```

## 7. Open a Pull Request

Open a pull request from the branch into `main`.

Use the pull request template and confirm that no secrets, credentials, personal data, or confidential information are included.

## 8. Review the Protected Check

The required `Repository smoke test` runs four direct gates:

1. Rust formatting;
2. strict Clippy checks across all targets and features;
3. locked tests across all targets and features;
4. the Rust repository doctor.

The same gate runs for pull requests, pushes to `main`, merge-queue groups, and manual dispatches. When another commit reaches the same pull request, the older in-progress run is canceled so only the current revision consumes CI time.

If the check fails, open the failed step, fix the reported defect, and push the correction to the same branch.

## 9. Merge

Merge only after the required check passes. The merge queue, when used, runs the same check against the proposed merge-group commit rather than trusting an earlier pull-request result.

## Dependency Updates

Dependabot checks GitHub Actions and Cargo every Monday at 06:00 Europe/Lisbon time. Routine minor and patch updates are grouped by ecosystem, while major updates remain separate for explicit review.

## Safety Rules

Do not commit:

- passwords;
- API keys;
- tokens;
- private keys;
- live `.env` files;
- personal data;
- customer data;
- confidential information;
- regulated data;
- production credentials.

This repository is a learning sandbox, not a production project.
