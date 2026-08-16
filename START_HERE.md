# Start Here

Documentation snapshot: **2026-08-16**.

This is the beginner map for the repository. It explains what currently exists, how changes reach the protected `main` branch, and which files matter first.

## What This Repository Is

This is a public Rust-first staging project for learning:

- how repositories, commits, branches, pull requests, and protected checks work;
- how a small Rust package is structured;
- how GitHub Actions and Dependabot automate repository maintenance;
- how policy and contribution files shape a public repository;
- how a dependency-free Rust doctor can enforce repository-specific rules;
- how classifier quality can be protected by measurable regression tests.

It is not a production app or supported security product. The point is to learn with a small repository whose safeguards are implemented rather than merely admired in Markdown.

## Current Snapshot

| Item | Current value |
| --- | --- |
| Rust package | `test-project-tbd` `0.1.0` |
| Rust toolchain | Pinned `1.97.1`, Edition 2024 |
| Runtime dependencies | None |
| Default branch | Protected `main` |
| Required check | `Repository smoke test` |
| Dependency updates | Mondays at 06:00 Europe/Lisbon |

## What the Main Files Do

| Path | Purpose |
| --- | --- |
| `README.md` | Current project overview, checks, automation, and boundaries. |
| `START_HERE.md` | This beginner-friendly map. |
| `Cargo.toml` | Package metadata, Rust 2024 edition, exact toolchain floor, and lint policy. |
| `Cargo.lock` | Reproducible dependency lockfile. |
| `rust-toolchain.toml` | Exact Rust, Clippy, and Rustfmt selection for local work and CI. |
| `src/main.rs` | Minimal Rust starter program. |
| `src/bin/check_repo.rs` | Repository inventory, bounded traversal and reads, required-file checks, and classifier orchestration. |
| `src/bin/check_repo/secrets.rs` | Provider token signatures and scored secret-assignment classification. |
| `src/bin/check_repo/workflows.rs` | Context-aware GitHub Actions permission, trigger, checkout, and immutable-reference checks. |
| `scripts/doctor.sh` | Local wrapper for the Rust repository doctor. |
| `docs/LOCAL_SETUP.md` | Current clone, branch, check, commit, and push instructions. |
| `docs/GITHUB_WORKFLOW.md` | Current protected branch-to-merge workflow. |
| `docs/PROJECT_STRUCTURE.md` | Complete repository layout and file responsibilities. |
| `CHANGELOG.md` | Meaningful changes recorded over time. |
| `ROADMAP.md` | Completed phases, current priorities, and optional future work. |
| `SECURITY.md` | Security expectations, implemented guardrails, and unsupported status. |
| `SUPPORT.md` | No-support and no-maintenance boundaries. |
| `CONTRIBUTING.md` | Contribution requirements and current validation commands. |
| `.github/workflows/basic-checks.yml` | Protected formatting, compilation, lint, test, documentation, and doctor gate. |
| `.github/dependabot.yml` | Scheduled and grouped GitHub Actions and Cargo updates. |
| `.github/ISSUE_TEMPLATE/` | Structured bug, feature, and documentation forms. |
| `.github/PULL_REQUEST_TEMPLATE.md` | Pull request scope, verification, risk, and documentation checklist. |

The remaining legal, conduct, funding, Git, and editor files are mapped in [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md).

## GitHub Words You Need First

- **Repository:** the project folder and its history.
- **Commit:** a saved snapshot with a message.
- **Branch:** a separate line of work created from another commit.
- **Pull request:** a proposed merge from one branch into another.
- **Clone:** copy the repository to your computer.
- **Push:** send local commits to GitHub.
- **Pull:** bring remote commits into your local branch.
- **Workflow:** an automated GitHub Actions process.
- **Required check:** an automated result that must pass before merge.
- **Merge queue:** GitHub validation of the proposed combined state before merging.
- **Dependabot:** GitHub's dependency-update automation.

## First Safe Practice Change

Do not work directly on `main`. It is protected, which is GitHub's way of preventing a five-second experiment from becoming permanent folklore.

```bash
git clone https://github.com/jjoanna2-debug/test-project-tbd.git
cd test-project-tbd
git switch -c practice/first-change
```

Edit `src/main.rs` or one relevant documentation file, then run:

```bash
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo doc --locked --no-deps --document-private-items
bash scripts/doctor.sh
```

Commit and push the branch:

```bash
git status
git add src/main.rs
git commit -m "Practice first branch change"
git push -u origin practice/first-change
```

Then:

1. Open a pull request into `main`.
2. Complete the pull request template.
3. Wait for `Repository smoke test` to pass.
4. Review the diff and check output.
5. Merge only after the protected gate succeeds.
6. Pull the updated `main` branch locally.

## Where to Go Next

- Use [docs/LOCAL_SETUP.md](docs/LOCAL_SETUP.md) for complete local commands.
- Use [docs/GITHUB_WORKFLOW.md](docs/GITHUB_WORKFLOW.md) for the current branch-to-merge contract.
- Read [README.md](README.md) for the doctor, classifier metric, CI events, and dependency automation.
- Read [ROADMAP.md](ROADMAP.md) before inventing another “helpful” file the repository does not need.
