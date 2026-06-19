# Local Setup

This guide explains how to open and edit this repository on your computer.

## Requirements

You need:

- Git, if you want to clone and push changes;
- Rust and Cargo, if you want to run the Rust starter locally;
- a code editor, such as VS Code, Cursor, Zed, or another editor.

No framework, server, external service, or production dependency is required.

## Clone the Repository

```bash
git clone https://github.com/jjoanna2-debug/test-project-tbd.git
cd test-project-tbd
```

## Open the Starter Project

You can also open the full folder in your code editor and edit:

```text
Cargo.toml
src/main.rs
src/bin/check_repo.rs
scripts/doctor.sh
```

## Make a Small Practice Change

For example:

1. Change the status message in `src/main.rs`.
2. Save the file.
3. Run `cargo test --locked` if Rust is installed.
4. Check the change.

## Run the Doctor Check

```bash
bash scripts/doctor.sh
```

The doctor check verifies the expected repository files and fails if anything
under `issue-evidence/` is not explicitly marked as redacted. It also rejects
common secret patterns, private-key blocks, sensitive filenames, broad workflow
write permissions, and GitHub Actions that are not pinned to full commit SHAs.

## Commit the Change

```bash
git status
git add src/main.rs
git commit -m "Practice Rust starter edit"
git push
```

## Safety Reminder

Do not add secrets, credentials, API tokens, private keys, personal data, customer data, confidential information, or production files to this repository.

This project is for learning, testing, and experimentation only.
