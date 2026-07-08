---
name: repo-check
description: Use for changes in test-project-tbd that need the repository's inspect, edit, verification, and summary loop.
---

# Repository Check Workflow

Use this skill for changes to the `test-project-tbd` repository, including Rust
source, shell scripts, documentation, GitHub workflows, policy files, Codex
instructions, local skills, and project hooks.

## Inspect

1. Run `git status --short`.
2. Read `AGENTS.md` and the files directly affected by the task.
3. Prefer existing repository patterns over new abstractions.
4. Keep the change small enough to review in one pull request.

## Edit

1. Preserve the repository's Rust-first staging-lab purpose.
2. Keep public-facing wording beginner-friendly and explicit about safety
   boundaries.
3. Do not add production dependencies or external services unless the user asks
   for them directly.
4. Do not introduce secrets, credentials, private keys, API tokens, personal
   data, customer data, confidential information, regulated data, or unredacted
   issue evidence.
5. Keep GitHub Actions on least-privilege permissions and pin third-party
   actions to full commit SHAs.

## Verify

Run the full local verification set before reporting completion:

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
bash scripts/doctor.sh
```

If the task only asks for a read-only review, run commands only when they are
needed to verify a finding. If any check fails after an edit, repair the failure
and rerun the relevant command.

## Report

Finish with:

- the files changed;
- the verification commands run and their result;
- any residual risk or skipped check.

Do not call the work ready while verification is failing.
