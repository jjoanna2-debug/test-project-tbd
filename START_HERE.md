# Start Here

This file is the beginner map for this repository.

If you are new to GitHub, think of a repository as a project folder that remembers every important change.

## What This Repo Is

This is a public staging project for learning the basics:

- what files live in a repository;
- how commits work;
- how branches work;
- how pull requests work;
- how public repository policy files work;
- how a small Rust project is structured;
- how small Rust and shell checks can support a repository.

It is not a production app. That is okay. The point is to learn the shape of a clean repository before building something bigger.

## What Each File Does

| File | What it means |
| --- | --- |
| `README.md` | The front page of the project. Start here when visiting the repo. |
| `START_HERE.md` | A beginner-friendly guide to what is going on. |
| `AGENTS.md` | Codex instructions for this repository. |
| `Cargo.toml` | Rust package manifest, project metadata, toolchain floor, and lint policy. |
| `Cargo.lock` | Rust dependency lockfile for reproducible local and CI checks. |
| `src/main.rs` | Small Rust starter program. |
| `src/bin/check_repo.rs` | Rust check for required files, Rust starter references, common secret patterns, workflow hardening, and redacted evidence artifact names. |
| `scripts/doctor.sh` | Shell wrapper for running the repository check locally. |
| `docs/LOCAL_SETUP.md` | How to clone, open, edit, and commit locally. |
| `docs/GITHUB_WORKFLOW.md` | How to branch, commit, push, open a pull request, and merge. |
| `docs/PROJECT_STRUCTURE.md` | A fuller map of the repository structure. |
| `LICENSE` | The legal permission for using the project. This repo uses Apache-2.0. |
| `NOTICE` | Copyright and project attribution notice. |
| `LEGAL_NOTICES.md` | Plain-language context for the license and public-use boundaries. |
| `DISCLAIMER.md` | Extra warning that this is experimental and used at your own risk. |
| `SECURITY.md` | Says this project is not security-reviewed or formally supported. |
| `SUPPORT.md` | Explains that there is no support or maintenance promise. |
| `CONTRIBUTING.md` | Rules for opening issues, pull requests, or contributions. |
| `CODE_OF_CONDUCT.md` | Participation and moderation expectations. |
| `SPONSORS.md` | Funding status and no-benefits clarification. |
| `CHANGELOG.md` | Notes meaningful repository changes over time. |
| `ROADMAP.md` | Shows possible next learning steps. |
| `.github/FUNDING.yml` | Placeholder funding configuration for future sponsor links. |
| `.github/dependabot.yml` | Weekly Dependabot checks for GitHub Actions and Cargo. |
| `.github/PULL_REQUEST_TEMPLATE.md` | Checklist shown when opening pull requests. |
| `.github/ISSUE_TEMPLATE/` | Issue forms for bugs, features, and documentation tasks. |
| `.github/workflows/basic-checks.yml` | Basic automated checks for the repository. |
| `.github/CODEOWNERS` | Default review visibility for repository changes. |
| `.agents/skills/repo-check/SKILL.md` | Repo-local Codex skill for inspect, edit, verify, and report work. |
| `.codex/hooks.json` | Project-local Codex hook configuration. |
| `.codex/hooks/run_repo_doctor.sh` | Hook wrapper that runs the repository doctor. |
| `.gitignore` | Tells Git which local junk files should stay out of the repo. |
| `.gitattributes` | Keeps text and binary file handling consistent. |
| `.editorconfig` | Keeps editor formatting consistent. |
| `.markdownlint.json` | Keeps Markdown linting useful without fighting long policy text. |

## Your First GitHub Words

- **Repository:** the project folder on GitHub.
- **Commit:** a saved snapshot of a change.
- **Branch:** a separate line of work.
- **Pull request:** a request to merge one branch into another.
- **Clone:** download the repo to your computer.
- **Push:** send your local commits to GitHub.
- **Pull:** bring GitHub changes down to your computer.
- **Workflow:** an automated GitHub Actions process.
- **Dependabot:** GitHub's automated dependency update helper.

## Tiny Practice Plan

1. Open `src/main.rs` in an editor.
2. Edit the status message or one sentence in `README.md`.
3. Run `git status` to see what changed.
4. Run `git add src/main.rs` or `git add README.md`.
5. Run `git commit -m "Practice Rust starter edit"`.
6. Run `git push`.
7. Refresh GitHub and look for the new commit.

## What To Try Next

- Create a new branch.
- Edit `src/main.rs`.
- Open a pull request.
- Check the pull request template.
- Check the GitHub Actions result.
- Merge the pull request.
- Look at the commit history.

Small steps are the whole game. You do not need to understand everything before touching the repo.
